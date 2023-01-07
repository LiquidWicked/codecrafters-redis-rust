use std::{io::Write, net::{TcpListener, TcpStream}};

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
    stream.write(b"+PONG\r\n").unwrap();
}