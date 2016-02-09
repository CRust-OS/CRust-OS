use ::xen::arch::mem::*;
use ::xen::event_channels::send;

pub trait ReadableRing {
    fn read(&mut self) {
        // TODO
    }
    fn input_buffer(&mut self) -> &mut [u8];
    fn get_input_consumer_idx(&self) -> usize; 
    fn get_input_producer_idx(&self) -> usize;
    fn set_input_consumer_idx(&mut self, usize);
    fn set_input_producer_idx(&mut self, usize);
    fn input_mask(&mut self) -> usize {
        let len = self.input_buffer().len();
        return len - 1;
    }
}
pub trait WritableRing {
    fn output_buffer(&mut self) -> &mut [u8];

    fn get_output_consumer_idx(&self) -> usize;
    fn get_output_producer_idx(&self) -> usize;
    fn set_output_consumer_idx(&mut self, usize);
    fn set_output_producer_idx(&mut self, usize);


    fn output_mask(&mut self) -> usize {
        let len = self.output_buffer().len();
        return len - 1;
    }

    fn write(&mut self, buf : &[u8]) {
        let mut sent = 0usize;

        let consumer_idx = self.get_output_consumer_idx(); 
        let mut producer_idx = self.get_output_producer_idx();

        mb();

        {
            let output_mask = { self.output_mask() };
            let output_len = { self.output_buffer().len() };
            let mut output = self.output_buffer();

            while (sent < buf.len()) && ((producer_idx - consumer_idx) < output_len) {
                let idx = producer_idx & output_mask; // mask the index to make sure we don't overflow
                producer_idx = producer_idx + 1;
                output[idx] = buf[sent];
                sent = sent + 1;
            }
        }

        wmb();

        self.set_output_producer_idx(producer_idx);

    }

    fn write_notify(&mut self, buf : &[u8], evtchn : u32) {
        self.write(buf);
        unsafe { send(evtchn); }
    }

}
