use security_task_2::des::DES;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encryption() {
        let des = DES::new();
        let key = "133457799BBCDFF1"; // 64-bit key in hex
        let plain_text = "0123456789ABCDEF"; // 64-bit block in hex
        let expected_cipher_text = "85E813540F0AB405"; // Known output from DES example
        let actual_cipher_text = des.encrypt(plain_text, key);
        assert_eq!(expected_cipher_text.to_uppercase(), actual_cipher_text.to_uppercase());
    }

    #[test]
    fn test_decryption() {
        let des = DES::new();
        let key = "133457799BBCDFF1";
        let cipher_text = "85E813540F0AB405";
        let expected_plain_text = "0123456789ABCDEF";
        let actual_plain_text = des.decrypt(cipher_text, key);
        assert_eq!(expected_plain_text.to_uppercase(), actual_plain_text.to_uppercase());
    }

    #[test]
    fn test_different_key() {
        let des = DES::new();
        let wrong_key = "A1B2C3D4E5F60708";
        let cipher_text = "85E813540F0AB405";
        let result = des.decrypt(cipher_text, wrong_key);
        assert_ne!("0123456789ABCDEF", result);
    }

    #[test]
    fn test_same_encryption_decryption() {
        let des = DES::new();
        let key = "AABB09182736CCDD";
        let plain_text = "1234567890ABCDEF";
        let encrypted = des.encrypt(plain_text, key);
        let decrypted = des.decrypt(&encrypted, key);
        assert_eq!(plain_text.to_uppercase(), decrypted.to_uppercase());
    }
}