use crate::api_versions::{ApiKeys, ApiVersionsResponse, build_apiversions_response};
use crate::byte_cursor::Buf;
use crate::describe_topic_partitions::{DescribeTopicPartitionsResponse, build_describe_response};
use crate::encoder::Encode;
use crate::fetch::{FetchResponse, build_fetch_response};
use std::io;

struct Header {
    api_key: i16,
    api_version: i16,
    correlation_id: i32,
    // client_id and TAG_BUFFER will be skipped for now
    // client_id: ??,
    // tag_buffer: u8,
}

enum ApiResponse {
    ApiVersions(ApiVersionsResponse),
    DescribeTopicPartitions(DescribeTopicPartitionsResponse),
    Fetch(FetchResponse),
}

impl Encode for ApiResponse {
    fn encode(&self, correlation_id: i32) -> Vec<u8> {
        match self {
            ApiResponse::ApiVersions(r) => r.encode(correlation_id),
            ApiResponse::DescribeTopicPartitions(d) => d.encode(correlation_id),
            ApiResponse::Fetch(f) => f.encode(correlation_id),
        }
    }
}

fn parse_header(buf: &mut Buf) -> Header {
    let api_key = buf.read_i16();
    let api_version = buf.read_i16();
    let correlation_id = buf.read_i32();

    // skip client_id and TAG_BUFFER
    let client_id_len = buf.read_i16();
    if client_id_len > 0 {
        buf.skip(client_id_len as usize); // skip client_id
    }
    buf.skip(1); // skip TAG_BUFFER which is assumed to be 1 byte for now

    Header {
        api_key,
        api_version,
        correlation_id,
    }
}

pub fn build_response(payload: Vec<u8>) -> Result<Vec<u8>, io::Error> {
    println!("building response");
    let mut buf = Buf::new(payload);
    let header = parse_header(&mut buf);

    let api_response = match header.api_key {
        18 => ApiResponse::ApiVersions(build_apiversions_response(header.api_version)),
        75 => ApiResponse::DescribeTopicPartitions(build_describe_response(&mut buf)?),
        1 => ApiResponse::Fetch(build_fetch_response(&mut buf)?),
        _ => {
            panic!("api_key {} not yet implemented", header.api_key);
        }
    };

    let body = api_response.encode(header.correlation_id);
    let message_size: i32 = body.len() as i32;

    let mut response: Vec<u8> = Vec::new();
    response.extend_from_slice(&message_size.to_be_bytes());
    response.extend_from_slice(&body);
    Ok(response)
}
