#![no_std]

#[repr(u32)]
pub enum Command {
    SubmitToken = 0u32,
    InvokeProprietary = 1u32,
    Unknown = 2u32,
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::SubmitToken,
            1 => Command::InvokeProprietary,
            _ => Command::Unknown,
        }
    }
}

impl From<Command> for u32 {
    #[inline]
    fn from(cmd: Command) -> u32 {
        cmd as u32
    }
}

pub const UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/uuid.txt"));
