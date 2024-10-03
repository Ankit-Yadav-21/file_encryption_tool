use std::fs;
use std::io::{self, Write};

// Function to generate a random key (16 bytes)
pub fn generate_key() -> Vec<u8> {
    let key: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect();
    println!("Generated Key: {:?}", key);
    key
}

// Function to parse a key from a comma-separated string
pub fn parse_key(key_str: &str) -> Result<Vec<u8>, &'static str> {
    let key: Vec<u8> = key_str.split(',')
        .map(|s| s.trim().parse().map_err(|_| "Invalid key format"))
        .collect::<Result<Vec<u8>, _>>()?;
    Ok(key)
}

// Function to encrypt the file using XOR (for demonstration purposes)
pub fn encrypt_decrypt_file(input_file: &str, output_file: &str, key: &[u8]) -> io::Result<()> {
    let data = fs::read(input_file)?;
    let encrypted_data: Vec<u8> = data.iter().enumerate().map(|(i, &byte)| byte ^ key[i % key.len()]).collect();
    let mut file = fs::File::create(output_file)?;
    file.write_all(&encrypted_data)?;
    println!("File encrypted successfully.");
    Ok(())
}

