use crate::api_versions::ApiKeys;
use crate::byte_cursor::Buf;
use crate::encoder::Encode;
use std::io;

pub struct AbortedTransaction {
    producer_id: i64,
    first_offset: i64,
}

pub struct Partition {
    partition_index: i32,
    error_code: i16,
    high_watermark: i64,
    last_stable_offset: i64,
    log_start_offset: i64,
    aborted_transactions: Vec<AbortedTransaction>,
    preferred_read_replica: i32,
    records: Option<Vec<u8>>, // what is this?
                              // TODO: we need to define some struct for the TAG_BUFFER, not used now
}

pub struct Response {
    topic_id: [u8; 16],
    partitions: Vec<Partition>,
}

pub struct FetchResponse {
    throttle_time_ms: i32,
    error_code: i16,
    session_id: i32,
    responses: Vec<Response>,
}

impl Encode for FetchResponse {
    fn encode(&self, correlation_id: i32) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();
        encoded.extend_from_slice(&correlation_id.to_be_bytes());
        encoded.push(0u8);
        encoded.extend_from_slice(&self.throttle_time_ms.to_be_bytes());
        encoded.extend_from_slice(&self.error_code.to_be_bytes());
        encoded.extend_from_slice(&self.session_id.to_be_bytes());
        let responses_len = self.responses.len();
        encoded.push((responses_len + 1) as u8); // This is the len of the compact
        // NOTE: Iterate over FetchResponse.responses
        for response in &self.responses {
            encoded.extend_from_slice(&response.topic_id);
            let partitions_len = response.partitions.len();
            encoded.push((partitions_len + 1) as u8);
            // NOTE: Iterate over Response.partitions
            for partition in &response.partitions {
                encoded.extend_from_slice(&partition.partition_index.to_be_bytes());
                encoded.extend_from_slice(&partition.error_code.to_be_bytes());
                encoded.extend_from_slice(&partition.high_watermark.to_be_bytes());
                encoded.extend_from_slice(&partition.last_stable_offset.to_be_bytes());
                encoded.extend_from_slice(&partition.log_start_offset.to_be_bytes());
                let aborted_transac_len = partition.aborted_transactions.len();
                encoded.push((aborted_transac_len + 1) as u8);
                // NOTE: Iterate over Partition.aborted_transactions
                for at in &partition.aborted_transactions {
                    encoded.extend_from_slice(&at.producer_id.to_be_bytes());
                    encoded.extend_from_slice(&at.first_offset.to_be_bytes());
                }
                encoded.extend_from_slice(&partition.preferred_read_replica.to_be_bytes());
                encoded.extend_from_slice(&partition.partition_index.to_be_bytes());
                // NOTE: Since Partition.records can be Option we match to hanlde this
                match &partition.records {
                    None => encoded.push(0),
                    Some(bytes) => {
                        encoded.push((bytes.len() + 1) as u8);
                        encoded.extend_from_slice(bytes);
                    }
                }
            }
        }
        encoded.push(0u8); // TODO: This is the TAG_BUFFER, I may need to remove it later
        encoded
    }
}

pub fn build_fetch_response(buf: &mut Buf) -> Result<FetchResponse, io::Error> {
    let throttle_time_ms: i32 = 0; // TODO: placeholder
    let error_code: i16 = 0; // TODO: this is a placeholder for now, replace
    buf.skip(13); // TODO: remove. For now we skip everything till session_id in the request
    let session_id = buf.read_i32();
    buf.skip(4); // TODO: Skip session_epoch, not needed now
    let topics_len = buf.read_compact_array_len();
    let mut responses: Vec<Response> = Vec::new();
    for _ in 0..topics_len {
        let topic_id = buf.read_uuid();
        let partitions_len = buf.read_compact_array_len();
        let mut partitions: Vec<Partition> = Vec::new();
        for _ in 0..partitions_len {
            let partition = buf.read_i32();
            buf.skip(28); // TODO: skip 28 bytes for things we dont need right now
            partitions.push(Partition {
                partition_index: partition,
                error_code: 100,
                high_watermark: 0,
                last_stable_offset: 0,
                log_start_offset: 0,
                aborted_transactions: vec![],
                preferred_read_replica: 0,
                records: None,
            })
        }
        responses.push(Response {
            topic_id,
            partitions,
        })
    }

    Ok(FetchResponse {
        throttle_time_ms,
        error_code,
        session_id,
        responses,
    })
}

pub fn fetch_keys() -> ApiKeys {
    ApiKeys {
        api_key: 1,
        min_version: 0,
        max_version: 16,
    }
}
