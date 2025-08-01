//! https://www.w3.org/TR/png-3/#samplecrc

pub fn print_crc_table() {
    let table = generate_crc_table();
    for entry in table {
        println!("0x{entry:08X}");
    }
}

pub fn generate_crc_table() -> [u32; 256] {
    let mut table = [0u32; 256];

    for (n, entry) in table.iter_mut().enumerate() {
        let mut c = n as u32;
        for _ in 0..8 {
            if (c & 1) == 1 {
                c = 0xEDB88320 ^ (c >> 1);
            } else {
                c >>= 1;
            }
        }
        *entry = c;
    }

    table
}
