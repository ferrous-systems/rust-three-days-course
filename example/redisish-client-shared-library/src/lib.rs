use std::os::raw::{c_char};
use std::io::prelude::*;
use std::ffi::*;
use std::net::TcpStream;

#[no_mangle]
extern "C" fn send_message(string: *const c_char) {
    let msg = unsafe { CStr::from_ptr(string) };
    let slice = msg.to_str().unwrap();
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let payload = format!("PUBLISH {}\n", slice);

    stream.write(payload.as_bytes());

}


#[no_mangle]
extern "C" fn retrieve_message() -> *const c_char {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let payload = "RETRIEVE\n".to_string();

    stream.write(payload.as_bytes());
    stream.shutdown(std::net::Shutdown::Write);

    let mut line = String::new();
    stream.read_to_string(&mut line).unwrap();

    let c_str = CString::new(line).unwrap();
    CString::into_raw(c_str)
}