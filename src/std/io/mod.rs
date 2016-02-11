pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, &'static str>;
    fn flush(&mut self) -> Result<(), &'static str> {
        Result::Ok(())
    }
}

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, &'static str>;
}
