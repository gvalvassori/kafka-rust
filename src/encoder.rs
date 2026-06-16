pub trait Encode {
    fn encode(&self, correlation_id: i32) -> Vec<u8>;
}
