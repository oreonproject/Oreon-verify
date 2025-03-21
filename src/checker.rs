#![allow(unused)]
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};

fn hash_iso(file_path: &str) -> Result<String, std::io::Error> {
    println!("[INFO] Hashing ISO file...");
    let mut file = File::open(file_path)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    let hash_result = hasher.finalize();
    Ok(format!("{hash_result:x}"))
}

pub fn check(iso_file_path: &str, hash: &str) -> Result<bool, std::io::Error> {
    println!("[INFO] Checking ISO file integrity...");
    let iso_file_hash = hash_iso(iso_file_path)?;
    Ok(iso_file_hash == hash)
}
