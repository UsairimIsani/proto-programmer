pub mod bittobyte {
    pub fn convert_bits_to_bytes(byte: u8) -> Vec<u8> {
        let st = format!("{:b}", byte);
        let st: Vec<&str> = st.split("").collect();
        let mut bytes_array: Vec<u8> = vec![];
        for i in st.iter() {
            match *i {
                "1" => {
                    &bytes_array.push(0x01);
                }
                "0" => {
                    &bytes_array.push(0x00);
                }
                _ => (),
            }
        }
        bytes_array
    }
}
