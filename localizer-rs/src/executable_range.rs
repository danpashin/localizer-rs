use mach_o_sys::{
    dyld::{mach_header, uintptr_t},
    getsect,
};
use std::ffi::CString;

#[derive(Debug)]
pub struct Range {
    start: uintptr_t,
    end: uintptr_t,
}

impl Range {
    pub fn from_header(mh: *const mach_header) -> Self {
        unsafe {
            let mh = mh as *const getsect::mach_header_64;

            let mut size = 0;
            let seg_name = CString::new("__TEXT").unwrap();
            let start = getsect::getsegmentdata(mh, seg_name.as_ptr(), &mut size);
            let end = start.offset(size as isize);

            Self {
                start: start as uintptr_t,
                end: end as uintptr_t,
            }
        }
    }

    pub fn contains_address(&self, address: uintptr_t) -> bool {
        (self.start..self.end).contains(&address)
    }
}
