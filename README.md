#`kernelx64`


This library offers safe abstractions around the crate `libc`. The goal is to
allow working with native file descriptors not painful in Rust.

###`read(3)` `write(3)`

`safe_read( fd: i32, buff: &mut [u8], count: usize) -> Result<bool,Error>`

`safe_write( fd: i32, buff: &[u8], count: usize ) -> Result<bool,Error>`

Safe Read/Write allow for a buffer larger then the intended write/read be used.
When `Ok(false)` is returned `count > buff.len()` is `true`. This means the
write/read being execute are larger then the allocation it is being operated on.

`generic_read< G: Sized >( fd: i32, buff: &mut G) -> Result<(),Error>`

`generic_write< G: Sized >( fd: i32, buff: &G ) -> Result<(), Error>`

The function `::std::mem::size_of<G>()` will be called to determine the size
of the read/write. While the the pointer to G will be treated as a `*void` in
C-Lang. Rust lang's size of does account for llvm/rustc added struct padding.


###`open(3)`

See: `ExtOpenOptions` below

Example Usage:

      let path = "/proc/143203/maps";
      let mut file = match ExtOpenOptions::new().direct().open( path ) {
        Ok(x) => unsafe{ File::from_raw_fd( x ) },
        Err(e) => panic!("Error on sys_call open(2). Error: {:?}", e )
      };

It should be noted that this function returns a RawFd not a File. For additional
documentation please see:

https://doc.rust-lang.org/std/os/unix/io/trait.FromRawFd.html#tymethod.from_raw_fd

https://doc.rust-lang.org/std/os/unix/io/trait.AsRawFd.html#tymethod.as_raw_fd

For a full description of the open(2) interface please see:

http://linux.die.net/man/2/open


Deep Dive:

     `ExtOpenOptions::new()`

Options value is 0. This corresponds to a READ_ONLY ( O_RDONLY ) flag.

###`.append()`

The file is opened in append mode. Before each write(2), the file offset is
positioned at the end of the file, as if with lseek(2). `.append()` may lead to
corrupted files on NFS file systems if more than one process appends data to a
file at once. This is because NFS does not support appending to a file, so the
client kernel has to simulate it, which can't be done without a race condition

###`.async()`

Enable signal-driven I/O: generate a signal (SIGIO by default, but this can be
changed via fcntl(2)) when input or output becomes possible on this file
descriptor. This feature is only available for terminals, pseudoterminals,
sockets, and (since Linux 2.6) pipes and FIFOs. See fcntl(2) for further details

###`.create()`

If the file does not exist it will be created. The owner (user ID) of the file
is set to the effective user ID of the process. The group ownership (group ID)
is set either to the effective group ID of the process or to the group ID of the
parent directory (depending on file system type and mount options, and the mode
of the parent directory, see the mount options bsdgroups and sysvgroups
described in mount(8)).

###`.direct()`

Try to minimize cache effects of the I/O to and from this file. In general this
will degrade performance, but it is useful in special situations, such as when
applications do their own caching. File I/O is done directly to/from user-space
buffers. The `.direct()` flag on its own makes an effort to transfer data
synchronously, but does not give the guarantees of the sync flag that data and
necessary metadata are transferred. To guarantee synchronous I/O, `.sync()` must
be used in addition to `.direct()`.

For more information see Notes on `O_DIRECT`: http://linux.die.net/man/2/open

###`.excl()`

Ensure that this call creates the file: if this flag is specified in conjunction
with `.create()`, and pathname already exists, then open() will fail. When these
two flags are specified, symbolic links are not followed: if pathname is a
symbolic link, then open() fails regardless of where the symbolic link points to

###`.noctty()`

If pathname refers to a terminal device--see tty(4)--it will not become the
process's controlling terminal even if the process does not have one.

###`.nofollow()`

If pathname is a symbolic link, then the open fails. This is a FreeBSD extension
which was added to Linux in version 2.1.126. Symbolic links in earlier
components of the pathname will still be followed.

###`.nonblocking()`

When possible, the file is opened in nonblocking mode. Neither the open() nor
any subsequent operations on the file descriptor which is returned will cause
the calling process to wait. For the handling of FIFOs (named pipes), see also
fifo(7). For a discussion of the effect of `.nonblocking()` in conjunction with
mandatory file locks and with file leases, see fcntl(2).

###`.sync()`

The file is opened for synchronous I/O. Any write(2)s on the resulting file
descriptor will block the calling process until the data has been physically
written to the underlying hardware.

For more information see notes on `O_SYNC`: http://linux.die.net/man/2/open

###`.truncating()`

If the file already exists and is a regular file and the open mode allows
writing (i.e., is `.readwrite()` or `.write_only()`) it will be truncated to
length 0. If the file is a FIFO or terminal device file, the `.truncating()`
flag is ignored. Otherwise the effect of `.truncating()` is unspecified.

###`.write_only()`

Program will only write to FD

###`.readwrite()`

Program will read and write to file descriptor

###`.open( &str ) -> Result< RawFd, ::std::io::Error>`

This function will check if the string being passed to it is null terminated.
In the event it is not it will re-allocate the string.

##Project License:

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
