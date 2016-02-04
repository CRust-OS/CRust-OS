use ::xen::arch::mem::*;
use ::xen::event_channels::send;

pub trait ReadableRing {
    fn read(&mut self) {
        // TODO
    }
    fn input_buffer(&mut self) -> &mut [u8];
    fn get_in_cons(&self) -> usize; 
    fn get_in_prod(&self) -> usize;
    fn set_in_cons(&mut self, usize);
    fn set_in_prod(&mut self, usize);
    fn input_mask(&mut self) -> usize {
        let len = self.input_buffer().len();
        return len - 1;
    }
}

pub trait WritableRing {
    fn output_buffer(&mut self) -> &mut [u8];

    fn get_out_cons(&self) -> usize;
    fn get_out_prod(&self) -> usize;
    fn set_out_cons(&mut self, usize);
    fn set_out_prod(&mut self, usize);


    fn output_mask(&mut self) -> usize {
        let len = self.output_buffer().len();
        return len - 1;
    }

    fn write(&mut self, buf : &[u8]) {
        let mut sent = 0usize;

        let cons = self.get_out_cons(); 
        let mut prod = self.get_out_prod();

        mb();

        {
            let output_mask = self.output_mask();
            let mut output = self.output_buffer();
            let output_len = output.len();

            while (sent < buf.len()) && ((prod - cons) < output_len) {
                let idx = prod & output_mask; // mask the index to make sure we don't overflow
                prod = prod + 1;
                output[idx] = buf[sent];
                sent = sent + 1;
            }
        }

        wmb();

        self.set_out_prod(prod);

    }

    fn write_notify(&mut self, buf : &[u8], evtchn : u32) {
        self.write(buf);
        unsafe { send(evtchn); }
    }
}
