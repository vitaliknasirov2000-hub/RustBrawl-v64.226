use std::str;

pub struct ByteStream {
    pub Buffer: Vec<u8>,
    pub Offset: usize,
    pub BitOffset: u8,
}

impl ByteStream {
    pub fn new() -> Self {
        Self {
            Buffer: Vec::new(),
            Offset: 0,
            BitOffset: 0,
        }
    }

    pub fn ensureCapacity(&mut self, capacity: usize) {
        if self.Offset + capacity > self.Buffer.len() {
            self.Buffer.resize(self.Buffer.len() + capacity, 0);
        }
    }

    pub fn readInt(&mut self) -> i32 {
        self.BitOffset = 0;
        let val = ((self.Buffer[self.Offset] as i32) << 24)
            | ((self.Buffer[self.Offset + 1] as i32) << 16)
            | ((self.Buffer[self.Offset + 2] as i32) << 8)
            | (self.Buffer[self.Offset + 3] as i32);
        self.Offset += 4;
        val
    }

    pub fn skip(&mut self, len: usize) {
        self.BitOffset += len as u8;
    }

    pub fn readShort(&mut self) -> i16 {
        self.BitOffset = 0;
        let val = ((self.Buffer[self.Offset] as i16) << 8)
            | (self.Buffer[self.Offset + 1] as i16);
        self.Offset += 2;
        val
    }

    pub fn writeShort(&mut self, value: i16) {
        self.BitOffset = 0;
        self.ensureCapacity(2);
        self.Buffer.push((value >> 8) as u8);
        self.Buffer.push((value & 0xFF) as u8);
        self.Offset += 2;
    }

    pub fn writeInt(&mut self, value: i32) {
        self.BitOffset = 0;
        self.ensureCapacity(4);
        self.Buffer.push((value >> 24) as u8);
        self.Buffer.push(((value >> 16) & 0xFF) as u8);
        self.Buffer.push(((value >> 8) & 0xFF) as u8);
        self.Buffer.push((value & 0xFF) as u8);
        self.Offset += 4;
    }

    pub fn writeIntZero(&mut self) {
        self.writeInt(0);
    }

    pub fn writeString(&mut self, value: Option<&str>) {
        if value.is_none() || value.unwrap().len() > 90000 {
            self.writeInt(-1);
            return;
        }

        let str_bytes = value.unwrap().as_bytes();
        self.writeInt(str_bytes.len() as i32);
        self.Buffer.extend_from_slice(str_bytes);
        self.Offset += str_bytes.len();
    }

    pub fn writeStringEmpty(&mut self) {
        self.writeString(None);
    }

    pub fn readString(&mut self) -> String {
        let length = self.readInt();
        if length > 0 && length < 90000 {
            if self.Offset + length as usize > self.Buffer.len() {
                return "".to_string();
            }
            let s = str::from_utf8(&self.Buffer[self.Offset..self.Offset + length as usize])
                .unwrap_or("")
                .to_string();
            self.Offset += length as usize;
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
        self.BitOffset = 0;
        let mut temp = ((value >> 25) & 0x40) as u8;
        let mut flipped = (value ^ (value >> 31)) as i32;
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
            let mut b = self.Buffer[self.Offset] as i32;
            self.Offset += 1;
            let (mut a1, mut a2, mut s);

            if shift == 0 {
                a1 = (b & 0x40) >> 6;
                a2 = (b & 0x80) >> 7;
                s = (b << 1) & !0x181;
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
        if self.BitOffset == 0 {
            self.ensureCapacity(1);
            self.Buffer.push(0);
            self.Offset += 1;
        }
        if value {
            let idx = self.Offset - 1;
            self.Buffer[idx] |= 1 << self.BitOffset;
        }
        self.BitOffset = (self.BitOffset + 1) & 7;
    }

    pub fn readBoolean(&mut self) -> bool {
        self.readVInt() >= 1
    }

    pub fn writeHex(&mut self, hex: Option<&str>) { // chatgpt
        self.BitOffset = 0;
        if let Some(mut data) = hex {
            if data.starts_with("0x") {
                data = &data[2..];
            }

            let cleaned_data = data.replace(&[' ', '-'][..], "");

            self.ensureCapacity(cleaned_data.len() / 2);

            for i in (0..cleaned_data.len()).step_by(2) {
                let byte = u8::from_str_radix(&cleaned_data[i..i + 2], 16).unwrap();
                self.writeByte(byte);
            }
        }
    }

    pub fn writeStringReference(&mut self, value: Option<&str>) {
        self.writeString(value);
    }

    pub fn writeStringReferenceEmpty(&mut self) {
        self.writeString(None);
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
        self.BitOffset = 0;
        self.ensureCapacity(1);
        self.Buffer.push(value);
        self.Offset += 1;
    }

    pub fn writeBytes(&mut self, bytes: Option<&[u8]>) {
        match bytes {
            Some(b) => {
                self.writeInt(b.len() as i32);
                self.Buffer.extend_from_slice(b);
                self.Offset += b.len();
            }
            None => self.writeInt(-1),
        }
    }

    pub fn reset(&mut self) {
        self.Buffer.clear();
        self.Offset = 0;
    }

    pub fn getLength(&self) -> usize {
        self.Buffer.len()
    }

    pub fn getBuffer(&self) -> &[u8] {
        &self.Buffer
    }

    pub fn replaceBuffer(&mut self, b: Vec<u8>) -> &Vec<u8> {
        self.Offset = 0;
        self.Buffer = b;
        &self.Buffer
    }
}
