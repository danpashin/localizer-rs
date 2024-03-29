use crate::IMAGES_TO_TRANSLATE;
use mach_o_sys::{
    dyld::{mach_header, mach_header_64, uint32_t, uuid_command},
    getsect::{load_command, LC_UUID},
};
use uuid::Uuid;

pub unsafe fn uuid_if_can_translate(mh: *const mach_header) -> Option<Uuid> {
    let uuid_cmd = find_uuid_cmd(mh)?;
    let uuid = Uuid::from_slice((*uuid_cmd).uuid.as_ref()).ok()?;

    let images = IMAGES_TO_TRANSLATE.read().unwrap();
    match images.contains_key(&uuid) {
        true => Some(uuid),
        false => None,
    }
}

pub unsafe fn find_uuid_cmd(mh: *const mach_header) -> Option<*const uuid_command> {
    let mh = mh as *const mach_header_64;
    let mut command_ptr = mh.offset(1) as *const load_command;

    for _ in 0..(*mh).ncmds {
        let load_command { cmd, cmdsize, .. } = *command_ptr;

        if cmd == LC_UUID as uint32_t {
            return Some(command_ptr as *const uuid_command);
        }

        command_ptr = (command_ptr as *const u8).offset(cmdsize as isize) as *const load_command;
    }

    None
}
