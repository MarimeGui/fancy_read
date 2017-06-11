use std::io::Read;
use std::mem::transmute;


pub fn read_to_u8<R: Read>(reader: &mut R) -> u8 {
    let mut temp: [u8; 1] = [0];
    reader.read_exact(&mut temp[..]).expect("Failed to read");
    temp[0]
}

pub fn read_le_to_u16<R: Read>(reader: &mut R) -> u16 {
    let mut temp: [u8; 2] = [0; 2];
    reader.read_exact(&mut temp[..]).expect("Failed to read");
    #[cfg(target_endian = "big")]
    temp.reverse();
    unsafe {
        transmute::<[u8; 2], u16>(temp)
    }
}

pub fn read_be_to_u16<R: Read>(reader: &mut R) -> u16 {
    let mut temp: [u8; 2] = [0; 2];
    reader.read_exact(&mut temp[..]).expect("Failed to read");
    #[cfg(target_endian = "little")]
    temp.reverse();
    unsafe {
        transmute::<[u8; 2], u16>(temp)
    }
}

pub fn read_le_to_u32<R: Read>(reader: &mut R) -> u32 {
    let mut temp: [u8; 4] = [0; 4];
    reader.read_exact(&mut temp[..]).expect("Failed to read");
    #[cfg(target_endian = "big")]
    temp.reverse();
    unsafe {
        transmute::<[u8; 4], u32>(temp)
    }
}

pub fn read_be_to_u32<R: Read>(reader: &mut R) -> u32 {
    let mut temp: [u8; 4] = [0; 4];
    reader.read_exact(&mut temp[..]).expect("Failed to read");
    #[cfg(target_endian = "little")]
    temp.reverse();
    unsafe {
        transmute::<[u8; 4], u32>(temp)
    }
}

pub fn read_le_to_i32<R: Read>(reader: &mut R) -> i32 {
    let mut temp: [u8; 4] = [0; 4];
    reader.read_exact(&mut temp[..]).expect("Failed to read");
    #[cfg(target_endian = "big")]
    temp.reverse();
    unsafe {
        transmute::<[u8; 4], i32>(temp)
    }
}

pub fn read_be_to_i32<R: Read>(reader: &mut R) -> i32 {
    let mut temp: [u8; 4] = [0; 4];
    reader.read_exact(&mut temp[..]).expect("Failed to read");
    #[cfg(target_endian = "little")]
    temp.reverse();
    unsafe {
        transmute::<[u8; 4], i32>(temp)
    }
}

pub fn read_to_string<R: Read>(reader: &mut R, length: u32) -> String {
    let mut temp = String::with_capacity(length as usize);
    reader.read_to_string(&mut temp);
    temp
}
