#![no_main]
#![no_std]

extern crate alloc;

mod restrict;

use core::convert::TryInto;
use optee_utee::{ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println, Error, ErrorKind, Parameters, Result};
use core::ffi::c_void;
use proto::Command;
use crate::restrict::{restrict_check_invocation, restrict_cleanup, restrict_init, restrict_submit_token, RestrictHandle, RestrictToken};

static mut RESTRICT_HANDLE: Option<RestrictHandle> = None;


fn propriatery_fn(a: usize, b: &[u8]) -> usize {
    a + b.len()
}


#[ta_create]
fn create() -> Result<()> {
    trace_println!("[+] TA create");

    let mut handle = RestrictHandle {
        pub_key: optee_utee::TransientObject::null_object(),
    };
    restrict_init(&mut handle, &ALICE_MODULUS, &ALICE_EXPONENT)?;

    unsafe {
        RESTRICT_HANDLE = Some(handle);
    }

    Ok(())
}

#[ta_open_session]
fn open_session(_params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA open session");
    Ok(())
}

#[ta_close_session]
fn close_session() {
    trace_println!("[+] TA close session");
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] TA destroy");

    unsafe {
        match &mut RESTRICT_HANDLE {
            None => {}
            Some(handle) => {
                restrict_cleanup(handle);
            }
        }
    }
}

#[ta_invoke_command]
fn invoke_command(cmd_id: u32, params: &mut Parameters) -> Result<()> {
    match Command::from(cmd_id) {
        Command::SubmitToken => {
            let handle = unsafe { RESTRICT_HANDLE.as_mut().unwrap() };

            let mut token_memref = unsafe { params.0.as_memref()? };
            let mut sig_memref = unsafe { params.1.as_memref()? };

            // Validate token buffer size
            let token_buf = token_memref.buffer();
            if token_buf.len() != core::mem::size_of::<RestrictToken>() {
                return Err(ErrorKind::BadParameters.into());
            }

            // SAFETY: The buffer must be exactly the size of RestrictToken
            let token = unsafe {
                &*(token_buf.as_ptr() as *const RestrictToken)
            };

            let signature = sig_memref.buffer();

            return restrict_submit_token(handle, token, signature);
        }
        Command::InvokeProprietary => {
            let handle = unsafe { RESTRICT_HANDLE.as_mut().unwrap() };

            // Check invocation restriction
            restrict_check_invocation(handle)?;

            // Extract input/output memrefs
            let mut input = unsafe { params.0.as_memref()? };
            let mut output = unsafe { params.1.as_memref()? };

            let input_buf = input.buffer();
            let output_buf = output.buffer();

            // At minimum: 4 bytes for `int a`, and 1 for null terminator
            if input_buf.len() < 5 {
                return Err(ErrorKind::BadParameters.into());
            }

            // Parse `a`
            let a = i32::from_ne_bytes(input_buf[0..4].try_into().unwrap());

            // Get `b` as a null-terminated byte slice
            let b_max = &input_buf[4..];
            let b_len = match b_max.iter().position(|&c| c == 0) {
                Some(len) => len,
                None => return Err(ErrorKind::BadParameters.into()), // not null-terminated
            };

            let b = &b_max[..b_len]; // Exclude null terminator

            // Call proprietary function
            let result = propriatery_fn(a as usize, b);

            // Check output buffer size
            if output_buf.len() < 4 {
                return Err(ErrorKind::ShortBuffer.into());
            }

            // Write result
            output_buf[..4].copy_from_slice(&(result as i32).to_ne_bytes());

            return Ok(())
        }

        Command::Unknown => {}
    }

    Ok(())
}



include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
include!(concat!(env!("OUT_DIR"), "/gen.rs"));