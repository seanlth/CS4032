use std::io::prelude::*;
use std::net::TcpStream;

use std::io;

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    // can change to localhost
    let addr = "109.78.84.202";

    if let Ok(mut s) = TcpStream::connect( &*(addr.to_string() + ":25566") ) {
        let request: String = format!("{}", line);
        let _ = s.write(request.as_bytes());
        let mut buf = [0; 1024];
        let _ = s.read(&mut buf);
        println!("{}", String::from_utf8_lossy( &buf[0..1024] ));
    }
}
