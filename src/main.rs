use std::{io::{Write, Read}, net::{TcpListener, TcpStream}, sync::Arc, thread};

fn main() {
    println!("Logs from your program will appear here!");
  
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                let stream = Arc::new(stream);
                thread::spawn(move || {
                    respond(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn respond(stream: Arc<TcpStream>) {
    let mut buffer:[u8; 1000] = [0; 1000];
    let mut stream = stream.try_clone().unwrap();
    loop {
        match stream.read(&mut buffer) {
            Ok(_) => {
                let mut stream = stream.try_clone().unwrap();
                stream.write(b"+PONG\r\n").unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        }
    }
}
