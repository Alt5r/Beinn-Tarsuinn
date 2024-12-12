use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod rules;  
use rules::*;

mod parsing;
use parsing::*;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection was closed by the client
            Ok(n) => {
               // println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

                // conv to vector of strings for the request parameters

                let request = listify(String::from_utf8_lossy(&buffer[..n]).to_string());

                let r: Result<String, std::net::AddrParseError> = master(request);

                
                // Echo the message back to the client
                if let Err(e) = stream.write_all(&buffer[..n]) {
                    eprintln!("Failed to send response: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                break;
            }
        }
    }
    println!("Connection closed.");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from: {}", stream.peer_addr()?);
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
