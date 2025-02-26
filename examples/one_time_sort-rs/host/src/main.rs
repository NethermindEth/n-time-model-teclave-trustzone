// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.

use optee_teec::{Context, Operation, ParamTmpRef, ParamNone, Session, Uuid};
use proto::{Command, UUID};

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(UUID).unwrap();
    let mut session = ctx.open_session(uuid)?;

    println!("\n----- One-Time Sort Trusted Application -----");
    
    // Create a sample array to sort
    let mut sample_array = [5, 3, 8, 1, 9, 2, 7, 4, 6, 0];
    println!("Original array: {:?}", sample_array);
    
    // Set up parameters - we're sending the array as a parameter
    let p0 = ParamTmpRef::new_input(&sample_array);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);

    // First attempt
    println!("\nAttempt #1 - First execution:");
    match session.invoke_command(Command::Sort as u32, &mut operation) {
        Ok(_) => {
            println!("Sort operation completed successfully!");
            println!("Array after sorting: {:?}", sample_array);
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    
    // Second attempt
    println!("\nAttempt #2 - Trying again:");
    match session.invoke_command(Command::Sort as u32, &mut operation) {
        Ok(_) => {
            println!("Sort operation completed successfully!");
            println!("Array after sorting: {:?}", sample_array);
        },
        Err(e) => {
            println!("Error: {:?}", e);
            println!("(Expected error - TA should only execute once)");
        }
    }

    println!("\nWe're done, close and release TEE resources");
    Ok(())
}