use std::{io::{Write, Read}, net::{TcpListener, TcpStream}};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
  
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                respond(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn respond(mut stream: TcpStream) {
    
    loop {
        let mut buffer:[u8; 1000] = [0; 1000];
        stream.read(&mut buffer).unwrap();
        stream.write(b"+PONG\r\n").unwrap();
    }
}