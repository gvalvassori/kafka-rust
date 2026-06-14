#![allow(unused_imports)]
use std::convert::TryInto;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // TODO: Uncomment the code below to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_stream(&stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_stream(mut stream: &TcpStream) {
    println!("Handling stream");
    let mut buffer = [0u8; 64];
    let _ = stream.read(&mut buffer);
    let message_size_bytes: [u8; 4] = buffer[0..4].try_into().unwrap();
    // Convert big-endian bytes to 32-bits integer
    // let message_size = i32::from_be_bytes(message_size_bytes);
    let correlation_id_bytes: [u8; 4] = buffer[8..12].try_into().unwrap();
    // let correlation_id = i32::from_be_bytes(correlation_id_bytes);
    let api_key_bytes: [u8; 2] = buffer[4..6].try_into().unwrap();
    let api_key = i16::from_be_bytes(api_key_bytes);
    let api_keys_arr = api_versions_response(api_key);
    let api_keys_byte = serialize_api_keys(&api_keys_arr);
    let api_version_bytes: [u8; 2] = buffer[6..8].try_into().unwrap();
    let api_version = i16::from_be_bytes(api_version_bytes);
    let error_code: [u8; 2] = check_valid_api_version(api_version);

    let throttle_time_ms: i32 = 0; // Placeholder
    let tagged_fields: i8 = 0; // Placeholder

    // Build the [u8; 8] but since this project will grow I will use Vec<u8>
    let mut body: Vec<u8> = Vec::new();
    body.extend_from_slice(&correlation_id_bytes);
    body.extend_from_slice(&error_code);
    body.extend_from_slice(&api_keys_byte);
    body.extend_from_slice(&throttle_time_ms.to_be_bytes());
    body.extend_from_slice(&tagged_fields.to_be_bytes());

    let message_size: i32 = body.len() as i32;

    let mut response: Vec<u8> = Vec::new();
    response.extend_from_slice(&message_size.to_be_bytes());
    response.extend_from_slice(&body);
    let _ = stream.write(&response);
}

fn check_valid_api_version(version: i16) -> [u8; 2] {
    println!("checking api version");
    let error_code: i16 = if (0..=4).contains(&version) { 0 } else { 35 };
    let error_code_bytes: [u8; 2] = i16::to_be_bytes(error_code);
    error_code_bytes
}

struct ApiKeys {
    api_key: i16,
    min_version: i16,
    max_version: i16,
}

fn api_versions_response(api_key: i16) -> Vec<ApiKeys> {
    println!("api versions response");
    let mut api_keys: Vec<ApiKeys> = Vec::new();
    api_keys.push(ApiKeys {
        api_key,
        min_version: 0,
        max_version: 4,
    });
    api_keys
}

fn serialize_api_keys(api_keys: &[ApiKeys]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    out.push((api_keys.len() + 1) as u8);
    for api_key in api_keys {
        out.extend_from_slice(&api_key.api_key.to_be_bytes());
        out.extend_from_slice(&api_key.min_version.to_be_bytes());
        out.extend_from_slice(&api_key.max_version.to_be_bytes());
        out.push(0u8);
    }
    out
}
