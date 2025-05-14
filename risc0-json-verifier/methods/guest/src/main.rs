#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use risc0_zkvm::guest::{env, entry};

entry!(main);

fn main() {
    //Read JSON response bytes
    let data: Vec<u8> = env::read();

    // Find `"score"`
    let key = b"\"score\"";
    let mut i = data
        .windows(key.len())
        .position(|w| w == key)
        .expect("no score key");
    i += key.len();

    //Skip to colon
    while i < data.len() && data[i] != b':' {
        i += 1;
    }
    assert!(i < data.len(), "no colon after score key");
    i += 1;

    //Skip whitespace
    while i < data.len() && (data[i] == b' ' || data[i] == b'\n' || data[i] == b'\r' || data[i] == b'\t') {
        i += 1;
    }

    //Find opening quote 
    assert!(data[i] == b'"', "no opening quote for score value");
    i += 1;
    let start = i;

    //Digits until next quote
    while i < data.len() && (data[i] >= b'0' && data[i] <= b'9') {
        i += 1;
    }
    let end = i;

    //Parse as u64
    let mut score: u64 = 0;
    for &b in &data[start..end] {
        score = score * 10 + (b - b'0') as u64;
    }
    assert!(score >= 5, "unexpected score: {}", score);
    
    env::commit(&score.to_le_bytes());
}
