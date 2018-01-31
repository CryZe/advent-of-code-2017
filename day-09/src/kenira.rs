use std::os::raw::c_int;

#[repr(C)]
struct Result {
    part1: c_int,
    part2: c_int,
}

extern "C" {
    fn kenira(txt: *const u8, len: c_int) -> Result;
}

pub fn both(text: &str) -> (usize, usize) {
    let res = unsafe { kenira(text.as_ptr(), text.len() as _) };
    (res.part1 as _, res.part2 as _)
}
