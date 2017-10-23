extern crate xsalsa20;

#[cfg(test)]
mod tests {
    use xsalsa20::*;
    use xsalsa20::salsa20::*;

    #[test]
    fn test_quarter_round() {
        let test_v1 = [0x00000000, 0x00000000, 0x00000000, 0x00000000];
        let test_r1 = [0x00000000, 0x00000000, 0x00000000, 0x00000000];
        assert_eq!(Salsa20::quarter_round(test_v1), test_r1);
        let test_v2 = [0x00000001, 0x00000000, 0x00000000, 0x00000000];
        let test_r2 = [0x08008145, 0x00000080, 0x00010200, 0x20500000];
        assert_eq!(Salsa20::quarter_round(test_v2), test_r2);
    }

    #[test]
    fn test_row_round() {
        let test_v1 = [
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
        ];
        let test_r1 = [
            0x08008145, 0x00000080, 0x00010200, 0x20500000,
            0x20100001, 0x00048044, 0x00000080, 0x00010000,
            0x00000001, 0x00002000, 0x80040000, 0x00000000,
            0x00000001, 0x00000200, 0x00402000, 0x88000100,
        ];
        assert_eq!(Salsa20::row_round(test_v1), test_r1);
    }

    #[test]
    fn test_column_round() {
        let test_v1 = [
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
        ];
        let test_r1 = [
            0x10090288, 0x00000000, 0x00000000, 0x00000000,
            0x00000101, 0x00000000, 0x00000000, 0x00000000,
            0x00020401, 0x00000000, 0x00000000, 0x00000000,
            0x40a04001, 0x00000000, 0x00000000, 0x00000000,
        ];
        assert_eq!(Salsa20::column_round(test_v1), test_r1)
    }

    #[test]
    fn test_double_round() {
        let test_v1 = [
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000,
        ];
        let test_r1 = [
            0x8186a22d, 0x0040a284, 0x82479210, 0x06929051,
            0x08000090, 0x02402200, 0x00004000, 0x00800000,
            0x00010200, 0x20400000, 0x08008104, 0x00000000,
            0x20500000, 0xa0000040, 0x0008180a, 0x612a8020,
        ];
        assert_eq!(Salsa20::double_round(test_v1), test_r1)
    }

    #[test]
    fn test_little_endian() {
        let test_v1 = [0, 0, 0, 0];
        assert_eq!(little_endian(test_v1), 0);
        assert_eq!(little_endian_inv(0), test_v1);
        let test_v2 = [86, 75, 30, 9];
        assert_eq!(little_endian(test_v2), 0x091e4b56);
        assert_eq!(little_endian_inv(0x091e4b56), test_v2);
        let test_v3 = [255, 255, 255, 250];
        assert_eq!(little_endian(test_v3), 0xfaffffff);
        assert_eq!(little_endian_inv(0xfaffffff), test_v3);
    }

    #[test]
    fn test_salsa20() {
        let test_v1 = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let r1 = Salsa20::salsa20(test_v1);
        let mut r1_result = true;
        for i in 0..64 {
            if r1[i] != test_v1[i] {
                r1_result = false;
            }
        }
        assert_eq!(r1_result, true);

        let test_v2 = [
            211,159, 13,115, 76, 55, 82,183, 3,117,222, 37,191,187,234,136,
            49,237,179, 48, 1,106,178,219,175,199,166, 48, 86, 16,179,207,
            31,240, 32, 63, 15, 83, 93,161,116,147, 48,113,238, 55,204, 36,
            79,201,235, 79, 3, 81,156, 47,203, 26,244,243, 88,118,104, 54,
        ];
        let test_r2 = [
            109, 42,178,168,156,240,248,238,168,196,190,203, 26,110,170,154,
            29, 29,150, 26,150, 30,235,249,190,163,251, 48, 69,144, 51, 57,
            118, 40,152,157,180, 57, 27, 94,107, 42,236, 35, 27,111,114,114,
            219,236,232,135,111,155,110, 18, 24,232, 95,158,179, 19, 48,202,
        ];

        let r2 = Salsa20::salsa20(test_v2);
        let mut r2_result = true;
        for i in 0..64 {
            if r2[i] != test_r2[i] {
                r2_result = false;
            }
        }
        assert_eq!(r2_result, true);
    }

    #[test]
    fn test_salsa20_expansion_128() {
        let k0 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let n = [101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116];
        let test_v1 = [
            39,173, 46,248, 30,200, 82, 17, 48, 67,254,239, 37, 18, 13,247,
            241,200, 61,144, 10, 55, 50,185, 6, 47,246,253,143, 86,187,225,
            134, 85,110,246,161,163, 43,235,231, 94,171, 51,145,214,112, 29,
            14,232, 5, 16,151,140,183,141,171, 9,122,181,104,182,177,193,
        ];
        let expansion = Salsa20::salsa20_expansion_128(k0, n);
        let r1 = Salsa20::salsa20(expansion);
        let mut r1_result = true;
        for i in 0..64 {
            if r1[i] != test_v1[i] {
                println!("{}: {} != {}: {}", i, r1[i], i, test_v1[i]);
                r1_result = false;
            }
        }
        assert_eq!(r1_result, true);
    }

    #[test]
    fn test_salsa20_expansion_256() {
        let k0 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let k1 = [201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216];
        let n = [101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116];
        let test_v1 = vec![
            69, 37, 68, 39, 41, 15,107,193,255,139,122, 6,170,233,217, 98,
            89,144,182,106, 21, 51,200, 65,239, 49,222, 34,215,114, 40,126,
            104,197, 7,225,197,153, 31, 2,102, 78, 76,176, 84,245,246,184,
            177,160,133,130, 6, 72,149,119,192,195,132,236,234,103,246, 74,
        ];
        let expansion = Salsa20::salsa20_expansion_256(k0, k1, n);
        let r1 = Salsa20::salsa20(expansion);
        assert_eq!(r1.to_vec().eq(&test_v1), true);
    }

    #[test]
    fn test_salsa20_encrypt_128() {
        let k = [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let v = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let m = vec![0; 512];
        // encrypt message
        let r1 = Salsa20::salsa20_20_128(k, v, m.clone());
        // decrypt message
        let m1 = Salsa20::salsa20_20_128(k, v, r1.clone());
        let d1 = xor_digest(r1.clone());
        let test_s1_xor_digest = vec![
            0xF7, 0xA2, 0x74, 0xD2, 0x68, 0x31, 0x67, 0x90, 0xA6, 0x7E, 0xC0, 0x58, 0xF4, 0x5C, 0x0F, 0x2A,
            0x06, 0x7A, 0x99, 0xFC, 0xDE, 0x62, 0x36, 0xC0, 0xCE, 0xF8, 0xE0, 0x56, 0x34, 0x9F, 0xE5, 0x4C,
            0x5F, 0x13, 0xAC, 0x74, 0xD2, 0x53, 0x95, 0x70, 0xFD, 0x34, 0xFE, 0xAB, 0x06, 0xC5, 0x72, 0x05,
            0x39, 0x49, 0xB5, 0x95, 0x85, 0x74, 0x21, 0x81, 0xA5, 0xA7, 0x60, 0x22, 0x3A, 0xFA, 0x22, 0xD4,
        ];
        // encrypted message
        assert_eq!(d1.eq(&test_s1_xor_digest), true);
        // decrypted message
        assert_eq!(m.eq(&m1), true);
    }

    #[test]
    fn test_salsa20_encrypt_256() {
        let k0 = [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let k1 = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let v = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let m = vec![0; 512];
        // encrypt message
        let r1 = Salsa20::salsa20_20_256(k0, k1, v, m.clone());
        // decrypt message
        let m1 = Salsa20::salsa20_20_256(k0, k1, v, r1.clone());
        let d1 = xor_digest(r1.clone());
        let test_s1_xor_digest = vec![
            0x50, 0xEC, 0x24, 0x85, 0x63, 0x7D, 0xB1, 0x9C, 0x6E, 0x79, 0x5E, 0x9C, 0x73, 0x93, 0x82, 0x80,
            0x6F, 0x6D, 0xB3, 0x20, 0xFE, 0x3D, 0x04, 0x44, 0xD5, 0x67, 0x07, 0xD7, 0xB4, 0x56, 0x45, 0x7F,
            0x3D, 0xB3, 0xE8, 0xD7, 0x06, 0x5A, 0xF3, 0x75, 0xA2, 0x25, 0xA7, 0x09, 0x51, 0xC8, 0xAB, 0x74,
            0x4E, 0xC4, 0xD5, 0x95, 0xE8, 0x52, 0x25, 0xF0, 0x8E, 0x2B, 0xC0, 0x3F, 0xE1, 0xC4, 0x25, 0x67,
        ];
        // encrypted message
        assert_eq!(d1.eq(&test_s1_xor_digest), true);
        // decrypted message
        assert_eq!(m.eq(&m1), true);
    }

}
