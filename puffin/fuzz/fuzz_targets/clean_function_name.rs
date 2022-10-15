#![no_main]

use puffin::clean_function_name;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let s = String::from_utf8_lossy(data);
    let _ = clean_function_name(&s);
});