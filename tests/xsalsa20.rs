extern crate xsalsa20;
extern crate rustc_serialize;

#[cfg(test)]
mod tests {
    use xsalsa20::*;
    use xsalsa20::xsalsa20::*;
    use rustc_serialize::hex::FromHex;

    #[test]
    fn test_xsalsa20_encrypt_256() {
        let key = "a6a7251c1e72916d11c2cb214d3c252539121d8e234e652d651fa4c8cff88030"
            .from_hex()
            .expect("Failed to decode hex string.");
        let iv = "9e645a74e9e0a60d8243acd9177ab51a1beb8d5a2f5d700c"
            .from_hex()
            .expect("Failed to decode hex string.");
        let plaintext = "093c5e5585579625337bd3ab619d615760d8c5b224a85b1d0efe0eb8a7ee163abb0376529fcc09bab506c618e13ce777d82c3ae9d1a6f972d4160287cbfe60bf2130fc0a6ff6049d0a5c8a82f429231f008082e845d7e189d37f9ed2b464e6b919e6523a8c1210bd52a02a4c3fe406d3085f5068d1909eeeca6369abc981a42e87fe665583f0ab85ae71f6f84f528e6b397af86f6917d9754b7320dbdc2fea81496f2732f532ac78c4e9c6cfb18f8e9bdf74622eb126141416776971a84f94d156beaf67aecbf2ad412e76e66e8fad7633f5b6d7f3d64b5c6c69ce29003c6024465ae3b89be78e915d88b4b5621d"
            .from_hex()
            .expect("Failed to decode hex string.");
        let ciphertext = "b2af688e7d8fc4b508c05cc39dd583d6714322c64d7f3e63147aede2d9534934b04ff6f337b031815cd094bdbc6d7a92077dce709412286822ef0737ee47f6b7ffa22f9d53f11dd2b0a3bb9fc01d9a88f9d53c26e9365c2c3c063bc4840bfc812e4b80463e69d179530b25c158f543191cff993106511aa036043bbc75866ab7e34afc57e2cce4934a5faae6eabe4f221770183dd060467827c27a354159a081275a291f69d946d6fe28ed0b9ce08206cf484925a51b9498dbde178ddd3ae91a8581b91682d860f840782f6eea49dbb9bd721501d2c67122dea3b7283848c5f13e0c0de876bd227a856e4de593a3"
            .from_hex()
            .expect("Failed to decode hex string.");

        let mut k0: [u8; 16] = [0; 16];
        let mut k1: [u8; 16] = [0; 16];
        let mut v: [u8; 24] = [0; 24];
        for i in 0..16 {
            k0[i] = key[i];
            k1[i] = key[i+16];
        }
        for i in 0..24 {
            v[i] = iv[i];
        }

        let rs = XSalsa20::xsalsa20_20_256(k0, k1, v, plaintext.clone());

        assert_eq!(bytes_eq(rs.to_vec(), ciphertext), true);
    }

    #[test]
    fn test_random_u64() {
        let r1 = XSalsa20::random_u64([0; 32]);
        let t1 = XSalsa20::random_u64([0; 32]);
        let r2 = XSalsa20::random_u64([1; 32]);
        let t2 = XSalsa20::random_u64([1; 32]);
        assert_eq!(r1, t1);
        assert_eq!(r2, t2);
    }
}
