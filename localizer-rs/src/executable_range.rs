use mach_o_sys::dyld::uintptr_t;

#[derive(Debug)]
pub struct Range {
    start: uintptr_t,
    end: uintptr_t,
}

impl Range {
    pub fn new(start: uintptr_t, end: uintptr_t) -> Self {
        Self { start, end }
    }

    pub fn contains_address(&self, address: uintptr_t) -> bool {
        (self.start..self.end).contains(&address)
    }
}
