use security_task_1::playfair_cipher::*;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption() {
        let cipher = PlayFairCipher::new("KEYWORD".to_string());
        assert_eq!("GYIZSCOKCFBU", cipher.encrypt("HELLO WORLD".to_string()))        
    }
    
    #[test]
    fn test_decryption() {
        let cipher = PlayFairCipher::new("SECRET".to_string());
        assert_eq!("HELLOWORLD", cipher.decrypt("GYIZSCOKCFBU".to_string()))        
    }

    #[test]
    fn test_different_key() {
        let cipher = PlayFairCipher::new("SECRET".to_string());
        assert_ne!("HELLOWORLD", cipher.decrypt("GATLMZCLRQX".to_string()))        
    }
}