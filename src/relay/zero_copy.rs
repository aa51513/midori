use std::io;
use std::ops::Drop;
use std::os::unix::io::AsRawFd;

use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};

use crate::utils::{self, PIPE_BUF_SIZE};

pub struct Pipe(i32, i32);

impl Drop for Pipe {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.0);
            libc::close(self.1);
        }
    }
}

impl Pipe {
    pub fn create() -> io::Result<Self> {
        use libc::{c_int, O_NONBLOCK};
        let mut pipes = std::mem::MaybeUninit::<[c_int; 2]>::uninit();
        unsafe {
            if libc::pipe2(pipes.as_mut_ptr() as *mut c_int, O_NONBLOCK) < 0 {
                return Err(utils::new_io_err("failed to create a pipe"));
            }
            Ok(Pipe(pipes.assume_init()[0], pipes.assume_init()[1]))
        }
    }
}

#[inline]
pub fn splice_n(r: i32, w: i32, n: usize) -> isize {
    use libc::{loff_t, SPLICE_F_MOVE, SPLICE_F_NONBLOCK};
    unsafe {
        libc::splice(
            r,
            std::ptr::null_mut::<loff_t>(),
            w,
            std::ptr::null_mut::<loff_t>(),
            n,
            SPLICE_F_MOVE | SPLICE_F_NONBLOCK,
        )
    }
}

#[inline]
pub fn is_wouldblock() -> bool {
    use libc::{EAGAIN, EWOULDBLOCK};
    let errno = unsafe { *libc::__errno_location() };
    errno == EWOULDBLOCK || errno == EAGAIN
}

pub async fn copy<X, Y, R, W>(mut r: R, mut w: W) -> io::Result<()>
where
    X: AsRawFd,
    Y: AsRawFd,
    R: AsyncRead + AsRef<X> + Unpin,
    W: AsyncWrite + AsRef<Y> + Unpin,
{
    // create pipe
    let pipe = Pipe::create()?;
    let (rpipe, wpipe) = (pipe.0, pipe.1);
    // get raw fd
    let rfd = r.as_ref().as_raw_fd();
    let wfd = w.as_ref().as_raw_fd();
    let mut n: usize = 0;
    let mut done = false;

    'LOOP: loop {
        // read until the socket buffer is empty
        // or the pipe is filled
        r.read(&mut [0u8; 0]).await?;
        while n < PIPE_BUF_SIZE {
            match splice_n(rfd, wpipe, PIPE_BUF_SIZE - n) {
                x if x > 0 => n += x as usize,
                x if x == 0 => {
                    done = true;
                    break;
                }
                x if x < 0 && is_wouldblock() => break,
                _ => break 'LOOP,
            }
        }
        // write until the pipe is empty
        while n > 0 {
            w.write(&[0u8; 0]).await?;
            match splice_n(rpipe, wfd, n) {
                x if x > 0 => n -= x as usize,
                x if x < 0 && is_wouldblock() => {
                    // clear readiness (EPOLLOUT)
                    w.write(&[0u8; 0]).await?;
                }
                _ => break 'LOOP,
            }
        }
        // complete
        if done {
            break;
        }
        // clear readiness (EPOLLIN)
        r.read(&mut [0u8; 0]).await?;
    }

    w.shutdown().await?;
    Ok(())
}
