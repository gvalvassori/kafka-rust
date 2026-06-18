use crate::byte_cursor::Buf;
use core::sync;
use std::fs;
use std::io;

pub struct TopicRecord {
    pub name: Option<String>,
    pub topic_id: [u8; 16],
}

pub struct PartitionRecord {
    pub partition_id: i32,
    pub topic_id: [u8; 16],
    pub replicas: Vec<i32>,
    pub isr: Vec<i32>,
    removing_replicas: Vec<i32>,
    pub adding_replicas: Vec<i32>,
    pub leader: i32,
    leader_recovery_state: i8,
    pub leader_epoch: i32,
    partition_epoch: i32,
    directories: Vec<[u8; 16]>,
    pub eligible_leader_replicas: Vec<i32>,
    pub last_known_elr: Vec<i32>,
}

pub struct ClusterMetadata {
    pub topics: Vec<TopicRecord>,
    pub partitions: Vec<PartitionRecord>,
}

pub fn read_file(path: &str) -> Result<Vec<u8>, io::Error> {
    let bytes = fs::read(path)?;
    eprintln!("logs bytes: {} total", bytes.len());
    eprintln!("hex: {:02X?}", &bytes[0..bytes.len().min(64)]);
    Ok(bytes)
}

pub fn read_records(bytes: Vec<u8>) -> ClusterMetadata {
    let mut buf = Buf::new(bytes);
    let mut topics: Vec<TopicRecord> = Vec::new();
    let mut partitions: Vec<PartitionRecord> = Vec::new();

    while buf.has_remaining() {
        let _base_offset = buf.read_i64();
        let _batch_length = buf.read_i32();
        let _partition_leader_epoch = buf.read_i32();
        let _magic = buf.read_i8();
        let _crc = buf.read_i32();
        let _attributes = buf.read_i16();
        let _last_offset_delta = buf.read_i32();
        let _base_timestamp = buf.read_i64();
        let _max_timestamp = buf.read_i64();
        let _producer_id = buf.read_i64();
        let _producer_epoch = buf.read_i16();
        let _base_sequence = buf.read_i32();
        let records_length = buf.read_i32(); // total number of records in this batch

        for _ in 0..records_length {
            let _length = buf.read_varint();
            let _attr = buf.read_u8();
            let _timestamp_delta = buf.read_varint();
            let _offset_delta = buf.read_varint();
            let key_length = buf.read_varint();
            // TODO: "key" field ignored for now, add if needed
            if key_length >= 0 {
                buf.skip(key_length as usize);
            };
            let value_length = buf.read_varint();

            let starting_pos = buf.pos();
            let _frame_version = buf.read_i8();
            let value_type = buf.read_i8();
            let _version = buf.read_i8();

            match value_type {
                2 => topics.push(TopicRecord {
                    name: buf.read_compact_string(),
                    topic_id: buf.read_uuid(),
                }),
                3 => {
                    let partition_id = buf.read_i32();
                    let topic_id = buf.read_uuid();
                    let len_replica_array = buf.read_compact_array_len();

                    let mut replica_array: Vec<i32> = Vec::new();
                    for _ in 0..len_replica_array {
                        replica_array.push(buf.read_i32());
                    }

                    let len_sync_replica_arr = buf.read_compact_array_len();
                    let mut sync_replica_arr: Vec<i32> = Vec::new();
                    for _ in 0..len_sync_replica_arr {
                        sync_replica_arr.push(buf.read_i32());
                    }
                    let _len_of_removing_replicas = buf.read_varint_unsigned();
                    let _len_of_adding_replicas = buf.read_varint_unsigned();
                    let leader = buf.read_i32();
                    let leader_epoch = buf.read_i32();
                    let partition_epoch = buf.read_i32();
                    let len_directories = buf.read_compact_array_len();
                    let mut directories: Vec<[u8; 16]> = Vec::new();
                    for _ in 0..len_directories {
                        directories.push(buf.read_uuid());
                    }

                    partitions.push(PartitionRecord {
                        partition_id,
                        topic_id,
                        replicas: replica_array,
                        isr: sync_replica_arr,
                        removing_replicas: vec![],
                        adding_replicas: vec![],
                        leader,
                        leader_recovery_state: 0,
                        leader_epoch,
                        partition_epoch,
                        directories,
                        eligible_leader_replicas: vec![],
                        last_known_elr: vec![],
                    })
                }
                _ => buf.skip(value_length as usize),
            }
            buf.set_pos(starting_pos + value_length as usize);
            let _headers_array_count = buf.read_varint();
        }
    }
    ClusterMetadata { topics, partitions }
}

impl ClusterMetadata {
    pub fn find_topic<'a>(&'a self, name: &str) -> Option<&'a TopicRecord> {
        self.topics.iter().find(|t| t.name.as_deref() == Some(name))
    }

    pub fn find_topic_by_id<'a>(&'a self, topic_id: [u8; 16]) -> Option<&'a TopicRecord> {
        self.topics.iter().find(|t| t.topic_id == topic_id)
    }

    pub fn find_partitions<'a>(&'a self, topic_id: [u8; 16]) -> Vec<&'a PartitionRecord> {
        self.partitions
            .iter()
            .filter(|p| p.topic_id == topic_id)
            .collect()
    }

    pub fn find_topic_and_partition<'a>(
        &'a self,
        topic_name: &str,
        partition_id: i32,
    ) -> Option<(&'a TopicRecord, &'a PartitionRecord)> {
        let topic = Self::find_topic(self, topic_name)?;
        let partition = self
            .partitions
            .iter()
            .find(|p| p.topic_id == topic.topic_id && p.partition_id == partition_id);

        Some((topic, partition?))
    }
}
