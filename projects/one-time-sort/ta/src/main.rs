#![no_std]
#![no_main]

extern crate alloc;

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{DataFlag, ObjectStorageConstants, PersistentObject};
use optee_utee::{Error, ErrorKind, Parameters, Result};
use proto::Command;

// Execution limit as a global constant
const MAX_EXECUTIONS: u32 = 1;
const EXECUTION_KEY: &[u8] = b"one_time_sort_counter\0";

#[ta_create]
fn create() -> Result<()> {
    trace_println!("[+] One-Time Sort TA create");
    Ok(())
}

#[ta_open_session]
fn open_session(_params: &mut Parameters) -> Result<()> {
    trace_println!("[+] One-Time Sort TA open session");
    Ok(())
}

#[ta_close_session]
fn close_session() {
    trace_println!("[+] One-Time Sort TA close session");
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] One-Time Sort TA destroy");
}

#[ta_invoke_command]
fn invoke_command(cmd_id: u32, params: &mut Parameters) -> Result<()> {
    trace_println!("[+] One-Time Sort TA invoke command");
    match Command::from(cmd_id) {
        Command::Sort => {
            return one_time_sort(params);
        }
        _ => {
            return Err(Error::new(ErrorKind::NotSupported));
        }
    }
}

fn one_time_sort(params: &mut Parameters) -> Result<()> {
    // Get current execution count and check if we can proceed
    let current_count = get_execution_count()?;
    
    if current_count >= MAX_EXECUTIONS {
        trace_println!("[+] TA has already been executed {} times (max: {})", 
                      current_count, MAX_EXECUTIONS);
        return Err(Error::new(ErrorKind::AccessDenied));
    }
    
    trace_println!("[+] Execution count: {} of {}, proceeding with sort", 
                  current_count, MAX_EXECUTIONS);
    
    // Get the array from parameters
    let mut p0 = unsafe { params.0.as_memref().unwrap() };
    let array_ptr = p0.buffer().as_ptr() as *mut i32;
    let array_len = p0.buffer().len() / core::mem::size_of::<i32>();
    
    // Create a mutable slice from the raw pointer
    let array = unsafe { core::slice::from_raw_parts_mut(array_ptr, array_len) };
    
    trace_println!("[+] Sorting array of {} elements", array_len);
    
    // Sort the array
    sort_array(array);
    
    // Increment the execution counter
    increment_counter(current_count)?;
    
    trace_println!("[+] Sort operation completed successfully");
    Ok(())
}

fn sort_array(array: &mut [i32]) {
    let len = array.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if array[j] > array[j + 1] {
                // Swap elements
                let temp = array[j];
                array[j] = array[j + 1];
                array[j + 1] = temp;
            }
        }
    }
}

fn get_execution_count() -> Result<u32> {
    // Try to open our execution counter
    match PersistentObject::open(
        ObjectStorageConstants::Private,
        EXECUTION_KEY,
        DataFlag::ACCESS_READ,
    ) {
        // If we can open it, read the current count
        Ok(object) => {
            let mut count_buf = [0u8; 4];
            object.read(&mut count_buf)?;
            let count = u32::from_ne_bytes(count_buf);
            trace_println!("[+] Found existing execution count: {}", count);
            Ok(count)
        },
        // If not found, this is the first execution (count = 0)
        Err(_) => {
            trace_println!("[+] No execution count found, assuming first execution");
            Ok(0)
        },
    }
}

fn increment_counter(current_count: u32) -> Result<()> {
    let new_count = current_count + 1;
    trace_println!("[+] Incrementing execution count: {} -> {}", current_count, new_count);
    
    // Flags for the counter object
    let data_flag = DataFlag::ACCESS_READ
        | DataFlag::ACCESS_WRITE
        | DataFlag::ACCESS_WRITE_META
        | DataFlag::OVERWRITE;
    
    // Convert count to bytes
    let mut count_bytes = new_count.to_ne_bytes();  // Added 'mut' here
    
    // Create or overwrite the counter object
    match PersistentObject::create(
        ObjectStorageConstants::Private,
        EXECUTION_KEY,
        data_flag,
        None,
        &mut count_bytes,
    ) {
        Ok(object) => {
            drop(object);
            trace_println!("[+] Execution count updated successfully");
            Ok(())
        },
        Err(e) => {
            trace_println!("[!] Failed to update execution count: {:?}", e);
            Err(e)
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));