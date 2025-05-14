fn main() {
    println!(
        "cargo:rustc-env=CFLAGS_riscv32im-risc0-zkvm-elf=-DOPENSSL_32_BIT -Dcrypto_word_t=uint32_t"
    );

    risc0_build::embed_methods();
}
