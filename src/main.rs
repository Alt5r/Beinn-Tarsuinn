use std::{collections::HashMap, io::{Read, Write}, net::SocketAddr};
use tokio::net::TcpListener; // Use async TcpListener
use surrealdb::engine::remote::ws::Client; // WebSocket Client engine
use surrealdb::Surreal;
use surrealdb::opt::auth::Namespace;
use tokio::task;

mod rules;
use rules::*;
mod parsing;
use parsing::*;
mod proxy;
use proxy::*;
use crate::rules::honeypot::directoryChecker;

mod csv;
use csv::*;

use tokio::io::AsyncReadExt; // Add this import
use tokio::io::AsyncWriteExt; // Add this import



async fn handle_client(mut stream: tokio::net::TcpStream, addr: SocketAddr) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => break, // Connection was closed by the client
            Ok(n) => {
                 // println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

                // Convert the request into a vector of strings for parameters
                let request = listify(String::from_utf8_lossy(&buffer[..n]).to_string());

                if let Ok(lst) = get_agents().await {
                    println!("Have a list from csv");
                    // Call your directoryChecker function
                
                
                if let Ok(result) = directoryChecker(request, &addr, lst).await {
                    if result == true {
                        // serve the request
                        let r = "\
                            HTTP/1.1 200 OK\r\n\
                            Content-Type: text/html; charset=UTF-8\r\n\
                            Content-Length: 51\r\n\
                            \r\n\
                            <html>\
                                <head><title>How bout get fucked</title></head>\
                                <body><h1>Hello, world!</h1></body>\
                            </html>";

                            stream.write_all(r.as_bytes()).await;
                    } else {
                        // dont serve the request 
                        println!("client was malicious so no content was served")
                    }
                }


                } else {
                    //eprintln!("error in csv reading: {}", e);
                }
                

                
                /* 
                let trgt = "google.com:80";
                let response = forward(&String::from_utf8_lossy(&buffer[..n]).to_string(), trgt).unwrap();

                // Send the response back to the client
                if let Err(e) = stream.write_all(response.as_bytes()).await {
                    eprintln!("Failed to send response: {}", e);
                    break;
                }
            
            */
        }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                break;
            }
            
            

        }
    }
    println!("Connection closed.");
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("New connection from: {}", addr);
                // Use tokio::spawn for async tasks
                tokio::spawn(handle_client(stream, addr));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
