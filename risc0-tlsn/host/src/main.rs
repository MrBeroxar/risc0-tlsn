// host/src/main.rs

use anyhow::Result;
use methods::GUEST_ELF;                   // Embedded RISC-V guest from methods/build.rs
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::{fs, convert::TryInto};
use bincode;

fn main() -> Result<()> {
    // 1️⃣ Read the TLSNotary presentation you fetched earlier
    let proof = fs::read("data/httpbin_score.presentation.tlsn")
        .expect("Failed to read presentation file");

    // 2️⃣ Build a zkVM environment so that env::read() in the guest returns `proof`
    let env = ExecutorEnv::builder()
        .write(&proof)?   // feeds proof into guest
        .build()?;

    // 3️⃣ Invoke the prover, running your guest inside the zkVM
    let info = default_prover().prove(env, GUEST_ELF)?;
    let receipt = info.receipt;

    // 4️⃣ Pull the first 8 bytes out of the journal (your committed u64 score)
    let journal_bytes: &[u8] = receipt.journal.as_ref();
    let score = u64::from_le_bytes(journal_bytes[0..8].try_into().unwrap());
    println!("☑️  Score verified: {}", score);

    // 5️⃣ (Optional) serialize and save the full zk-proof for on-chain use
    let proof_bytes = bincode::serialize(&receipt)
        .expect("Failed to serialize receipt");
    fs::write("receipt.bin", proof_bytes)?;
    println!("📦  Wrote `receipt.bin`");

    Ok(())
}
