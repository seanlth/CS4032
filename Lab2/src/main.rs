use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::io::prelude::*;

extern crate threadpool;
use threadpool::ThreadPool;

extern crate Lab2;

use Lab2::host::*;


fn handle_connection(mut stream: TcpStream, kill: Arc<Mutex<bool>>, port: String)  {
    if let Ok(local) = stream.local_addr() {
        // let ip_string = match local {
        //     SocketAddr::V4(ip) => {
        //         format!("{}", ip.ip())
        //     },
        //     SocketAddr::V6(ip) => {
        //         format!("{}", ip.ip())
        //     }
        // };

        let ip_string = ip_of_host("seanlth.duckdns.org");


        let mut buf = [0; 1024];
        if let Ok(size) = stream.read(&mut buf) {
            let message  = &*(String::from_utf8_lossy( &buf[0..size] ));

            if message.len() >= 4 {
                if &message[0..4] == "HELO" {
                    let reply = message.to_string() + &*format!("IP:{}\nPort:{}\nStudentID:f5438a275459e6d07c647e34bc1650be7565c817e7fe3c89ee28c122e3162fb0\n", ip_string, local.port() );
                    let _ = stream.write( reply.to_string().as_bytes() );
                }
            }
            else if message == "KILL_SERVICE" {
                let mut k = kill.lock().unwrap();
                *k = true;
                let _ = TcpStream::connect( &*("127.0.0.1:".to_string() + &port.to_string()));
            }
            else {
                let _ = stream.write( "Unsupported\n".to_string().as_bytes() );
            }
        }
    }
}

fn main() {


    if let Some(port) = std::env::args().nth(1) {
        let kill = Arc::new(Mutex::new(false));

        let listener = TcpListener::bind( &*("0.0.0.0:".to_string() + &*port.to_string()) ).unwrap();
        let pool = ThreadPool::new(4);

        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    let k = kill.lock().unwrap();
                    if *k == true { break; }

                    let data_clone = kill.clone();
                    let port_clone = port.clone();

                    pool.execute(move || {
                        handle_connection(stream, data_clone, port_clone.clone());
                    });
                },
                Err(_) => { println!("An error occured " ); }
            }
        }
    }
}
