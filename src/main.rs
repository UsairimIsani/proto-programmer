mod bittobyte;
mod config;
use bittobyte::bittobyte::convert_bits_to_bytes;
use config::config::{configure, Config};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
fn main() {
    let _config: Config = configure();
    let byte = 0x89;
    let bytes_array: Vec<u8> = convert_bits_to_bytes(byte);
    println!("{:?}", &bytes_array);
    for i in bytes_array.iter() {
        print!("{:b}", i);
    }
    println!();
    let file = File::open("/home/usairim/proto-programmer/binary-xilinx.bin").unwrap();
    let buf_reader = BufReader::new(file);
    let bytes = buf_reader.bytes();
    let mut new_byte_array: Vec<Vec<u8>> = vec![];
    for bt in bytes {
        let some = convert_bits_to_bytes(bt.unwrap());
        &new_byte_array.push(some);
    }
    let new_byte_array: Vec<u8> = new_byte_array
        .iter()
        .flat_map(|array| array.iter())
        .cloned()
        .collect();
    let mut bang_file = File::create("/home/usairim/proto-programmer/bang.bang").unwrap();
    bang_file.write_all(&mut &new_byte_array);
    print!("{:?} {:?}", new_byte_array, bang_file);
}
