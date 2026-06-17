pub struct Buf {
    bytes: Vec<u8>,
    pos: usize,
}

impl Buf {
    pub fn new(bytes: Vec<u8>) -> Buf {
        Buf { bytes, pos: 0 }
    }

    // I can avoid repeating this by using generics, I will refactor once I
    // learn more about Rust.
    pub fn read_u8(&mut self) -> u8 {
        let data = self.bytes[self.pos];
        self.pos += 1;
        data
    }

    pub fn read_i16(&mut self) -> i16 {
        let data = &self.bytes[self.pos..self.pos + 2];
        let data_bytes_arr: [u8; 2] = data.try_into().unwrap();
        let data_i16 = i16::from_be_bytes(data_bytes_arr);
        self.pos += 2;
        data_i16
    }

    pub fn read_i8(&mut self) -> i8 {
        let data = &self.bytes[self.pos..self.pos + 1];
        let data_bytes_arr: [u8; 1] = data.try_into().unwrap();
        let data_i8 = i8::from_be_bytes(data_bytes_arr);
        self.pos += 1;
        data_i8
    }

    // pub fn read_u32(&mut self) -> u32 {
    //     let data = &self.bytes[self.pos..self.pos + 4];
    //     let data_bytes_arr: [u8; 4] = data.try_into().unwrap();
    //     let data_u32 = u32::from_be_bytes(data_bytes_arr);
    //     self.pos += 4;
    //     data_u32
    // }

    pub fn read_i32(&mut self) -> i32 {
        let data = &self.bytes[self.pos..self.pos + 4];
        let data_bytes_arr: [u8; 4] = data.try_into().unwrap();
        let data_i32 = i32::from_be_bytes(data_bytes_arr);
        self.pos += 4;
        data_i32
    }

    pub fn read_i64(&mut self) -> i64 {
        let data = &self.bytes[self.pos..self.pos + 8];
        let data_bytes_arr: [u8; 8] = data.try_into().unwrap();
        let data_i64 = i64::from_be_bytes(data_bytes_arr);
        self.pos += 8;
        data_i64
    }

    pub fn read_compact_string(&mut self) -> Option<String> {
        let size = self.read_varint_unsigned();

        if size == 0 {
            None
        } else if size == 1 {
            Some(String::new())
        } else {
            let n = (size - 1) as usize;
            let data = &self.bytes[self.pos..self.pos + n];
            self.pos += n;
            Some(String::from_utf8(data.to_vec()).unwrap())
        }
    }

    pub fn read_compact_array_len(&mut self) -> usize {
        let size = self.read_u8();
        if size == 0 {
            panic!("compact array of size {}", size)
        }
        (size - 1) as usize
    }

    pub fn skip(&mut self, n: usize) {
        // this is to skip client_id and TAG_BUFFER that are not used for now
        self.pos += n;
    }

    pub fn read_varint_unsigned(&mut self) -> u64 {
        let mut result: u64 = 0;
        let mut shift: u32 = 0;

        loop {
            let byte = self.read_u8();
            result |= ((byte & 0x7F) as u64) << shift;
            if (byte & 0x80) == 0 {
                break;
            }
            shift += 7
        }
        result
    }

    pub fn read_varint(&mut self) -> i64 {
        let result: i64;
        let varint = self.read_varint_unsigned();
        if varint % 2 == 0 {
            result = (varint / 2) as i64;
        } else {
            result = -((varint + 1) as i64) / 2;
        }
        result
    }

    pub fn read_uuid(&mut self) -> [u8; 16] {
        let data = &self.bytes[self.pos..self.pos + 16];
        let uuid: [u8; 16] = data.try_into().unwrap();
        self.pos += 16;
        uuid
    }

    pub fn has_remaining(&self) -> bool {
        self.pos < self.bytes.len()
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }
}
