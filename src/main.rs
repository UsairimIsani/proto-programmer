mod bittobyte;
mod config;
use bittobyte::bittobyte::convert_bits_to_bytes;
use config::config::{configure, Config};
fn main() {
    let _config: Config = configure();
    let byte = 0x89;
    let bytes_array: Vec<u8> = convert_bits_to_bytes(byte);
    println!("{:?}", &bytes_array);
    for i in bytes_array.iter() {
        print!("{:b}", i);
    }
    println!();
}
