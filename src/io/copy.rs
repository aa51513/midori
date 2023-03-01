use std::io;

use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};

use crate::utils::BUF_SIZE;

pub async fn copy<R, W>(mut r: R, mut w: W) -> io::Result<()>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut buf = vec![0u8; BUF_SIZE];
    let mut n: usize;
    loop {
        n = r.read(&mut buf).await?;
        if n == 0 {
            break;
        };
        let write_bytes_count = w.write(&buf[..n]).await?;
        if write_bytes_count!=n {
        }
    }
    w.shutdown().await?;
    Ok(())
}
