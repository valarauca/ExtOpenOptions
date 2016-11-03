/*
Copyright (C) 2016-August-07 William Cody Laeder


Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
associated documentation files (the "Software"), to deal in the Software without restriction,
including without limitation the rights to use, copy, modify, merge, publish, distribute,
sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or
substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE X CONSORTIUM BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
extern crate libc;
use libc::{ lseek, open, read, write, O_APPEND, O_ASYNC, O_CREAT, O_DIRECT, O_DIRECTORY, O_EXCL, O_NOCTTY, O_NOFOLLOW, O_NONBLOCK, O_SYNC, O_TRUNC,O_WRONLY,O_RDWR};

use std::io::Error;
use std::os::unix::io::RawFd;
use std::mem;
use std::io::SeekFrom;

pub fn is_str_null_terminated( buffer: &str ) -> bool {
    let end_index = buffer.as_bytes().len() - 1usize;
    buffer.as_bytes()[end_index] == 0u8
}
#[test]
fn test_is_str_null_terminated() {

    let dut_pid = 123456i32;
    let dut_a = format!("/proc/{}/mem", dut_pid);
    let dut_b = format!("/proc/{}/mem\x00", dut_pid);

    assert!( ! is_str_null_terminated( &dut_a ));
    assert!(   is_str_null_terminated( &dut_b ));
}


pub struct ExtOpenOptions {
    options: i32
}
impl ExtOpenOptions {

    //
    //Build a new ExtendedOpenOptions
    //
    pub fn new() -> ExtOpenOptions {
        ExtOpenOptions {
            options: 0i32
        }
    }

    //
    //This whole section covers Linux x64 Kernel Vulns
    //      See $man open
    //      or http://linux.die.net/man/2/open
    //
    pub fn append(self) -> ExtOpenOptions {
        let mut x = self;
        x.options |= O_APPEND;
        x
    }
    pub fn async(self) -> ExtOpenOptions {
        let mut x = self;
        x.options |= O_ASYNC;
        x
    }
    pub fn create(self) -> ExtOpenOptions {
        let mut x = self;
        x.options |= O_CREAT;
        x
    }
    pub fn direct(self) -> ExtOpenOptions {
        let mut x = self;
        x.options |= O_DIRECTORY;
        x
    }
    pub fn excl(self) -> ExtOpenOptions {
        let mut x = self;
        x.options |= O_EXCL;
        x
    }
    pub fn noctty(self) -> ExtOpenOptions {
        let mut x = self;
        x.options |= O_NOCTTY;
        x
    }
    pub fn nofollow(self) -> ExtOpenOptions {
        let mut x = self;
        x.options |= O_NOFOLLOW;
        x
    }
    pub fn nonblocking(self) -> ExtOpenOptions {
        let mut x = self;
        x.options |= O_NONBLOCK;
        x
    }
    pub fn sync(self) -> ExtOpenOptions {
        let mut x = self;
        x.options |= O_SYNC;
        x
    }
    pub fn truncating(self) -> ExtOpenOptions {
        let mut x = self;
        x.options |= O_TRUNC;
        x
    }
    pub fn write_only(self) -> ExtOpenOptions {
        let mut x = self;
        x.options |= O_WRONLY;
        x
    }
    pub fn readwrite(self) -> ExtOpenOptions {
        let mut x = self;
        x.options |= O_RDWR;
        x
    }

    //
    //Consume open
    //
    //      This function consumes the options and makes the actual call into open(2)
    //      If the path being passed in is NOT NULL TERMINATED, it will re-allocate
    //      the path to be NULL TERMINATED
    //
    //      If no write_only/readwite are used the default behavior is READ_ONLY
    //
    pub fn open(self, path: &str ) -> Result<RawFd, Error> {
        if is_str_null_terminated( path ) {
            let p = format!("{}\x00", path );
            let ret: i32 = unsafe{ open( mem::transmute(p.as_ptr()), self.options) };
            if ret == -1i32 {
                Err( Error::last_os_error() )
            } else {
                Ok( ret as RawFd )
            }
        } else {
            let ret: i32 = unsafe{ open( mem::transmute(path.as_ptr()), self.options) };
            if ret == -1i32 {
                Err( Error::last_os_error() )
            } else {
                Ok( ret as RawFd )
            }
        }
    }

}


//  Ok(true)    -> EVERYTHING GOOD
//  Ok(false)   -> reading more then buffer
//  Err(::std::io::Error) ->
pub fn safe_read( fd: i32, buffer: &mut [u8], count: usize) -> Result<bool,Error> {
    if count > buffer.len() {
        Ok(false)
    } else {
        let ret = unsafe{ read( fd, mem::transmute( buffer.as_mut_ptr() ), count ) };
        if ret == -1 {
            Err( Error::last_os_error() )
        } else {
            Ok(true)
        }
    }
}
pub fn generic_read< G: Sized >( fd: i32, buffer: &mut G ) -> Result<(),Error> {
    let read_size: usize = mem::size_of::< G >();
    let ret = unsafe{ read( fd, mem::transmute( buffer ), read_size ) };
    if ret == -1 {
        Err( Error::last_os_error() )
    } else {
        Ok(())
    }
}


pub fn safe_write( fd: i32, buffer: &[u8], count: usize) -> Result<bool,Error> {
    if count > buffer.len() {
        Ok(false)
    } else {
        let ret = unsafe{ write( fd, mem::transmute( buffer.as_ptr() ), count ) };
        if ret == -1 {
            Err( Error::last_os_error() )
        } else {
            Ok(true)
        }
    }
}
pub fn generic_write< G: Sized >( fd: i32, buffer: &G ) -> Result<(),Error> {
    let read_size: usize = mem::size_of::< G >();
    let ret = unsafe{ write( fd, mem::transmute( buffer ), read_size ) };
    if ret == -1 {
        Err( Error::last_os_error() )
    } else {
        Ok(())
    }
}


pub fn fseek( fd: i32, operation: SeekFrom ) -> Result<i64,Error> {
    let mut seek_op = 0i32;
    let mut dist = 0i64;
    match operation {
        SeekFrom::Start(x) => {
            seek_op = 0;
            dist = x as i64;
        },
        SeekFrom::End(x) => {
            seek_op = 2;
            dist = x;
        },
        SeekFrom::Current(x) => {
            seek_op = 1;
            dist = x;
        }
    };
    let ret = unsafe{ lseek( fd, dist, seek_op )};
    if ret == 0 {
        Ok(ret as i64)
    } else {
        Err(Error::last_os_error())
    }
}
pub fn fsize( fd: i32 ) -> Result<i64,Error> {
    let size = match fseek( fd, SeekFrom::End(0) ) {
        Ok(x) => x,
        Err(e) => return Err(e)
    };
    //reset buffer
    match fseek( fd, SeekFrom::Start(0) ) {
        Ok(_) => Ok(size),
        Err(e) => Err(e)
    }
}
