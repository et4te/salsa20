use super::bytes::*;

pub struct Salsa20 {
    pub key: [u8; 32],
    pub nonce: [u8; 8],
    pub counter: u64,
}

// Salsa20 implements the iterator trait.
impl Iterator for Salsa20 {
    type Item = Block64;

    fn next(&mut self) -> Option<Block64> {
        let mut v = [0; 16];
        let counter = le_inverse_u64(self.counter);
        for i in 0..8 {
            v[i] = self.nonce[i];
            v[i+8] = counter[i];
        }
        let c = Salsa20::sigma(self.key, v);
        let b = Salsa20::next(c);
        self.counter += 1;
        Some(Block64 { block: b })
    }
}

impl Salsa20 {

    pub fn new(key: [u8; 32], nonce: [u8; 8]) -> Salsa20 {
        Salsa20 {
            key: key,
            nonce: nonce,
            counter: 0,
        }
    }

    pub fn next(x: [u8; 64]) -> [u8; 64] {
        let mut xs = [0u32; 16];
        for i in 0..16 {
            xs[i] = le_u32([x[i*4], x[i*4+1], x[i*4+2], x[i*4+3]]);
        }

        // Run salsa20 double_round r/2 where r= 20 (Salsa20/20)
        let mut zs = xs;
        for _ in 0..10 {
            zs = Salsa20::double_round(zs);
        }

        let mut rs = [0; 64];
        for i in 0..16 {
            let vs = le_inverse_u32(zs[i].wrapping_add(xs[i]));
            rs[i*4] = vs[0];
            rs[i*4+1] = vs[1];
            rs[i*4+2] = vs[2];
            rs[i*4+3] = vs[3];
        }
        rs
    }

    // Primitives
    pub fn quarter_round(y: [u32; 4]) -> [u32; 4] {
        let z1 = y[1] ^ (y[0].wrapping_add(y[3])).rotate_left(7);
        let z2 = y[2] ^ (z1.wrapping_add(y[0])).rotate_left(9);
        let z3 = y[3] ^ (z2.wrapping_add(z1)).rotate_left(13);
        let z0 = y[0] ^ (z3.wrapping_add(z2)).rotate_left(18);
        [z0, z1, z2, z3]
    }

    pub fn row_round(y: [u32; 16]) -> [u32; 16] {
        let [z0, z1, z2, z3] = Salsa20::quarter_round([y[0], y[1], y[2], y[3]]);
        let [z5, z6, z7, z4] = Salsa20::quarter_round([y[5], y[6], y[7], y[4]]);
        let [z10, z11, z8, z9] = Salsa20::quarter_round([y[10], y[11], y[8], y[9]]);
        let [z15, z12, z13, z14] = Salsa20::quarter_round([y[15], y[12], y[13], y[14]]);
        [ z0, z1, z2, z3, z4, z5, z6, z7, z8, z9, z10, z11, z12, z13, z14, z15 ]
    }

    pub fn column_round(x: [u32; 16]) -> [u32; 16] {
        let [y0, y4, y8, y12] = Salsa20::quarter_round([x[0], x[4], x[8], x[12]]);
        let [y5, y9, y13, y1] = Salsa20::quarter_round([x[5], x[9], x[13], x[1]]);
        let [y10, y14, y2, y6] = Salsa20::quarter_round([x[10], x[14], x[2], x[6]]);
        let [y15, y3, y7, y11] = Salsa20::quarter_round([x[15], x[3], x[7], x[11]]);
        [ y0, y1, y2, y3, y4, y5, y6, y7, y8, y9, y10, y11, y12, y13, y14, y15 ]
    }

    pub fn double_round(x: [u32; 16]) -> [u32; 16] {
        Salsa20::row_round(Salsa20::column_round(x))
    }

    // Salsa20 constant expansion (16 byte key).
    pub fn expand16(k: [u8; 16], n: [u8; 16]) -> [u8; 64] {
        let mut r = [0; 64];

        let t0 = [101, 120, 112, 97];
        for i in 0..4 {
            r[i] = t0[i];
        }
        for i in 0..16 {
            r[i+4] = k[i];
        }

        let t1 = [110, 100, 32, 49];
        for i in 0..4 {
            r[i+20] = t1[i];
        }
        for i in 0..16 {
            r[i+24] = n[i];
        }

        let t2 = [54, 45, 98, 121];
        for i in 0..4 {
            r[i+40] = t2[i];
        }
        for i in 0..16 {
            r[i+44] = k[i];
        }

        let t3 = [116, 101, 32, 107];
        for i in 0..4 {
            r[i+60] = t3[i];
        }

        r
    }

    pub fn sigma(k: [u8; 32], n: [u8; 16]) -> [u8; 64] {
        let mut k0 = [0; 16];
        let mut k1 = [0; 16];
        for i in 0..16 {
            k0[i] = k[i];
            k1[i] = k[i+16];
        }
        Salsa20::expand32(k0, k1, n)
    }

    // Salsa20 constant expansion (32 bytes).
    pub fn expand32(k0: [u8; 16], k1: [u8; 16], n: [u8; 16]) -> [u8; 64] {
        let mut r = [0; 64];

        let o0 = [101, 120, 112, 97];
        for i in 0..4 {
            r[i] = o0[i];
        }
        for i in 0..16 {
            r[i+4] = k0[i];
        }

        let o1 = [110, 100, 32, 51];
        for i in 0..4 {
            r[i+20] = o1[i];
        }
        for i in 0..16 {
            r[i+24] = n[i];
        }

        let o2 = [50, 45, 98, 121];
        for i in 0..4 {
            r[i+40] = o2[i];
        }
        for i in 0..16 {
            r[i+44] = k1[i];
        }

        let o3 = [116, 101, 32, 107];
        for i in 0..4 {
            r[i+60] = o3[i];
        }

        r
    }

    // Salsa20/20 128-bit key
    pub fn salsa20_20_16(k: [u8; 16], v: [u8; 8], m: Vec<u8>) -> Vec<u8> {
        let mut r = vec![0; m.len()];
        let mut i = 0;
        let mut c = 0;
        loop {
            if i >= m.len() {
                break;
            } else {
                let mut vi = [0; 16];
                for j in 0..8 {
                    vi[j] = v[j];
                }

                let counter: [u8; 8] = le_inverse_u64(c as u64);
                for j in 0..8 {
                    vi[j+8] = counter[j];
                }

                // Generate the salsa20 constant as a 128bit expansion
                let expansion = Salsa20::expand16(k, vi);

                // Run salsa20/20 on the salsa20 constant
                let bytes = Salsa20::next(expansion);

                // Copy generated bytes to output whilst XORing them with the
                // provided message truncating the length to m.len().
                if (m.len() - i) >= 64 {
                    for j in 0..64 {
                        r[i + j] = bytes[j] ^ m[i + j];
                    }
                } else {
                    for j in 0..(m.len() % 64) {
                        r[i + j] = bytes[j] ^ m[i + j];
                    }
                }

                i += 64;
                c += 1;
            }
        }
        r
    }

    // Salsa20/20 256-bit (32-byte key)
    pub fn salsa20_20_32(k0: [u8; 16], k1: [u8; 16], v: [u8; 8], m: Vec<u8>) -> Vec<u8> {
        let mut r = vec![0; m.len()];
        let mut i = 0;
        let mut n = 0;
        loop {
            if i >= m.len() {
                break;
            } else {
                let mut vi = [0; 16];
                for j in 0..8 {
                    vi[j] = v[j];
                }

                let counter: [u8; 8] = le_inverse_u64(n as u64);
                for j in 0..8 {
                    vi[j+8] = counter[j];
                }

                let c = Salsa20::expand32(k0, k1, vi);
                let b = Salsa20::next(c);

                // copy generated bytes to output whilst XORing them with the
                // provided message truncating the length to m.len().
                if (m.len() - i) >= 64 {
                    for j in 0..64 {
                        r[i + j] = b[j] ^ m[i + j];
                    }
                } else {
                    for j in 0..(m.len() % 64) {
                        r[i + j] = b[j] ^ m[i + j];
                    }
                }

                i += 64;
                n += 1
            }
        }
        r
    }
}
