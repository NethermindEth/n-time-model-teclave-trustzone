use optee_teec::{Context, Operation, ParamNone, ParamTmpRef, Uuid};
use proto::{Command, UUID};
use std::env;

fn main() -> optee_teec::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <counter_key>", args[0]);
        std::process::exit(1);
    }

    let key = args[1].as_bytes();

    // Set up OP-TEE session
    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(UUID).expect("Invalid UUID in proto");
    let mut session = ctx.open_session(uuid)?;

    // Send the key as a temp memref input
    let p0 = ParamTmpRef::new_input(key);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);

    println!("Resetting counter with key: '{}'", args[1]);

    match session.invoke_command(Command::Reset as u32, &mut operation) {
        Ok(()) => println!("✅ Counter deleted successfully."),
        Err(e) => println!("❌ Failed to delete counter: {:?}", e),
    }

    Ok(())
}
