#![no_std]
#![no_main]

extern crate alloc;

use n_time_model::ExecutionCounter;
use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{Error, ErrorKind, Parameters, Result};
use proto::Command;

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
        Command::Sort => one_time_sort(params),
        _ => Err(Error::new(ErrorKind::NotSupported)),
    }
}

fn one_time_sort(params: &mut Parameters) -> Result<()> {
    let counter = ExecutionCounter::new(EXECUTION_KEY, MAX_EXECUTIONS);
    counter.check_and_increment()?;

    let mut p0 = unsafe { params.0.as_memref().unwrap() };
    let array_ptr = p0.buffer().as_ptr() as *mut i32;
    let array_len = p0.buffer().len() / core::mem::size_of::<i32>();

    let array = unsafe { core::slice::from_raw_parts_mut(array_ptr, array_len) };
    trace_println!("[+] Sorting array of {} elements", array_len);
    sort_array(array);
    trace_println!("[+] Sort operation completed successfully");
    Ok(())
}

fn sort_array(array: &mut [i32]) {
    let len = array.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if array[j] > array[j + 1] {
                let temp = array[j];
                array[j] = array[j + 1];
                array[j + 1] = temp;
            }
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
