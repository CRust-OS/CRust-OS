// TODO: Break up Ring Trait in WriteableRing and ReadableRing
pub trait Ring {
    fn input_buffer(&mut self) -> &mut [u8];
    fn output_buffer(&mut self) -> &mut [u8];

    fn get_in_cons(&self) -> usize; 
    fn get_out_cons(&self) -> usize;
    fn get_in_prod(&self) -> usize;
    fn get_out_prod(&self) -> usize;

    fn set_in_cons(&mut self, usize);
    fn set_out_cons(&mut self, usize);
    fn set_in_prod(&mut self, usize);
    fn set_out_prod(&mut self, usize);

    fn input_mask(&mut self) -> usize {
        let len = self.input_buffer().len();
        return len - 1;
    }

    fn output_mask(&mut self) -> usize {
        let len = self.output_buffer().len();
        return len - 1;
    }

    fn write(&mut self) {
    }
    fn read(&mut self) {
    }
}
