use anyhow::Result;
use bincode;                     
use methods::GUEST_CODE_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::{fs, convert::TryInto};

fn main() -> Result<()> {
    //Read JSON blob
    let input = fs::read("data/httpbin_score.json")
        .expect("Failed to read data/httpbin_score.json");

    //Build zkVM env 
    let env = ExecutorEnv::builder()
        .write(&input).unwrap()
        .build().unwrap();

    //Run prover
    let info = default_prover()
        .prove(env, GUEST_CODE_ELF)?;  
    let receipt = info.receipt;

    // Extract raw bytes from Journal
    let journal_bytes: &[u8] = receipt.journal.as_ref();
    let score = u64::from_le_bytes(journal_bytes[0..8].try_into().unwrap());
    println!("Score verified: {}", score);

    //Serialize Receipt 
    let proof_bytes = bincode::serialize(&receipt)
        .expect("Failed to serialize Receipt");
    fs::write("receipt.bin", proof_bytes)?;
    println!("Wrote receipt.bin");

    Ok(())
}
