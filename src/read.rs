use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub trait MakeReader {
    type Reader: Read;
    fn make_reader(self) -> io::Result<Self::Reader>;
}

impl MakeReader for &Path {
    type Reader = File;

    fn make_reader(self) -> io::Result<Self::Reader> {
        File::open(self)
    }
}

pub struct CatRead<T>
where
    T: Iterator,
    T::Item: MakeReader,
{
    sources: T,
    current: <<T as Iterator>::Item as MakeReader>::Reader,
}

impl<T> CatRead<T>
where
    T: Iterator,
    T::Item: MakeReader,
{
    pub fn new(initial: <T::Item as MakeReader>::Reader, sources: T) -> Self {
        Self {
            sources,
            current: initial,
        }
    }
}

impl<T> Read for CatRead<T>
where
    T: Iterator,
    T::Item: MakeReader,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut read = 0;
        loop {
            match self.current.read(&mut buf[read..]) {
                // The current reader is exhausted
                Ok(0) => match self.sources.next() {
                    Some(source) => self.current = source.make_reader()?,
                    None => return Ok(read),
                },

                // The current reader provided at least some data
                Ok(len) => {
                    read += len;
                    if read == buf.len() {
                        return Ok(read);
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => (),
                Err(e) => return Err(e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Cursor, Read},
        iter,
    };

    use super::{CatRead, MakeReader};

    impl<'a> MakeReader for Cursor<&'a str> {
        type Reader = Self;

        fn make_reader(self) -> std::io::Result<Self::Reader> {
            Ok(self)
        }
    }

    #[test]
    fn concatenated_read_works() {
        let left = Cursor::new("Hello, ");
        let right = Cursor::new("world!");
        let mut concat = CatRead::new(left, iter::once(right));
        let mut buf = String::new();

        concat.read_to_string(&mut buf).unwrap();

        assert_eq!(buf, "Hello, world!");
    }
}
