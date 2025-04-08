#![no_std]
#![no_main]

extern crate alloc;

use optee_utee::{
    trace_println, ta_create, ta_destroy, ta_open_session, ta_close_session, ta_invoke_command,
    Error, ErrorKind, Parameters, Result, ObjectStorageConstants, DataFlag, PersistentObject,
};
use proto::Command;

#[ta_create]
fn create() -> Result<()> {
    trace_println!("[+] Reset TA created");
    Ok(())
}

#[ta_open_session]
fn open_session(_params: &mut Parameters) -> Result<()> {
    trace_println!("[+] Session opened");
    Ok(())
}

#[ta_close_session]
fn close_session() {
    trace_println!("[+] Session closed");
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] Reset TA destroyed");
}

#[ta_invoke_command]
fn invoke_command(cmd_id: u32, params: &mut Parameters) -> Result<()> {
    match Command::from(cmd_id) {
        Command::Reset => reset_counter(params),
        Command::Unknown => {
            trace_println!("[!] Unknown command ID: {}", cmd_id);
            Err(Error::new(ErrorKind::NotSupported))
        }
    }
}

fn reset_counter(params: &mut Parameters) -> Result<()> {
    let mut p0 = unsafe { params.0.as_memref()? };
    let key = core::str::from_utf8(p0.buffer())
        .map_err(|_| Error::new(ErrorKind::BadParameters))?;

    trace_println!("[+] Resetting secure counter with key: {}", key);

    match PersistentObject::open(
        ObjectStorageConstants::Private,
        key.as_bytes(),
        DataFlag::ACCESS_READ | DataFlag::ACCESS_WRITE | DataFlag::ACCESS_WRITE_META,
    ) {
        Ok(mut obj) => {
            obj.close_and_delete()?;
            trace_println!("[+] Object deleted");
            Ok(())
        }
        Err(e) => {
            trace_println!("[!] Failed to open/delete object: {:?}", e);
            Err(e)
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
