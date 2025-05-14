#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use risc0_zkvm::guest::{env, entry};
use tlsn_core::presentation::Presentation;
use tlsn_core::verify::verify_presentation;
use serde_json_core::from_slice;
use hex::decode;

entry!(main);

fn main() {
    //Read the wrapper JSON from host
    let raw: Vec<u8> = env::read();
    // Parse JSON to get the hex-encoded presentation
    let wrapper = from_slice::<serde_json_core::Value>(&raw)
        .expect("wrapper JSON parse failed").0;
    let data_hex = wrapper.get("data")
        .and_then(|v| v.as_str())
        .expect("no data field");
    //Decode hex → raw presentation bytes
    let pres_bytes = decode(data_hex).expect("hex decode failed");
    // Verify TLSNotary proof
    let record = verify_presentation(&pres_bytes)
        .expect("TLSn verification failed");
    //Extract HTTP body
    let body = &record.body;
    //Parse body JSON, extract args.score
    let body_wrapper = from_slice::<serde_json_core::Value>(body)
        .expect("body JSON parse failed").0;
    let score_str = body_wrapper.get("args")
        .and_then(|o| o.get("score"))
        .and_then(|v| v.as_str())
        .expect("no args.score");
    let score: u64 = score_str.parse().expect("invalid score");
    // Assert and commit
    assert!(score == 42, "unexpected score: {}", score);
    env::commit(&score.to_le_bytes());
}
