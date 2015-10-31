use std::ffi::CString;
use std::ffi::CStr;

use libc::*;


#[link(name = "host")]
extern {
    fn get_host(s: *const c_char) -> *const c_char;
}

pub fn ip_of_host(host: &str) -> String {
    unsafe {
        let s: *const c_char = get_host( CString::new(host).unwrap().as_ptr() );


        let slice = CStr::from_ptr( s );

        String::from_utf8_lossy(slice.to_bytes()).as_ref().to_string()
    }
}
