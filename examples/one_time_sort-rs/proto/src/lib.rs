// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.

#![no_std]

pub enum Command {
    Sort,
    Unknown,
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::Sort,
            _ => Command::Unknown,
        }
    }
}

pub const UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/uuid.txt"));