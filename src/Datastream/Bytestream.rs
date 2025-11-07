use std::str;

pub struct ByteStream {
    pub buffer: Vec<u8>,
    pub offset: usize,
    pub bitOffset: u8,
}

impl ByteStream {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            offset: 0,
            bitOffset: 0,
        }
    }

    pub fn ensureCapacity(&mut self, capacity: usize) {
        let bufferLength = self.buffer.len();
        if self.offset + capacity > bufferLength {
            let mut newBuffer = vec![0; bufferLength + capacity];
            newBuffer[..bufferLength].copy_from_slice(&self.buffer);
            self.buffer = newBuffer;
        }
    }

    pub fn readInt(&mut self) -> i32 {
        self.bitOffset = 0;
        let val = ((self.buffer[self.offset] as i32) << 24)
            | ((self.buffer[self.offset + 1] as i32) << 16)
            | ((self.buffer[self.offset + 2] as i32) << 8)
            | (self.buffer[self.offset + 3] as i32);
        self.offset += 4;
        val
    }

    pub fn skip(&mut self, len: usize) {
        self.bitOffset += len as u8;
    }

    pub fn readShort(&mut self) -> i32 {
        self.bitOffset = 0;
        let val = ((self.buffer[self.offset] as i32) << 8)
            | (self.buffer[self.offset + 1] as i32);
        self.offset += 2;
        val
    }

    pub fn writeShort(&mut self, value: i32) {
        self.bitOffset = 0;
        self.ensureCapacity(2);
        self.buffer[self.offset] = ((value >> 8) & 0xFF) as u8;
        self.buffer[self.offset + 1] = (value & 0xFF) as u8;
        self.offset += 2;
    }

    pub fn writeInt(&mut self, value: i32) {
        self.bitOffset = 0;
        self.ensureCapacity(4);
        self.buffer[self.offset] = ((value >> 24) & 0xFF) as u8;
        self.buffer[self.offset + 1] = ((value >> 16) & 0xFF) as u8;
        self.buffer[self.offset + 2] = ((value >> 8) & 0xFF) as u8;
        self.buffer[self.offset + 3] = (value & 0xFF) as u8;
        self.offset += 4;
    }

    pub fn writeIntZero(&mut self) {
        self.writeInt(0);
    }

    pub fn writeString(&mut self, strrrrng: &str) {
        let value: Option<&str> = Some(strrrrng);

        if value.is_none() || value.unwrap().len() > 90000 {
            self.writeInt(-1);
            return;
        }

        let str_bytes = value.unwrap().as_bytes();
        self.writeInt(str_bytes.len() as i32);
        self.ensureCapacity(str_bytes.len());
        for b in str_bytes {
            self.buffer[self.offset] = *b;
            self.offset += 1;
        }
    }

    pub fn writeStringEmpty(&mut self) {
        self.writeString("");
    }

    pub fn readString(&mut self) -> String {
        let length = self.readInt();
        if length > 0 && length < 90000 {
            if self.offset + length as usize > self.buffer.len() {
                return "".to_string();
            }
            let s = str::from_utf8(&self.buffer[self.offset..self.offset + length as usize])
                .unwrap_or("")
                .to_string();
            self.offset += length as usize;
            s
        } else {
            "".to_string()
        }
    }

    pub fn readDataReference(&mut self) -> [i32; 2] {
        let a1 = self.readVInt();
        [a1, if a1 == 0 { 0 } else { self.readVInt() }]
    }

    pub fn writeDataReference(&mut self, value1: i32, value2: i32) {
        if value1 < 1 {
            self.writeVInt(0);
        } else {
            self.writeVInt(value1);
            self.writeVInt(value2);
        }
    }

    pub fn writeVInt(&mut self, mut value: i32) {
        self.bitOffset = 0;
        let mut temp = ((value >> 25) & 0x40) as u8;
        let mut flipped = value ^ (value >> 31);
        temp |= (value & 0x3F) as u8;
        value >>= 6;
        flipped >>= 6;

        if flipped == 0 {
            self.writeByte(temp);
            return;
        }

        self.writeByte(temp | 0x80);
        flipped >>= 7;
        let mut r = if flipped != 0 { 0x80 } else { 0 };
        self.writeByte(((value & 0x7F) as u8) | r);
        value >>= 7;

        while flipped != 0 {
            flipped >>= 7;
            r = if flipped != 0 { 0x80 } else { 0 };
            self.writeByte(((value & 0x7F) as u8) | r);
            value >>= 7;
        }
    }

    pub fn writeVIntZero(&mut self) {
        self.writeVInt(0);
    }

    pub fn readVInt(&mut self) -> i32 {
        let mut result = 0;
        let mut shift = 0;

        loop {
            let mut b = self.buffer[self.offset] as i32;
            self.offset += 1;

            if shift == 0 {
                let a1 = (b & 0x40) >> 6;
                let a2 = (b & 0x80) >> 7;
                let s = (b << 1) & !0x181;
                b = s | (a2 << 7) | a1;
            }

            result |= (b & 0x7F) << shift;
            shift += 7;

            if (b & 0x80) == 0 {
                break;
            }
        }

        (result >> 1) ^ (-(result & 1))
    }

    pub fn writeBoolean(&mut self, value: bool) {
        if self.bitOffset == 0 {
            self.ensureCapacity(1);
            self.buffer[self.offset] = 0;
            self.offset += 1;
        }
        if value {
            let idx = self.offset - 1;
            self.buffer[idx] |= 1 << self.bitOffset;
        }
        self.bitOffset = (self.bitOffset + 1) & 7;
    }

    pub fn readBoolean(&mut self) -> bool {
        self.readVInt() >= 1
    }

    pub fn writeHex(&mut self, hex: Option<&str>) {
        self.bitOffset = 0;
        if let Some(mut data) = hex {
            if data.starts_with("0x") {
                data = &data[2..];
            }
            let cleaned_data = data.replace(&[' ', '-'][..], "");
            self.ensureCapacity(cleaned_data.len() / 2);
            let mut i = 0;
            while i < cleaned_data.len() {
                let byte = u8::from_str_radix(&cleaned_data[i..i + 2], 16).unwrap();
                self.writeByte(byte);
                i += 2;
            }
        }
    }

    pub fn writeStringReference(&mut self, value: &str) {
        self.writeString(value);
    }

    pub fn writeStringReferenceEmpty(&mut self) {
        self.writeString("");
    }

    pub fn writeLongLong(&mut self, value: i64) {
        self.writeInt((value >> 32) as i32);
        self.writeInt(value as i32);
    }

    pub fn writeLogicLong(&mut self, value1: i32, value2: i32) {
        self.writeVInt(value1);
        self.writeVInt(value2);
    }

    pub fn readLogicLong(&mut self) -> [i32; 2] {
        [self.readVInt(), self.readVInt()]
    }

    pub fn writeLong(&mut self, value1: i32, value2: i32) {
        self.writeInt(value1);
        self.writeInt(value2);
    }

    pub fn readLong(&mut self) -> [i32; 2] {
        [self.readInt(), self.readInt()]
    }

    pub fn writeByte(&mut self, value: u8) {
        self.bitOffset = 0;
        self.ensureCapacity(1);
        self.buffer[self.offset] = value;
        self.offset += 1;
    }

    pub fn writeBytes(&mut self, bytes: Option<&[u8]>) {
        match bytes {
            Some(b) => {
                self.writeInt(b.len() as i32);
                self.ensureCapacity(b.len());
                for byte in b {
                    self.buffer[self.offset] = *byte;
                    self.offset += 1;
                }
            }
            None => self.writeInt(-1),
        }
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.offset = 0;
        self.bitOffset = 0;
    }

    pub fn getLength(&self) -> usize {
        self.buffer.len()
    }

    pub fn getBuffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn replaceBuffer(&mut self, b: Vec<u8>) -> &Vec<u8> {
        self.offset = 0;
        self.buffer = b;
        &self.buffer
    }
}
