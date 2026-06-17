use crate::dispatch::build_response;

use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_stream(stream: &mut TcpStream) {
    println!("handling stream");

    while let Some(body) = read_one_request(stream) {
        match build_response(body) {
            Ok(response) => {
                if let Err(e) = stream.write_all(&response) {
                    eprintln!("error writing response {}", e);
                };
            }
            Err(e) => {
                eprintln!("error building response {}", e);
            }
        }
    }
}

fn read_one_request(stream: &mut TcpStream) -> Option<Vec<u8>> {
    // Read the first 4 bytes of a stream which is the message_size
    // and then read the following message_size bytes that is the body.
    let mut body_size = [0u8; 4];
    if stream.read_exact(&mut body_size).is_err() {
        return None;
    }
    let body_len = u32::from_be_bytes(body_size) as usize;

    let mut payload = vec![0u8; body_len];
    stream.read_exact(&mut payload).ok()?;
    Some(payload)
}
