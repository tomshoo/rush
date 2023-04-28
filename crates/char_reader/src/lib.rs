pub mod error;

use crate::error::Error;
use std::io::Read;

const UTF8_MAX_SIZE: usize = 6;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct ReadChars<R>
where
    R: Read,
{
    reader: R,
    bytes_read: usize,
    buffer: [u8; UTF8_MAX_SIZE],
}

#[allow(dead_code)]
impl<R> ReadChars<R>
where
    R: Read,
{
    pub fn bytes_read(&self) -> usize {
        self.bytes_read
    }

    fn clear_buffer(&mut self) {
        self.buffer.iter_mut().for_each(|i| *i = 0)
    }
}

impl<R> From<R> for ReadChars<R>
where
    R: Read,
{
    fn from(reader: R) -> Self {
        Self {
            reader,
            bytes_read: 0,
            buffer: [0; UTF8_MAX_SIZE],
        }
    }
}

impl<R> ReadChars<R>
where
    R: Read,
{
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

impl<R> Iterator for ReadChars<R>
where
    R: Read,
{
    type Item = Result<char>;

    fn next(&mut self) -> Option<Self::Item> {
        self.clear_buffer();
        self.next_char()
    }
}
