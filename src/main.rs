#![allow(unused)]
use clap::Parser;
mod checker;
#[derive(Parser, Debug)]
#[command[version = "1.0", long_about = None, about = "Verify the integrity of ISO files."]]
struct Args {
    #[arg(short = 'f', long = "file")]
    file_name: Option<String>,
    #[arg(short = 'H', long = "hash")]
    hash: Option<String>,
}
fn main() {
    let args = Args::parse();
    println!("[INFO] Validating arguments.");
    match validate_args(&args) {
        Ok(()) => {
            println!("[INFO] Arguments seem correct.");
        }
        Err(msg) => panic!("[ERR] {msg}")
    };
    match checker::check(&args.file_name.unwrap(), &args.hash.unwrap()) {
        true => println!("ISO file is fine!"),
        false => println!("ISO file is corrupted!")
    }
}

fn validate_args(args: &Args) -> Result<(), &str> {
    if !args.file_name.is_some() {
        Err("ISO file argument (-f or --file) is required! Please specify an ISO file to check.")
    }
    else {
        if !args.hash.is_some() {
            Err("Hash argument (-H or --hash) is required! please provide a hash to check against,   Get a hash from https://oreonproject.org")
        }
        else {
            Ok(())
        }
    }
}