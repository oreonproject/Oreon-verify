#![allow(unused)]
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};

fn hash_iso(file_path: &str) -> Result<String, std::io::Error> {
    println!("[INFO] Hashing ISO file...");
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buff = vec![0u8; 4 * 1024 * 1024];
    while let Ok(bytes_read) = reader.read(&mut buff) {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buff[..bytes_read]);
        buff.clear();
    }
    let hash_result = hasher.finalize();
    Ok(format!("{:x}", hash_result))
}

pub fn check(iso_file_path: &str, hash: &str) -> Result<bool, std::io::Error> {
    println!("[INFO] Checking ISO file integrity...");
    let iso_file_hash = hash_iso(iso_file_path)?;
    Ok(iso_file_hash == hash)
}
