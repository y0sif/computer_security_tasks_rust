use std::collections::HashMap;

pub struct ColumnarCipher;

impl ColumnarCipher {
    pub fn new() -> Self {
        ColumnarCipher {}
    }

    pub fn analyse(&self, plain_text: &str, cipher_text: &str) -> Vec<usize> {
        // TODO: Analyze the plainText and cipherText to determine the key(s)
        
        Vec::new() // Placeholder return
    }

    pub fn decrypt(&self, cipher_text: &str, key: &Vec<usize>) -> String {
        let cipher_size = cipher_text.len();
        let rows = (cipher_size as f64 / key.len() as f64).ceil() as usize;
        let mut grid = vec![vec!['\0'; key.len()]; rows];
        let mut count = 0;

        let mut key_map = HashMap::new();
        for i in 0..key.len() {
            key_map.insert(key[i] - 1, i);
        }

        let remaining_cols = cipher_size % key.len();
        for i in 0..key.len() {
            for j in 0..rows {
                if remaining_cols != 0 && j == rows - 1 && key_map.get(&i).unwrap() >= &remaining_cols {
                    continue;
                }
                grid[j][*key_map.get(&i).unwrap()] = cipher_text.chars().nth(count).unwrap();
                count += 1;
            }
        }

        let mut result = String::new();
        for i in 0..rows {
            for j in 0..key.len() {
                result.push(grid[i][j]);
            }
        }
        result.to_uppercase().trim().to_string()
    }

    pub fn encrypt(&self, plain_text: &str, key: &Vec<usize>) -> String {
        let pt_size = plain_text.len();
        let rows = (pt_size as f64 / key.len() as f64).ceil() as usize;
        let mut grid = vec![vec!['\0'; key.len()]; rows];
        let mut count = 0;

        let plain_chars: Vec<char> = plain_text.chars().collect();
        for i in 0..rows {
            for j in 0..key.len() {
                if count >= pt_size {
                    grid[i][j] = 'x';
                } else {
                    grid[i][j] = plain_chars[count];
                    count += 1;
                }
            }
        }

        let mut key_map = HashMap::new();
        for i in 0..key.len() {
            key_map.insert(key[i] - 1, i);
        }

        let mut cipher_text = String::new();
        for i in 0..key.len() {
            for j in 0..rows {
                cipher_text.push(grid[j][*key_map.get(&i).unwrap()].to_ascii_uppercase());
            }
        }
        cipher_text
    }
}