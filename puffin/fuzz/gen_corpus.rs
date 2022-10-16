// create a debug stream and write it to a file.

use puffin::Stream;

fn main() {
    let stream_1 = {
        let mut stream = Stream::default();
        let t0 = stream.begin_scope(100, "top", "top.rs", "data_top");
        let m1 = stream.begin_scope(200, "middle_0", "middle.rs", "data_middle_0");
        stream.end_scope(m1, 300);
        let m1 = stream.begin_scope(300, "middle_1", "middle.rs:42", "data_middle_1");
        stream.end_scope(m1, 400);
        stream.end_scope(t0, 400);
        stream
    };

    let stream_2 = {
        let mut stream = Stream::default();

        for i in 0..2 {
            let ns = 1000 * i;
            let a = stream.begin_scope(ns + 100, "a", "", "");
            stream.end_scope(a, ns + 200);

            let b = stream.begin_scope(ns + 200, "b", "", "");

            let ba = stream.begin_scope(ns + 400, "ba", "", "");
            stream.end_scope(ba, ns + 600);

            let bb = stream.begin_scope(ns + 600, "bb", "", "");
            let bba = stream.begin_scope(ns + 600, "bba", "", "");
            stream.end_scope(bba, ns + 700);
            stream.end_scope(bb, ns + 800);
            stream.end_scope(b, ns + 900);
        }

        stream
    };

    let stream_3 = {
        let mut stream = Stream::default();
        let t0 = stream.begin_scope(100, "top", "top.rs", "data_top");
        let m1 = stream.begin_scope(200, "middle_earth_0", "middle.rs", "data_middle_0");
        stream.end_scope(m1, 500);
        let m1 = stream.begin_scope(300, "middle_earth_1", "middle.rs:42", "data_middle_1");
        stream.end_scope(m1, 400);
        stream.end_scope(t0, 700);
        stream
    };

    // Write the streams to a file.
    std::fs::write("stream_1.bin", stream_1.bytes()).unwrap();
    std::fs::write("stream_2.bin", stream_2.bytes()).unwrap();
    std::fs::write("stream_3.bin", stream_3.bytes()).unwrap();

    // Read the streams from the file.
    let stream_1 = Stream::from(std::fs::read("stream_1.bin").unwrap());
    let stream_2 = Stream::from(std::fs::read("stream_2.bin").unwrap());
    let stream_3 = Stream::from(std::fs::read("stream_3.bin").unwrap());

    // Print the length of the streams.
    println!("stream_1.len(): {}", stream_1.len());
    println!("stream_2.len(): {}", stream_2.len());
    println!("stream_3.len(): {}", stream_3.len());

    // Parse the streams.
    let _ = puffin::StreamInfo::parse(stream_1).unwrap();
    let _ = puffin::StreamInfo::parse(stream_2).unwrap();
    let _ = puffin::StreamInfo::parse(stream_3).unwrap();
}