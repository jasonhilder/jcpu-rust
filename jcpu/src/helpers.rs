use std::{fs,fs::File};
use std::io::Read;

pub fn read_bin_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

pub fn read_instructions_to_vec(filename: &str) -> Vec<String> {
    let read = fs::read_to_string(&filename).expect("Could not read file");
    let res: Vec<String> = read.split("\n").map(|s| s.to_string()).collect();

    res
}
