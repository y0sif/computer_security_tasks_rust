use security_task_1::monoalphabetic_cipher::{decrypt, encrypt, find_key};
use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption() {
        let key = "QWERTYUIOPASDFGHJKLZXCVBNM"; 
        let plaintext = "HELLO WORLD";
        let expected_ciphertext = "ITSSG VGKSR";

        let actual_ciphertext = encrypt(plaintext.to_string(), key.to_string());
        assert_eq!(expected_ciphertext.to_string(), actual_ciphertext);
    }

    #[test]
    fn test_decryption() {
        let key = "QWERTYUIOPASDFGHJKLZXCVBNM"; 
        let ciphertext = "ITSSG VGKSR";
        let expected_plaintext = "HELLO WORLD";

        let actual_plaintext = decrypt(ciphertext.to_string(), key.to_string());
        assert_eq!(expected_plaintext.to_string(), actual_plaintext);
    }

    #[test]
    fn test_find_key() {
        let main_plain = "meetmeafterthetogaparty".to_uppercase();
        let main_cipher = "phhwphdiwhuwkhwrjdsduwb".to_uppercase();
        let re = Regex::new("d.{3}hijk.{4}p.rs.u.w.{4}b.").unwrap();
        
        let key = find_key(main_plain, main_cipher);

        // Validate key format against expected pattern
        assert!(re.is_match(&key), "Key does not match expected pattern.")
   }
}