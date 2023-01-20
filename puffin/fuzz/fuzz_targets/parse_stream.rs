#![no_main]

use puffin::Stream;
use puffin::StreamInfo;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let data_vec = data.to_vec();
    let stream = Stream::from(data_vec);
    let _ = StreamInfo::parse(stream);
});