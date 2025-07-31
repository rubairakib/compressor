extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;

fn main() {
    let mut arguments = args();

    arguments.next();

    let source = match arguments.next() {
        Some(path) => path,
        None => {
            eprintln!("Usage: <source> <target>");
            return;
        }
    };

    let target = match arguments.next() {
        Some(path) => path,
        None => {
            eprintln!("Usage: <source> <target>");
            return;
        }
    };

    let input_file = File::open(&source).expect("Failed to open source file");
    let mut input = BufReader::new(input_file);
    let output = File::create(&target).expect("Failed to create target file");

    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();

    copy(&mut input, &mut encoder).expect("Failed to compress");

    let output = encoder.finish().expect("Failed to finish compression");

    println!(
        "Source len: {} bytes",
        input.get_ref().metadata().unwrap().len()
    );
    println!(
        "Target len: {} bytes",
        output.metadata().unwrap().len()
    );
    println!("Elapsed: {:?}", start.elapsed());
}
