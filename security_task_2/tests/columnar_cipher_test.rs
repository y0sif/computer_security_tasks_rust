use security_task_2::columnar_cipher::ColumnarCipher;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encryption_with_valid_key() {
        let cipher = ColumnarCipher::new();
        let key = vec![1, 3, 4, 2, 5];
        let encrypted_text = cipher.encrypt("COMPUTERSCIENCE", &key);
        assert_eq!("CTIPSCOEEMRNUCE", encrypted_text, "Encryption failed with key 13425.");
    }

    #[test]
    fn test_decryption_with_valid_key() {
        let cipher = ColumnarCipher::new();
        let key = vec![1, 3, 4, 2, 5];
        let decrypted_text = cipher.decrypt("CTIPSCOEEMRNUCE", &key);
        assert_eq!("COMPUTERSCIENCE", decrypted_text, "Decryption failed with key 13425.");
    }

    #[test]
    fn test_encryption_with_different_key() {
        let cipher = ColumnarCipher::new();
        let key = vec![5, 4, 3, 2, 1];
        let encrypted_text = cipher.encrypt("COMPUTERSCIENCE", &key);
        assert_ne!("CTIPSCOEEMRNUCE", encrypted_text, "Encryption with key 54321 should produce different output.");
    }

    #[test]
    fn test_decryption_with_different_key() {
        let cipher = ColumnarCipher::new();
        let key = vec![5, 4, 3, 2, 1];
        let decrypted_text = cipher.decrypt("CTIMRNPSEOEUCE", &key);
        assert_ne!("COMPUTERSCIENCE", decrypted_text, "Decryption should fail with incorrect key.");
    }

    #[test]
    fn test_key_analysis() {
        let cipher = ColumnarCipher::new();
        let key = cipher.analyse("COMPUTERSCIENCE", "CTIPSCOEEMRNUCE");
        assert_eq!(vec![1, 3, 4, 2, 5], key, "Key analysis failed.");
    }

    #[test]
    fn test_encryption_with_padding() {
        let cipher = ColumnarCipher::new();
        let key = vec![1, 2, 3, 4];
        let encrypted_text = cipher.encrypt("DATASECURITY", &key);
        assert_eq!("DSRAEITCTAUY", encrypted_text, "Encryption with padding failed.");
    }

    #[test]
    fn test_decryption_with_padding() {
        let cipher = ColumnarCipher::new();
        let key = vec![1, 2, 3, 4];
        let decrypted_text = cipher.decrypt("DSRXAEITCTAUY", &key);
        assert_eq!("DATASECURITYX", decrypted_text, "Decryption with padding failed.");
    }

    #[test]
    fn test_decryption() {
        let cipher = ColumnarCipher::new();
        let key = vec![1, 3, 2];
        let decrypted_text = cipher.decrypt("CPEMTOUR", &key);
        assert_eq!("COMPUTER", decrypted_text, "Decryption failed with key 132.");
    }

    #[test]
    fn test_empty_string_encryption() {
        let cipher = ColumnarCipher::new();
        let key = vec![1, 2, 3];
        assert_eq!("", cipher.encrypt("", &key), "Encryption of an empty string should return an empty string.");
    }

    #[test]
    fn test_empty_string_decryption() {
        let cipher = ColumnarCipher::new();
        let key = vec![1, 2, 3];
        assert_eq!("", cipher.decrypt("", &key), "Decryption of an empty string should return an empty string.");
    }
}