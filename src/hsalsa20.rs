use super::*;
use salsa20::Salsa20;

pub struct HSalsa20;

impl HSalsa20 {

    // Generate a 256-bit block where r = 20.
    pub fn hsalsa20(x: [u8; 64]) -> [u8; 32] {
        let mut xs = [0u32; 16];
        for i in 0..16 {
            xs[i] = little_endian([x[i*4], x[i*4+1], x[i*4+2], x[i*4+3]]);
        }

        // Run salsa20 double_round r/2
        let mut zs = xs;
        for _ in 0..10 {
            zs = Salsa20::double_round(zs);
        }

        let block: [u32; 8] = [zs[0], zs[5], zs[10], zs[15], zs[6], zs[7], zs[8], zs[9]];

        let mut rs = [0; 32];
        for i in 0..8 {
            let vs = little_endian_inv(block[i]);
            rs[i*4] = vs[0];
            rs[i*4+1] = vs[1];
            rs[i*4+2] = vs[2];
            rs[i*4+3] = vs[3];
        }
        rs
    }

}
