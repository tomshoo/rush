use std::{
    fs::File,
    io::Lines,
    io::{BufRead, BufReader},
};

pub struct FileBuffer<'f> {
    buflr: Lines<BufReader<File>>,
    source: std::str::Chars<'f>,
}

impl<'f> FileBuffer<'f> {
    pub fn new(fd: File) -> std::io::Result<Self> {
        let mut buflr = BufReader::new(fd).lines();

        let strbuf: &'f _ = match buflr.next() {
            Some(line) => Box::leak(line?.into_boxed_str()),
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "File is empty",
                ))
            },
        };

        Ok(Self {
            buflr,
            source: strbuf.chars(),
        })
    }
}

impl<'f> Iterator for FileBuffer<'f> {
    type Item = std::io::Result<char>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next() {
            Some(ch) => Some(Ok(ch)),
            None => {
                let stream = self.buflr.next()?;
                match stream {
                    Ok(strbuf) => {
                        let buf: &'f _ = Box::leak(strbuf.into_boxed_str());
                        self.source = buf.chars();
                        self.source.next().map(Ok)
                    },
                    Err(err) => Some(Err(err)),
                }
            },
        }
    }
}
