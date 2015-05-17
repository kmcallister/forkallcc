#![feature(libc)]
#![deny(warnings)]

extern crate libc;

use std::mem;
use std::marker::PhantomData;
use std::os::unix::io::RawFd;
use libc::{c_void, size_t, ssize_t};
use libc::funcs::posix88::unistd::fork;

/// A continuation, accepting an argument of type `T`.
pub struct Cont<T> {
    fd: RawFd,
    phantom: PhantomData<Box<FnOnce(T)>>,
}

impl<T> Cont<T>
    where T: Copy + Send + 'static,
{
    /// Invoke the continuation.
    pub fn invoke(&self, x: T) -> ! {
        let ptr = &x as *const T;
        let size = mem::size_of::<T>();
        unsafe {
            let r = libc::write(self.fd, ptr as *const c_void, size as size_t);
            assert_eq!(r, size as ssize_t);
            libc::exit(0)
        }
    }
}

impl<T> Drop for Cont<T> {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}

/// Call with current continuation.
pub fn call_cc<T, F>(f: F) -> T
    where T: Copy + Send + 'static,
          F: FnOnce(Cont<T>) -> T
{
    unsafe {
        let mut fds: [RawFd; 2] = [0, 0];
        let r = libc::pipe(fds.as_mut_ptr());
        assert_eq!(r, 0);

        let pid = fork();

        if pid < 0 {
            panic!("fork() failed");
        } else if pid > 0 {
            // parent: call f immediately.
            libc::close(fds[0]);
            f(Cont {
                fd: fds[1],
                phantom: PhantomData,
            })
        } else {
            // child: wait for the continuation to be invoked.
            libc::close(fds[1]);

            // read(2) will return 0 if the write end of the pipe is closed.
            // This will happen when the parent exits, or when the
            // corresponding Cont<T> is deleted.
            //
            // We naively assume the value can be read in a single call to
            // read(2).
            let mut buf: T = mem::uninitialized();
            let ptr = &mut buf as *mut T;
            let size = mem::size_of::<T>();

            if libc::read(fds[0], ptr as *mut c_void, size as size_t) <= 0 {
                libc::exit(0);
            }
            libc::close(fds[0]);
            buf
        }
    }
}
