use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/// this will perform the proxying feature of the sofrwatere, acutally getting
/// a respiksne for the the forwared request ? database implenetation
pub fn forward(original_request: &str) -> std::io::Result<String> {

    let trgt= "google.com:80";
    let mut trgt_strm = TcpStream::connect(trgt)?;
    trgt_strm.write_all(original_request.as_bytes());
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
    
    Ok(response) // Return the accumulated response

}