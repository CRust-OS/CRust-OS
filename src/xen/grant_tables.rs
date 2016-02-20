use xen::console_io::STDOUT;
use core::fmt::Write;
use nodrop::NoDrop;
use core::mem;
use core::ops::{Deref, DerefMut};
use collections::{String, Vec};
use ::xen::hypercalls::*;
use ::xen::hypercalls::grant_table_op::*;

const NR_RESERVED_ENTRIES : usize = 8;
const NR_FRAMES : usize = 4;
const NR_ENTRIES : usize = 1024; // == PAGE_SIZE*NR_FRAMES / sizeof(grant_entry)

static mut GRANT_ENTRIES : *mut GrantEntry = 0 as *mut GrantEntry;
static mut len : usize = 0;
static mut cap : usize = 0;

struct GrantEntryTable {
    table : NoDrop<Vec<GrantEntry>>
}

impl GrantEntryTable {
    fn init_table(){
        let mut v = Vec::with_capacity(NR_ENTRIES);
        unsafe {
            GRANT_ENTRIES = v.as_mut_ptr();
        }
        GrantEntryTable {
            table: NoDrop::new(v)
        };
    }
    pub fn create() -> GrantEntryTable {
        let v = unsafe{ Vec::from_raw_parts(GRANT_ENTRIES, len, cap) };
        GrantEntryTable {
            table : NoDrop::new(v)
        }
    }
}

// When the struct gets dropped, write back all of the changes for when we make a new one
impl Drop for GrantEntryTable {
    fn drop(&mut self) {
        unsafe {
            len = self.table.len();
            cap = self.table.capacity();
        }
    }
}

impl Deref for GrantEntryTable {
    type Target = NoDrop<Vec<GrantEntry>>;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl DerefMut for GrantEntryTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

pub fn init_grant_table() -> Result<(), String> {
    unsafe{ 
        GrantEntryTable::init_table();
        let mut args = setup_table::Args {
            dom: DomID::SELF,
            nr_frames: NR_FRAMES as u32,
            status: mem::zeroed(),
            frame_list: &mut PFN(GRANT_ENTRIES as u64) as *mut PFN
        };
        let _result = hypercall!(
            i64,
            Command::grant_table_op,
            SubCommand::setup_table,
            &mut args as *mut setup_table::Args,
            1u32            // number of arguments: 1
            );
        args.status.into()
    }
}

pub fn map_page(){}
pub fn tranfer_page(){}
