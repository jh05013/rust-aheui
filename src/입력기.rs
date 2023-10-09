// kiwiyou님 제공

use std::io::Read;

#[derive(Clone)]
pub struct InputReader<R> {
    buffer: Vec<u8>,
    offset: usize,
    end: usize,
    stream: R,
}

impl<R: Read> InputReader<R> {
    pub fn with_capacity(capacity: usize, stream: R) -> Self {
        Self {
            buffer: vec![0; capacity],
            offset: capacity,
            end: capacity,
            stream,
        }
    }
    fn skip_whitespace(&mut self) {
        while self.end > 0 {
            if let Some(i) = self.remain().iter().position(|&b| b > b' ') {
                self.offset += i;
                break;
            }
            self.fill();
        }
    }
    fn remain(&self) -> &[u8] {
        &self.buffer[self.offset..self.end]
    }
    fn fill(&mut self) {
        let len = self.stream.read(&mut self.buffer).unwrap();
        self.end = len;
        self.offset = 0;
    }
    pub fn read_char(&mut self) -> Option<char> {
        if self.remain().is_empty() {
            self.fill();
        }
        let first = *self.remain().get(0)?;
        self.offset += 1;
        match first.leading_ones() {
            0 => Some(first as char),
            len @ 2..=4 => {
                let mut code = (first & ((1 << (8 - len)) - 1)) as u32;
                let mut count = 1;
                while self.end > 0 {
                    for (i, &b) in self.remain().iter().enumerate() {
                        if count >= len || b & 0b1000_0000 == 0 {
                            self.offset += i;
                            return None;
                        }
                        code <<= 6;
                        code |= (b & 0b0011_1111) as u32;
                        count += 1;
                    }
                    self.fill();
                }
                char::from_u32(code)
            }
            _ => None,
        }
    }
    pub fn read_integer(&mut self) -> Option<i64> {
        self.skip_whitespace();
        let mut integer = 0;
        let mut is_read_sign = false;
        let mut is_read_number = false;
        let mut neg = false;
        'consume_buffer: while self.end > 0 {
            for (i, &b) in self.remain().iter().enumerate() {
                if !is_read_sign && b == b'-' {
                    neg = true;
                    is_read_sign = true;
                } else if (b'0'..=b'9').contains(&b) {
                    is_read_sign = true;
                    is_read_number = true;
                    integer = integer * 10 + (b - b'0') as u64;
                } else {
                    self.offset += i;
                    break 'consume_buffer;
                }
            }
            self.fill();
        }
        if neg {
            integer = integer.wrapping_neg();
        }
        is_read_number.then_some(integer as i64)
    }
}
