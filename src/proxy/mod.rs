use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/// this will perform the proxying feature of the sofrwatere, acutally getting
/// a respiksne for the the forwared request ? database implenetation
pub fn forward(original_request: &str) -> std::io::Result<String> {
    println!("forward has been called");

    let trgt= "en.wikipedia.org:80/wiki/United_Kingdom";
    let mut trgt_strm = TcpStream::connect(trgt)?;

    let or = original_request;
    let or = or.replace("localhost:8080", "en.wikipedia.org:80/wiki/United_Kingdom");
    let or = or.replace("127.0.0.1:8080", "en.wikipedia.org:80/wiki/United_Kingdom");
    let or = or.replace("Connection: keep-alive", "Connection: close");

    println!("{}", or);

    trgt_strm.write_all(or.as_str().as_bytes());
    trgt_strm.flush();

    let mut response_buffer = [0; 4096];

    let mut response = String::new();

    loop {
        let byts_read = trgt_strm.read(&mut response_buffer)?;
        if byts_read == 0 {
            break;
        }

     // Append the data read to the response
     response.push_str(&String::from_utf8_lossy(&response_buffer[..byts_read]));
    }
    println!("this is the response data from the req {}", response);
    
    Ok(response) // Return the accumulated response

}