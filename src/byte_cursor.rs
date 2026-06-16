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

    pub fn read_compact_string(&mut self) -> Option<String> {
        let size = self.read_u8();

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
        (size - 1) as usize
    }

    pub fn skip(&mut self, n: usize) {
        // this is to skip client_id and TAG_BUFFER that are not used for now
        self.pos += n;
    }
}
