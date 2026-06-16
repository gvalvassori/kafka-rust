use crate::api_versions::ApiKeys;
use crate::byte_cursor::Buf;
use crate::encoder::Encode;

pub struct Partition {
    error_code: i16,
    partition_index: i32,
    leader_id: i32,
    leader_epoch: i32,
    replica_nodes: Vec<i32>,
    isr_nodes: Vec<i32>,
    eligible_leader_replicas: Vec<i32>,
    last_known_elr: Vec<i32>,
    offline_replicas: Vec<i32>,
}

impl Partition {
    fn to_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend_from_slice(&self.error_code.to_be_bytes());
        out.extend_from_slice(&self.partition_index.to_be_bytes());
        out.extend_from_slice(&self.leader_id.to_be_bytes());
        out.extend_from_slice(&self.leader_epoch.to_be_bytes());
        out.extend_from_slice(&self.vec_to_bytes(self.replica_nodes.clone()));
        out.extend_from_slice(&self.vec_to_bytes(self.isr_nodes.clone()));
        out.extend_from_slice(&self.vec_to_bytes(self.eligible_leader_replicas.clone()));
        out.extend_from_slice(&self.vec_to_bytes(self.last_known_elr.clone()));
        out.extend_from_slice(&self.vec_to_bytes(self.offline_replicas.clone()));
        out
    }

    fn vec_to_bytes(&self, vector: Vec<i32>) -> Vec<u8> {
        let bytes: Vec<u8> = vector.iter().flat_map(|&x| x.to_be_bytes()).collect();
        bytes
    }
}

pub struct Topic {
    error_code: i16,
    name: Option<String>,
    topic_id: [u8; 16],
    is_internal: bool,
    partitions: Vec<Partition>,
    topic_authorized_operations: i32,
}

impl Topic {
    fn to_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend_from_slice(&self.error_code.to_be_bytes());

        match &self.name {
            None => out.push(0),
            Some(s) => {
                // Here I handle the compact string response
                out.push((s.len() + 1) as u8);
                out.extend_from_slice(s.as_bytes());
            }
        }

        out.extend_from_slice(&self.topic_id); // TODO: convert to be bytes once topic_id is a real
        // UUID, right now it is an array of bytes
        out.push(self.is_internal as u8);

        let partitions_len = self.partitions.len();
        out.push((partitions_len + 1) as u8);
        for partition in &self.partitions {
            out.extend_from_slice(&partition.to_bytes());
        }
        out.extend_from_slice(&self.topic_authorized_operations.to_be_bytes());
        out.push(0u8);
        out
    }
}

pub struct DescribeTopicPartitionsResponse {
    throttle_time_ms: i32,
    topics: Vec<Topic>,
    next_cursor: i8, // next_cursor: NextCursor not implemented yet but should contain topic_name COMPACT STRING and
                     // partition_index i32
}

pub fn describe_topic_partitions_keys() -> ApiKeys {
    ApiKeys {
        api_key: 75,
        min_version: 0,
        max_version: 0,
    }
}

pub fn build_describe_response(buf: &mut Buf) -> DescribeTopicPartitionsResponse {
    let topics_len = buf.read_compact_array_len();
    let mut topics_arr: Vec<Topic> = Vec::new();

    for _ in 0..topics_len {
        let partitions = vec![]; // This is an empty partitions for now
        let topic_name = buf.read_compact_string();
        buf.skip(1);

        // Topic hardcoded for now
        let topic = Topic {
            error_code: 3,
            name: topic_name,
            topic_id: [0u8; 16],
            is_internal: false,
            partitions,
            topic_authorized_operations: 0,
        };

        topics_arr.push(topic);
    }

    DescribeTopicPartitionsResponse {
        throttle_time_ms: 0,
        topics: topics_arr,
        next_cursor: -1,
    }
}

impl Encode for DescribeTopicPartitionsResponse {
    fn encode(&self, correlation_id: i32) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();

        encoded.extend_from_slice(&correlation_id.to_be_bytes());
        encoded.push(0u8);
        encoded.extend_from_slice(&self.throttle_time_ms.to_be_bytes());
        let topics_len = self.topics.len();
        encoded.push((topics_len + 1) as u8); // This is the len of the compact
        // array topics

        for topic in &self.topics {
            encoded.extend_from_slice(&topic.to_bytes());
        }
        encoded.extend_from_slice(&self.next_cursor.to_be_bytes());
        encoded.push(0u8);
        encoded
    }
}
