use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::Signer;
use std::env;
use std::fs;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 3 {
        eprintln!("Usage: token-gen <private_key_path> <output_path> <int1> <int2> ...");
        std::process::exit(1);
    }

    let private_key_path = &args[0];
    let output_path = &args[1];

    let numbers: Vec<i32> = args[2..]
        .iter()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()
        .unwrap_or_else(|_| {
            eprintln!("All arguments after the output path must be valid integers.");
            std::process::exit(1);
        });

    let sequence_number: u32 = 1;
    let usage_limit: u32 = 1;

    // Serialize message: [seq, limit, payload...]
    let mut message = Vec::new();
    message.extend(&sequence_number.to_le_bytes());
    message.extend(&usage_limit.to_le_bytes());
    for &num in &numbers {
        message.extend(&num.to_le_bytes());
    }

    // Sign the message
    let private_key_pem =
        fs::read(private_key_path).unwrap_or_else(|_| panic!("Failed to read: {}", private_key_path));
    let rsa = Rsa::private_key_from_pem(&private_key_pem).expect("Invalid private key PEM");
    let key = PKey::from_rsa(rsa).expect("Failed to wrap RSA in PKey");
    let mut signer = Signer::new(MessageDigest::sha256(), &key).expect("Failed to create signer");
    signer.update(&message).expect("Failed to hash message");
    let signature = signer.sign_to_vec().expect("Signing failed");

    // Write [message || signature]
    let mut file = fs::File::create(output_path).expect("Failed to create output file");
    file.write_all(&message).unwrap();
    file.write_all(&signature).unwrap();

    println!("[+] {} written:", output_path);
    println!("    Payload: {:?} ({} integers)", numbers, numbers.len());
    println!("    Signature: {} bytes", signature.len());
}
