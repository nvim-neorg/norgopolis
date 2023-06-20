use std::pin::Pin;

use tokio::io::{Stdin, Stdout, AsyncWrite, AsyncRead, ReadBuf};
use tonic::transport::server::Connected;

#[derive(Debug)]
pub struct StdioService {
    pub stdin: Stdin,
    pub stdout: Stdout,
}

impl AsyncWrite for StdioService {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        AsyncWrite::poll_write(Pin::new(&mut self.stdout), cx, buf)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        AsyncWrite::poll_flush(Pin::new(&mut self.stdout), cx)
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        AsyncWrite::poll_shutdown(Pin::new(&mut self.stdout), cx)
    }
}

impl AsyncRead for StdioService {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        AsyncRead::poll_read(Pin::new(&mut self.stdin), cx, buf)
    }
}

#[derive(Clone)]
pub struct ConnectionInfo {}

impl Connected for StdioService {
    type ConnectInfo = ConnectionInfo;

    fn connect_info(&self) -> Self::ConnectInfo {
        ConnectionInfo {}
    }
}
