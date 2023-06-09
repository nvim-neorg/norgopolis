//! This file manages the handling of modules (subprocesses)

use anyhow::Result;
use std::path::PathBuf;
use std::pin::Pin;
use std::process::Stdio;
use std::task::{Context, Poll};
use tokio::io::AsyncWrite;
use tokio::process::Child;
use tokio::{
    io::{AsyncRead, ReadBuf},
    process::{ChildStdin, ChildStdout, Command},
};
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

/// Stdio Service handle that allows AsyncRead|Writes to both
/// the stdin and stdout handles.
///
/// * `stdin`: The child stdin handle
/// * `stdout`: The child stdout handle
struct StdioService {
    stdin: ChildStdin,
    stdout: ChildStdout,
}

impl StdioService {
    fn new(command: Child) -> Self {
        let (stdin, stdout) = (command.stdin.unwrap(), command.stdout.unwrap());

        StdioService { stdin, stdout }
    }
}

/// Propagates the AsyncWrite trait of `stdin`
impl AsyncWrite for StdioService {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        AsyncWrite::poll_write(Pin::new(&mut self.stdin), cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        AsyncWrite::poll_flush(Pin::new(&mut self.stdin), cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        AsyncWrite::poll_shutdown(Pin::new(&mut self.stdin), cx)
    }
}

/// Propagates the AsyncRead trait of `stdout`
impl AsyncRead for StdioService {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        AsyncRead::poll_read(Pin::new(&mut self.stdout), cx, buf)
    }
}

/// Launches a new subprocess, returning a Channel for communication.
/// The Channel may then be used to send gRPC data over stdin/stdout.
///
/// * `name`: The name of the subprocess to launch.
/// * `args`: A vector of arguments to pass to the application on startup.
pub async fn new_subprocess(
    name: String,
    args: Vec<String>,
    search_dir: PathBuf,
) -> Result<tonic::transport::Channel> {
    // NOTE: The URL passed to `from_shared` must resemble a real URI, but it is not used.
    // This is why we use `example.com`. No connection to that resource is ever made.
    let channel = Endpoint::from_shared("http://example.com")?
        .connect_with_connector(service_fn(move |_: Uri| {
            let name = name.clone();
            let args = args.clone();
            let search_dir = search_dir.clone();

            async move {
                let command = Command::new(&name)
                    .args(&args)
                    .current_dir(&search_dir)
                    .env("PATH", &search_dir)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;

                Ok::<_, anyhow::Error>(StdioService::new(command))
            }
        }))
        .await?;

    Ok(channel)
}
