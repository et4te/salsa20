#![feature(slice_patterns)]

pub mod salsa20;
pub mod hsalsa20;
pub mod xsalsa20;

// Generates code for the conversion of bytes to an integral type in little
// endian format.
macro_rules! little_endian {
    ($type:ty, $arr:expr, $nbytes:expr) => {
        {
            let mut r: $type = $arr[0] as $type;
            for i in 1..$nbytes {
                r += ($arr[i] as $type) << (8 * i);
            }
            r
        }
    }
}

// Generates code for the conversion of an integral type to bytes in little
// endian format.
macro_rules! little_endian_inv {
    ($x:expr, $nbytes:expr) => {
        {
            let mut b: [u8; $nbytes] = [0; $nbytes];
            b[0] = ($x & 0xFF) as u8;
            for i in 1..$nbytes {
                b[i] = (($x & (0xFF << i * 8)) >> i * 8) as u8;
            }
            b
        }
    }
}

pub fn little_endian(b: [u8; 4]) -> u32 {
    little_endian!(u32, b, 4)
}

pub fn little_endian_u64(b: [u8; 8]) -> u64 {
    little_endian!(u64, b, 8)
}

pub fn little_endian_inv(x: u32) -> [u8; 4] {
    little_endian_inv!(x, 4)
}

pub fn little_endian_inv64(x: u64) -> [u8; 8] {
    little_endian_inv!(x, 8)
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
