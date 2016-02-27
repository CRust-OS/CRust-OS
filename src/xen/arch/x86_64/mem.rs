#[inline]
pub fn mb(){
    unsafe { asm!("mfence" : : : "memory" : "volatile"); }
}

#[inline]
pub fn wmb(){
    unsafe{ asm!("sfence" : : : "memory" : "volatile"); }
}