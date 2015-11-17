#[repr(C)]
struct trap_info {
    vector:     u8,
    flags:      u8,
    cs:         u16,
    addres:     usize
}

const fn divide_error(){ loop{} }
const fn debug(){ loop{} }
const fn int3(){ loop{} }
const fn overflow(){ loop{} }

//static [trap_info; 20] table = 
//[
    //trap_info{ vector: 0, flags: 0, cs: 0xe033, 
