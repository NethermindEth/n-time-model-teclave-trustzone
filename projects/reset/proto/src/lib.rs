#![no_std]

/// Commands supported by this TA.
pub enum Command {
    /// Deletes the counter stored under a given key.
    Reset,
    /// Fallback for unsupported commands.
    Unknown,
}

impl From<u32> for Command {
    fn from(value: u32) -> Command {
        match value {
            0 => Command::Reset,
            _ => Command::Unknown,
        }
    }
}

/// UUID for this TA — expected to be generated at build time and written to `uuid.txt`
pub const UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/uuid.txt"));
