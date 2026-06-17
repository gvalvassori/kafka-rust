#![allow(unused_imports)]
mod api_versions;
mod byte_cursor;
mod describe_topic_partitions;
mod dispatch;
mod encoder;
mod fetch;
mod log_reader;
mod stream;

use crate::stream::handle_stream;

use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                thread::spawn(move || handle_stream(&mut stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
