use optee_teec::{Context, Operation, ParamNone, ParamTmpRef, Uuid};
use proto::{Command, UUID};
use std::env;
use std::mem;

fn main() -> optee_teec::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: host <int1> <int2> ...");
        return Ok(());
    }

    let mut numbers: Vec<i32> = match args.iter().map(|s| s.parse()).collect() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("All arguments must be valid integers.");
            return Ok(());
        }
    };

    println!("Original array: {:?}", numbers);

    // Transmute &[i32] -> &[u8]
    let byte_slice: &mut [u8] = unsafe {
        core::slice::from_raw_parts_mut(
            numbers.as_mut_ptr() as *mut u8,
            numbers.len() * mem::size_of::<i32>(),
        )
    };

    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(UUID).unwrap();
    let mut session = ctx.open_session(uuid)?;

    let p0 = ParamTmpRef::new_output(byte_slice);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);

    println!("\nAttempting TA sort...");
    match session.invoke_command(Command::Sort as u32, &mut operation) {
        Ok(_) => {
            println!("Sort operation completed successfully!");
            println!("Sorted array: {:?}", numbers);
        }
        Err(e) => {
            println!("Error from TA: {:?}", e);
            println!("(This is expected if TA was already executed once)");
        }
    }

    println!("\nDone.");
    Ok(())
}
