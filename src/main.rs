use std::io;
use std::io::{BufRead, BufReader};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[cfg(unix)]
use std::os::unix::io::FromRawFd;

#[cfg(target_os = "wasi")]
use std::os::wasi::io::FromRawFd;

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buf = String::new();
        let mut reader = BufReader::new(&stream);
        let result = reader.read_line(&mut buf);
        match result {
            Ok(0) => {
                break;
            }
            Ok(data) => {
                stream.write_all(buf.as_bytes());
                println!("{}", buf.trim());
            }
            Err(e) => {
                println!("error reading: {}", e);
                break;
            }
        }
    }
}

fn main() {
    eprintln!("Starting up");

    // Setup the TCP server socket.
    let listener = unsafe { TcpListener::from_raw_fd(3) };

    listener.set_nonblocking(false);

    eprintln!("accepting connections from 127.0.0.1:9000");

    for stream in listener.incoming() {
        match stream {
            Err(e) => println!("failed: {}", e),
            Ok(stream) => {
                handle_client(stream);
            }
        }
    }
}
