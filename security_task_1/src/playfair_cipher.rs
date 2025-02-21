use std::collections::HashSet;
pub struct PlayFairCipher {
    key_matrix: Vec<Vec<char>>,
}

impl PlayFairCipher {
    pub fn new(key: String) -> Self {
        PlayFairCipher {
            key_matrix: Self::generate_key_matrix(key)
        }
    }

    // Generates the 5x5 key matrix for Playfair Cipher
    fn generate_key_matrix(key: String) -> Vec<Vec<char>> {
        let mut used = HashSet::new();
        let k = key.to_uppercase().replace("[^A-Z]", "").replace("J", "I");
        
        for c in key.chars() {
            used.insert(c);
        }
        
        for c in 'A'..'Z' {
            if c != 'J' {
                used.insert(c);
            }
        }
        
        let mut matrix = vec![vec![]];
        let mut it = used.iter();
        for i in 0..5 {
            for j in 0..5 {
                matrix[i][j] = *it.next().unwrap();
            }
        }
        
        matrix
    }

    // Prepares the text by removing invalid characters, replacing 'J' with 'I', and ensuring even length
    fn prepare_text(text: String) -> String {
        let text= text.to_uppercase().replace("[^A-Z]", "").replace("J", "I");
        let text = text.as_bytes();
        let mut sb = String::new();
        
        for i in 0..text.len() {
            sb.push(text[i] as char);
            // Insert 'X' if two consecutive letters are the same
            if i < text.len() - 1 && text[i] == text[i+1] && text[i] != b'X' {
                sb.push('X');
            }
        }
        
        // Ensure even length
        if sb.len() % 2 != 0 {
            sb.push('X');
        }
        
        sb
    }

    // TODO: Implement this method to find the position of a character in the key matrix
    fn find_position(c: char) -> Option<Vec<usize>> {
        // Students should complete this part
        None
    }

    // Encrypts the given plaintext using the Playfair cipher algorithm
    pub fn encrypt(&self, text: String) -> String {
        let text = Self::prepare_text(text);
        let mut encrypted_text = String::new();
        let text = text.as_bytes();

        for i in (0..text.len()).step_by(2) { 
            let pos1 = Self::find_position(text[i] as char);
            let pos2 = Self::find_position(text[i + 1] as char);
            
            if let (Some(pos1), Some(pos2)) = (pos1, pos2) {
                if pos1[0] == pos2[0] { // Same row
                    encrypted_text.push(self.key_matrix[pos1[0]][(pos1[1] + 1) % 5]);
                    encrypted_text.push(self.key_matrix[pos2[0]][(pos2[1] + 1) % 5]);
                }else if pos1[1] == pos2[1] { // Same column
                    encrypted_text.push(self.key_matrix[(pos1[0] + 1) % 5][pos1[1]]);
                    encrypted_text.push(self.key_matrix[(pos2[0] + 1) % 5][pos2[1]]);
                }else { // Rectangle swap
                    encrypted_text.push(self.key_matrix[pos1[0]][pos2[1]]);
                    encrypted_text.push(self.key_matrix[pos2[0]][pos1[1]]);
                }
            }
        }
        
        encrypted_text    
    }
    
    // TODO: Implement this method to decrypt the ciphertext back to plaintext
    pub fn decrypt(&self, text: String) -> String {
        // Students should complete this part
        String::new()
    }
}
