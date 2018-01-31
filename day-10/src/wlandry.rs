extern "C" {
    fn wlandry(ptr: *const u8, len: usize) -> *mut u8;
}

pub fn part2(text: &str) {
    unsafe {
        wlandry(text.as_ptr(), text.len());
    }
}
