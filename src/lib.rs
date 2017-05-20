extern crate libc;

use libc::*;

use std::net::{TcpStream};
//use std::thread;
use std::io::{Read, Write};
use std::str;

use std::os::unix::io::AsRawFd;

#[link(name = "osl", kind = "static")]
#[link(name = "ssl", kind = "static")]
#[link(name = "crypto", kind = "static")]
extern {

    fn newctx(cert_file: *const c_uchar, key_file: *const c_uchar) -> *mut c_void;

    fn newssl(ctx: *const c_void, fd: c_int) -> *mut c_void;

    //fn sslaccept(ssl: *const c_void) -> c_int;

    fn sslread(ssl: *mut c_void, buf: *mut c_uchar, size: c_int) -> c_int;

    fn sslwrite(ssl: *mut c_void, buf: *const c_uchar, size: c_int) -> c_int;

    fn deletessl(ssl: *mut c_void);

    fn deletectx(ssl: *mut c_void);

}

// need to allow dead code because we need to own the stream
// even though we dont use it "directly"
#[allow(dead_code)]
pub struct OsslStream {
    ssl: *mut c_void,
    stream: TcpStream,
}

unsafe impl Send for OsslStream {}

impl OsslStream {

    pub fn accept(ctx: &Ctx, mut stream: TcpStream) -> Result<Self, &'static str> {
        let ssl: *mut c_void;

        unsafe { ssl = newssl(ctx.0, stream.as_raw_fd()) }

        if ssl == std::ptr::null::<c_void>() as *mut c_void {
            // for now, we tell the client to reconnect using ssl
            // could return the stream back in Err in order to handle more
            // situations
            println!("NOT TLS");
            stream.write_all(b"HTTP/1.1 307 Temporary Redirect\r\nLocation: https://localhost:8080\r\n\r\n").unwrap();
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            return Err("can't accpet")
        }

        Ok ( OsslStream { ssl, stream } )
    }
}

impl Read for OsslStream {

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = unsafe { sslread(self.ssl, buf.as_mut_ptr() as *mut u8, buf.len() as c_int) };
        Ok(n as usize)
    }
}

impl Write for OsslStream {

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = unsafe { sslwrite(self.ssl, buf.as_ptr() as *const u8, buf.len() as c_int) };
        Ok(n as usize)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Drop for OsslStream {

    fn drop(&mut self) {
        println!("droping ssl stream");
        unsafe { deletessl(self.ssl) };
    }
}

//#[derive(Clone, Copy)]
pub struct Ctx(*mut c_void);

unsafe impl Send for Ctx {}

impl Drop for Ctx {

    fn drop(&mut self) {
        println!("droping ctx");
        unsafe { deletectx(self.0) };
    }
}

pub fn make_ctx(cert_file: &str, key_file: &str) -> Ctx {
    Ctx ( unsafe { newctx (  (String::from(cert_file) + "\0").as_ptr() as *const c_uchar,
                             (String::from(key_file) + "\0").as_ptr() as *const c_uchar
                ) }
        )
}

// mod tests {

//     use super::*;

//     #[test]
//     fn it_works() {
//         let listener = TcpListener::bind("127.0.0.1:8080").unwrap();


//         let ctx = get_ctx();

//         for stream in listener.incoming() {
//             match stream {
//                 Ok(stream) => {
//                     println!("{:?}", stream);
//                     println!("CHECK SSL");

//                     //##############################################

//                     if  let Ok(mut ssl_stream) = OsslStream::accept(&ctx, stream) {
//                         //println!("{:?}", ssl_stream);
//                         thread::spawn(move|| {
//                             println!("GOT SSL CONNECTION");

//                             let mut buf = [0;4096];

//                             let n = ssl_stream.read(&mut buf).unwrap();

//                             println!("{}", n);

//                             let s = str::from_utf8(&buf[..n]).unwrap();

//                             //for c in buf.iter() {
//                             println!("{}", s);
//                             //}

//                             ssl_stream.write(b"HTTP/1.1 200 OK\r\n\r\nhello world!").unwrap();

//                             //handle_client(tls_stream);
//                             return;
//                         });
//                     }

//                 }
//                 Err(_) => { /* connection failed */ }
//             }
//         }

//         println!("Hello, world!");
//     }
// }
