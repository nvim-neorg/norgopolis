//! This file manages the handling of modules (subprocesses)

use anyhow::Result;
use std::sync::Arc;
use std::{process::Stdio, task::Poll};
use tokio::io::{AsyncBufReadExt, AsyncWrite, AsyncWriteExt};
use tokio::{
    io::{AsyncRead, BufReader, BufWriter},
    process::{ChildStdin, ChildStdout, Command},
};
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

#[derive(Debug)]
struct StdioService {
    writer: BufWriter<ChildStdin>,
    reader: BufReader<ChildStdout>,
}

impl StdioService {
    fn new(stdin: ChildStdin, stdout: ChildStdout) -> StdioService {
        StdioService {
            writer: BufWriter::new(stdin),
            reader: BufReader::new(stdout),
        }
    }
}

impl AsyncRead for StdioService {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        async {
            self.reader
                .read_until(0, &mut buf.filled_mut().into())
                .await
                .unwrap();
        };

        Poll::Ready(Ok(()))
    }
}

impl AsyncWrite for StdioService {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<std::result::Result<usize, std::io::Error>> {
        let written;

        async {
            written = self.writer.write(buf).await;
        };

        Poll::Ready(written)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::result::Result<(), std::io::Error>> {
        let result;
        async { result = self.writer.flush().await };

        Poll::Ready(result)
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::result::Result<(), std::io::Error>> {
        let result;

        async {
            result = self.writer.shutdown().await;
        };

        drop(self.writer);

        Poll::Ready(result)
    }
}

pub async fn new_subprocess(name: &String, args: &Vec<String>) -> Result<tonic::transport::Channel> {
    let mut command = Command::new(name)
        .args(args)
        .stdin(Stdio::piped())
        .spawn()?;

    let reader = Arc::new(StdioService::new(command.stdin.take().unwrap(), command.stdout.take().unwrap()));

    let channel = Endpoint::try_from("http://example.com")?
        .connect_with_connector(service_fn(move |_: Uri| {
            let reader_clone = Arc::clone(&reader);
            async move { Ok::<StdioService, anyhow::Error>(Arc::try_unwrap(reader_clone).unwrap().into()) }
        }))
        .await?;

    Ok(channel)
}
