use std::{collections::HashMap, hash::Hash};

// TODO: Implement this method to generate a substitution map from A-Z using the provided key
fn generate_encryption_map(key: String) -> HashMap<char, char> {
    let encryption_map = HashMap::new();
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let key = key.to_uppercase();
    
    // Students should complete this loop
    for i in 0..alphabet.len() {
        // encryptionMap // Hint: Map plaintext letter to cipher letter
    }

    encryption_map
}

// TODO: Implement this method to reverse the encryption map (ciphertext -> plaintext)
fn generate_decryption_map(key: String) -> HashMap<char, char> {
    let decryption_map = HashMap::new();
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let key = key.to_uppercase();

    // Students should complete this loop
    for i in 0..alphabet.len() {
        // decryptionMap // Hint: Reverse mapping
    }
    
    decryption_map
}

pub fn encrypt(plaintext: String, key: String) -> String {
    let encryption_map = generate_decryption_map(key);
    let plaintext = plaintext.to_uppercase();
    let encryption_text = String::new();
    
    for c in plaintext.chars() {
        // TODO: Use the encryption map to convert each letter
    }
    
    encryption_text
}

pub fn decrypt(ciphertext: String, key: String) -> String {
    let mut decryption_map = generate_decryption_map(key);
    let ciphertext = ciphertext.to_uppercase();
    let decrypted_text = String::new();
    
    for c in ciphertext.chars() {
        // TODO: Use the decryption map to convert each letter
    }
    
    decrypted_text
}

pub fn find_key(plaintext: String, ciphertext: String) -> String {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let key_map = vec![' '; 26];

    let plaintext = plaintext.to_uppercase();
    let ciphertext = ciphertext.to_uppercase();
    
    for (plain_char, cipher_char) in plaintext.chars().zip(ciphertext.chars()) {
        if plain_char.is_alphabetic() {
            let index = alphabet.find(plain_char).unwrap();
            // TODO: Ensure each letter is mapped only once

        } 
    }
    
    key_map.into_iter().collect::<String>()
}