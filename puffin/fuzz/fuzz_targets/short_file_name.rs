#![no_main]

use puffin::short_file_name;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let s = String::from_utf8_lossy(data);
    let _ = short_file_name(&s);
});