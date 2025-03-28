use std::fs;
use std::io::Write;
use std::path::Path;
use openssl::rsa::Rsa;
use proto;
use optee_utee_build::{TaConfig, RustEdition, Error};

fn main() -> Result<(), Error> {
    let ta_config = TaConfig::new_default_with_cargo_env(proto::UUID)?;
    optee_utee_build::build(RustEdition::Before2024, ta_config)?;

    // Generate a new 2048-bit RSA keypair
    let keypair = Rsa::generate(2048).unwrap();

    // Save private key to file in PEM format
    let private_key_pem = keypair.private_key_to_pem().unwrap();
    fs::write("private_key.pem", private_key_pem).unwrap();

    // Extract public key components
    let modulus = keypair.n().to_vec(); // Public modulus (n)
    let exponent = keypair.e().to_vec(); // Public exponent (e)

    // Write components into a generated Rust file
    let out_dir = std::env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("gen.rs");
    let mut f = fs::File::create(&dest_path).unwrap();

    writeln!(f, "pub const ALICE_MODULUS: [u8; {}] = {:?};", modulus.len(), modulus).unwrap();
    writeln!(f, "pub const ALICE_EXPONENT: [u8; {}] = {:?};", exponent.len(), exponent).unwrap();

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
