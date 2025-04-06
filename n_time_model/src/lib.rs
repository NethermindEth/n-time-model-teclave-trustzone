#![no_std]

//! # execution_counter
//!
//! A simple utility for OP-TEE Trusted Applications to enforce a maximum number of allowed
//! executions of a sensitive operation. It uses secure persistent storage to track how
//! many times an operation has been invoked, and prevents further execution once the
//! specified maximum is reached.
//!
//! ## Example
//!
//! ```no_run
//! use execution_counter::ExecutionCounter;
//!
//! const EXECUTION_KEY: &[u8] = b"my_exec_counter\0";
//! const MAX_EXECUTIONS: u32 = 1;
//!
//! let counter = ExecutionCounter::new(EXECUTION_KEY, MAX_EXECUTIONS);
//! counter.check_and_increment()?;
//!
//! // proceed with sensitive operation...
//! # Ok::<(), optee_utee::Error>(())
//! ```

extern crate optee_utee;

use optee_utee::{
    trace_println, DataFlag, ObjectStorageConstants, PersistentObject,
    Result, Error, ErrorKind,
};

/// A utility for enforcing a fixed number of allowed executions of a TA operation.
///
/// This is intended for use in OP-TEE Trusted Applications that want to implement
/// one-time or quota-limited execution logic. The counter is stored in secure persistent
/// storage under a user-defined key.
///
/// Call [`check_and_increment`](Self::check_and_increment) at the beginning of
/// a guarded operation. If the limit has not been exceeded, it will update the counter
/// and allow execution. If the maximum has been reached, it returns `AccessDenied`.
pub struct ExecutionCounter<'a> {
    /// Key used to persist the counter in secure storage.
    key: &'a [u8],
    /// Maximum allowed number of executions.
    max: u32,
}

impl<'a> ExecutionCounter<'a> {
    /// Creates a new `ExecutionCounter`.
    ///
    /// # Arguments
    ///
    /// * `key` - A null-terminated byte slice used as the key in secure storage.
    /// * `max` - The maximum number of allowed executions.
    ///
    /// # Returns
    ///
    /// A new instance of `ExecutionCounter`.
    pub const fn new(key: &'a [u8], max: u32) -> Self {
        Self { key, max }
    }

    /// Checks the current execution count and increments it if under the allowed limit.
    ///
    /// If the limit has been reached or exceeded, returns an `AccessDenied` error.
    ///
    /// # Returns
    ///
    /// `Ok(())` if execution is allowed and the counter was updated.
    ///
    /// `Err(Error { code: AccessDenied, .. })` if the execution limit has been reached.
    pub fn check_and_increment(&self) -> Result<()> {
        let current = self.get()?;

        if current >= self.max {
            trace_println!(
                "[+] Execution limit reached: {} of {}",
                current,
                self.max
            );
            return Err(Error::new(ErrorKind::AccessDenied));
        }

        trace_println!(
            "[+] Current count {} of {}, proceeding",
            current,
            self.max
        );

        self.set(current + 1)
    }

    /// Retrieves the current value of the execution counter from secure storage.
    ///
    /// If the counter object does not exist yet, returns 0 (first execution).
    fn get(&self) -> Result<u32> {
        match PersistentObject::open(
            ObjectStorageConstants::Private,
            self.key,
            DataFlag::ACCESS_READ,
        ) {
            Ok(object) => {
                let mut buf = [0u8; 4];
                object.read(&mut buf)?;
                Ok(u32::from_ne_bytes(buf))
            }
            Err(_) => {
                trace_println!("[+] No counter found, assuming first use");
                Ok(0)
            }
        }
    }

    /// Stores the updated execution count in secure persistent storage.
    ///
    /// Overwrites the existing object if it already exists.
    fn set(&self, value: u32) -> Result<()> {
        let data_flag = DataFlag::ACCESS_READ
            | DataFlag::ACCESS_WRITE
            | DataFlag::ACCESS_WRITE_META
            | DataFlag::OVERWRITE;

        let mut bytes = value.to_ne_bytes();

        match PersistentObject::create(
            ObjectStorageConstants::Private,
            self.key,
            data_flag,
            None,
            &mut bytes,
        ) {
            Ok(object) => {
                drop(object);
                trace_println!("[+] Execution counter updated to {}", value);
                Ok(())
            }
            Err(e) => {
                trace_println!("[!] Failed to store counter: {:?}", e);
                Err(e)
            }
        }
    }
}
