pub mod error;

use crate::error::Error;
use std::io::{BufRead, Read};

const UTF8_MAX_SIZE: usize = 6;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct ReadChars<R: Read> {
    reader: R,
    bytes_read: usize,
    buffer: [u8; UTF8_MAX_SIZE],
}

#[allow(dead_code)]
impl<R: Read> ReadChars<R> {
    pub fn bytes_read(&self) -> usize {
        self.bytes_read
    }

    fn clear_buffer(&mut self) {
        self.buffer.iter_mut().for_each(|i| *i = 0)
    }
}

impl<R: Read> From<R> for ReadChars<R> {
    fn from(reader: R) -> Self {
        Self {
            reader,
            bytes_read: 0,
            buffer: [0; UTF8_MAX_SIZE],
        }
    }
}

impl<R: Read> ReadChars<R> {
    fn next_char(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut buffer = [0u8; UTF8_MAX_SIZE];
        let mut offset = 0;

        loop {
            if offset >= UTF8_MAX_SIZE {
                return Some(Err(Error::InvalidUtf8));
            }

            match self.reader.read(&mut buffer[offset..=offset]) {
                Ok(0) => return None,
                Ok(_) => self.bytes_read += 1,
                Err(e) => return Some(Err(e.into())),
            };

            match std::str::from_utf8(&buffer[..=offset]) {
                Ok(s) => return Some(Ok(s.chars().next().unwrap())),
                Err(_) => offset += 1,
            }
        }
    }
}

impl<R: Read> Iterator for ReadChars<R> {
    type Item = Result<char>;

    fn next(&mut self) -> Option<Self::Item> {
        self.clear_buffer();
        self.next_char()
    }
}

impl<R: Read> Read for ReadChars<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.bytes_read = buf.len();
        self.reader.read(buf)
    }
}

impl<R: BufRead> BufRead for ReadChars<R> {
    fn consume(&mut self, amt: usize) {
        self.bytes_read = amt;
        self.reader.consume(amt)
    }

    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.reader.fill_buf()
    }
}
