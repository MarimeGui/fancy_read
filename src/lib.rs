use std::io::Read;

fn read_to_u32_le<T: Read>(reader: &mut T) -> u32 {
    let mut temp_read: [u8; 4] = [0; 4];
    reader.read_exact(&mut temp_read[..]).expect("Failed to read");
    let temp: [u8; 4] = [temp_read[3], temp_read[2], temp_read[1], temp_read[0]];
    unsafe {
        mem::transmute(temp)
    }
}

fn read_to_u8<T: Read>(reader: &mut T) -> u8 {
    let mut temp: [u8; 1] = [0];
    reader.read_exact(&mut temp[..]).expect("Failed to read");
    temp[0]
}
