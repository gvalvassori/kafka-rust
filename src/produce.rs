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
        // Header TAG_BUFFER
        encoded.push(0u8);

        // Topics array
        let topics_len = self.topics.len();
        encoded.push((topics_len + 1) as u8);
        for topic in &self.topics {
            encoded.push((topic.topic_name.len() + 1) as u8);
            encoded.extend_from_slice(topic.topic_name.as_bytes());

            // Partitions array
            let partitions_len = topic.partitions.len();
            encoded.push((partitions_len + 1) as u8);
            for partition in &topic.partitions {
                encoded.extend_from_slice(&partition.partition_id.to_be_bytes());
                encoded.extend_from_slice(&partition.error_code.to_be_bytes());
                encoded.extend_from_slice(&partition.base_offset.to_be_bytes());
                encoded.extend_from_slice(&partition.log_append_time.to_be_bytes());
                encoded.extend_from_slice(&partition.log_start_offset.to_be_bytes());

                // Records array
                let records_len = partition.records.len();
                encoded.push((records_len + 1) as u8);
                encoded.extend_from_slice(&partition.records);

                // Error message
                match &partition.error_message {
                    None => encoded.push(0),
                    Some(n) => {
                        encoded.push((n.len() + 1) as u8);
                        encoded.extend_from_slice(n.as_bytes());
                    }
                }

                // Partition response TAG_BUFFER
                encoded.push(0u8);
            }

            // Topic response TAG_BUFFER
            encoded.push(0u8);
        }

        // Throttle time
        encoded.extend_from_slice(&self.throttle_time.to_be_bytes());

        // Root TAG_BUFFER
        encoded.push(0u8);
        encoded
    }
}

pub fn build_produce_response(buf: &mut Buf) -> Result<ProduceResponse, io::Error> {
    let bytes =
        read_file("/tmp/kraft-combined-logs/__cluster_metadata-0/00000000000000000000.log")?;
    let log_records = read_records(bytes);
    let transaction_id = buf.read_compact_string();
    let required_acks = buf.read_i16();
    let timeout = buf.read_i32();
    let error_code: i16 = 0;

    let mut topics: Vec<Topic> = Vec::new();
    let topics_len = buf.read_compact_array_len();
    for _ in 0..topics_len {
        let topic_name = buf.read_compact_string();
        if let Some(name) = topic_name.as_deref() {
            let mut partitions: Vec<Partition> = Vec::new();
            let partitions_len = buf.read_compact_array_len();
            for _ in 0..partitions_len {
                let partition_id = buf.read_i32();

                let partition = match log_records.find_topic_and_partition(name, partition_id) {
                    Some((t, p)) => Partition {
                        partition_id,
                        error_code: 0,
                        base_offset: 0,
                        log_append_time: 0,
                        log_start_offset: 0,
                        records: vec![],
                        error_message: None,
                    },
                    _ => Partition {
                        partition_id,
                        error_code: 3,
                        base_offset: -1,
                        log_append_time: -1,
                        log_start_offset: -1,
                        records: vec![],
                        error_message: None,
                    },
                };
                partitions.push(partition);
            }
            let topic = Topic {
                topic_name: name.to_string(),
                partitions,
            };
            // TODO: read the whole record batch array
            topics.push(topic);
        }
    }

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
