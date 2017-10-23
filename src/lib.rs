#![feature(slice_patterns)]

pub mod salsa20;
pub mod hsalsa20;
pub mod xsalsa20;

pub fn little_endian(b: [u8; 4]) -> u32 {
    b[0] as u32 + ((b[1] as u32) << 8) + ((b[2] as u32) << 16) + ((b[3] as u32) << 24)
}

pub fn little_endian_u64(b: [u8; 8]) -> u64 {
    b[0] as u64 + ((b[1] as u64) << 8) + ((b[2] as u64) << 16) + ((b[3] as u64) << 24) +
        ((b[4] as u64) << 32) + ((b[5] as u64) << 40) + ((b[6] as u64) << 48) + ((b[7] as u64) << 56)
}

pub fn little_endian_inv(x: u32) -> [u8; 4] {
    let mut bytes: [u8; 4] = [0; 4];
    bytes[0] = (x & 0xFF) as u8;
    for i in 1..4 {
        bytes[i] = ((x & (0xFF << i * 8)) >> i * 8) as u8;
    }
    bytes
}

pub fn little_endian_inv64(x: u64) -> [u8; 8] {
    let mut bytes: [u8; 8] = [0; 8];
    bytes[0] = (x & 0xFF) as u8;
    for i in 1..8 {
        bytes[i] = ((x & (0xFF << i * 8)) >> i * 8) as u8;
    }
    bytes
}

pub fn bytes_eq(v1: Vec<u8>, v2: Vec<u8>) -> bool {
    if v1.len() != v2.len() {
        false
    } else {
        for i in 0..v1.len() {
            if v1[i] != v2[i] {
                return false;
            }
        }
        true
    }
}

pub fn xor_digest(enc: Vec<u8>) -> Vec<u8> {
    let mut r = vec![0; 64];
    let mut i = 0;
    loop {
        if i >= enc.len() / 64 {
            break;
        } else {
            if enc.len() >= i * 64 + 64 {
                for j in 0..64 {
                    let p = i * 64 + j;
                    r[p % 64] = r[p % 64] ^ enc[p];
                }
            } else {
                for j in 0..enc.len() % 64 {
                    let p = i * 64 + j;
                    r[p % 64] = r[p % 64] ^ enc[p];
                }
            }
            i += 1;
        }
    }
    r
}
