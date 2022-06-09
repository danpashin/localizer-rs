use mach_o_sys::{
    dyld::{self, intptr_t, mach_header, uintptr_t},
    getsect,
};
use std::ffi::CString;

use crate::{executable_range::Range, helpers, EXECUTABLE_RANGES};

pub fn init() {
    unsafe {
        dyld::_dyld_register_func_for_add_image(Some(handler_register_image));
        dyld::_dyld_register_func_for_remove_image(Some(handler_remove_image));
    }
}

unsafe extern "C" fn handler_register_image(mh: *const mach_header, vmaddr_slide: intptr_t) {
    log::trace!(
        "register image with header={:?}; slide={:#x}",
        mh,
        vmaddr_slide
    );
    let uuid = helpers::uuid_if_can_translate(mh);
    if uuid.is_none() {
        return;
    }
    let uuid = uuid.unwrap();
    log::debug!("Can process image with uuid {}", uuid);

    let mh = mh as *const getsect::mach_header_64;

    let mut size = 0;
    let seg_name = CString::new("__TEXT").unwrap();
    let start = getsect::getsegmentdata(mh, seg_name.as_ptr(), &mut size);
    let end = start.offset(size as isize);

    log::debug!("image __TEXT segment range = {:?}-{:?}", start, end);

    let mut ranges = EXECUTABLE_RANGES.write().expect("Can't lock for writing");
    ranges.insert(uuid, Range::new(start as uintptr_t, end as uintptr_t));
}

unsafe extern "C" fn handler_remove_image(mh: *const mach_header, _vmaddr_slide: intptr_t) {
    if let Some(uuid) = helpers::uuid_if_can_translate(mh) {
        log::trace!("Called unregiser image with header={:?}", mh);
        let mut ranges = EXECUTABLE_RANGES.write().expect("Can't lock for writing");
        ranges.remove(&uuid);
    }
}
