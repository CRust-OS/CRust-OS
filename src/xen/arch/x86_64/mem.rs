#[inline]
pub unsafe fn mb(){
    asm!("mfence" : : : "memory" : "volatile");
}

#[inline]
pub unsafe fn wmb(){
    asm!("sfence" : : : "memory" : "volatile");
}
