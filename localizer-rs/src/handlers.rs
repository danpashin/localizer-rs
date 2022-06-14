use crate::{executable_range::Range, helpers, EXECUTABLE_RANGES};
use mach_o_sys::dyld::{self, intptr_t, mach_header};

pub fn init() {
    unsafe {
        dyld::_dyld_register_func_for_add_image(Some(handler_register_image));
        dyld::_dyld_register_func_for_remove_image(Some(handler_remove_image));
    }
}

unsafe extern "C" fn handler_register_image(mh: *const mach_header, _vmaddr_slide: intptr_t) {
    if let Some(uuid) = helpers::uuid_if_can_translate(mh) {
        let mut ranges = EXECUTABLE_RANGES.write().expect("Can't lock for writing");
        ranges.insert(uuid, Range::from_header(mh));
    }
}

unsafe extern "C" fn handler_remove_image(mh: *const mach_header, _vmaddr_slide: intptr_t) {
    if let Some(uuid) = helpers::uuid_if_can_translate(mh) {
        let mut ranges = EXECUTABLE_RANGES.write().expect("Can't lock for writing");
        ranges.remove(&uuid);
    }
}
