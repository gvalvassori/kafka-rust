use crate::api_versions::ApiKeys;
use crate::byte_cursor::Buf;
use crate::encoder::{Encode, encode_unsigned_varint};
use crate::log_reader::{read_file, read_records};
use std::io;

pub struct Partition {
    partition_id: i32,
    error_code: i16,
    base_offset: i64,
    log_append_time: i64,
    log_start_offset: i64,
    records: Vec<u8>, // TODO: I think this is temporal
    error_message: Option<String>,
    // TODO: add TAG_BUFFER when encoding
}

pub struct Topic {
    topic_name: String, // NOTE: Remember this is a compact string
    partitions: Vec<Partition>,
    // TODO: add TAG_BUFFER when encoding
}

pub struct ProduceResponse {
    topics: Vec<Topic>,
    throttle_time: i32,
}

impl Encode for ProduceResponse {
    fn encode(&self, correlation_id: i32) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();
        encoded.extend_from_slice(&correlation_id.to_be_bytes());
        encoded.push(0u8);
        encoded
    }
}

pub fn build_produce_response(buf: &mut Buf) -> Result<ProduceResponse, io::Error> {
    let transaction_id = buf.read_compact_string();
    let required_acks = buf.read_i16();
    let timeout = buf.read_i32();
    let error_code: i16 = 0;

    let mut partitions: Vec<Partition> = Vec::new();

    let partition = Partition {
        partition_id: 0,
        error_code,
        base_offset: 0,
        log_append_time: 0,
        log_start_offset: 0,
        records: vec![],
        error_message: None,
    };
    partitions.push(partition);

    let mut topics: Vec<Topic> = Vec::new();
    let topic = Topic {
        topic_name: "some topic".to_string(),
        partitions,
    };
    topics.push(topic);

    Ok(ProduceResponse {
        topics,
        throttle_time: 0,
    })
}

pub fn produce_keys() -> ApiKeys {
    ApiKeys {
        api_key: 0,
        min_version: 0,
        max_version: 11,
    }
}
