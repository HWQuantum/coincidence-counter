//! Contains a struct which is one measurement run of the coincidence counter
//! should be used to extract the time information from the fifo data
use std::thread;
use std::time::Duration;

use crate::types::{HydraHarpError, CTCStatus};

const OVERFLOW_PERIOD: u64 = 33554432;
const OVERFLOW_MASK: u32 = (63 << 25);
const TIME_MASK: u32 = (1 << 24) - 1;

/// Describes the different types a T2 value can have
pub enum T2Value {
    /// The u8 is the channel number, the u32 is the time
    Time(u8, u32),
    /// The u32 is the time
    Sync(u32),
    Overflow(u32),
    InternalSync(u8),
    ExternalSync(u8),
}

/// Converts a single 4 byte phrase into a T2Value
pub fn convert_T2_value(v: &u32) -> T2Value {
    use crate::measurement::T2Value::*;
    match (v & (1 << 31)) {
        0 => Time((v >> 25) as u8 & 63u8, v & TIME_MASK),
        _ => match (v & OVERFLOW_MASK) {
            OVERFLOW_MASK => Overflow(v & ((1 << 24) - 1)),
            0 => Sync(v & TIME_MASK),
            _ => InternalSync(0),
        },
    }
}

/// The measurement struct which keeps track of timining overflows
pub struct Measurement {
    pub time_overflow: u64,
}

impl Measurement {
    /// Define a new measurement, setting an overflow if needed
    pub fn new(overflow: u64) -> Measurement {
        Measurement {
            time_overflow: overflow,
        }
    }

    /// Convert a set of fifo outputs in T2 mode into a vector of channels and times
    /// Sets the sync channel to index zero and the rest higher
    pub fn convert_values_T2(&mut self, input: &[u32]) -> Vec<(u8, u64)> {
        use crate::measurement::T2Value::*;
        let mut times = Vec::with_capacity(input.len());
        for i in input {
            match convert_T2_value(i) {
                Time(c, t) => times.push((c+1, t as u64 + self.time_overflow)),
                Sync(t) => times.push((0, t as u64 + self.time_overflow)),
                Overflow(t) => self.time_overflow += (t as u64) * OVERFLOW_PERIOD,
                _ => (),
            }
        }
        times
    }
}

pub trait Measureable {
    fn start_measurement(&mut self, acquisition_time: i32) -> Result<(), HydraHarpError>;
    fn read_fifo(&mut self, buffer: &mut [u32], records_to_fetch: i32) -> Result<i32, HydraHarpError>;
    fn get_CTC_status(&self) -> Result<CTCStatus, HydraHarpError>;
}

pub struct TestMeasureable {
    time: u64,
}

impl TestMeasureable {
    fn new() -> TestMeasureable {
        TestMeasureable {
            time: 0
        }
    }
}

impl Measureable for TestMeasureable {
    fn start_measurement(&mut self, acquisition_time: i32) -> Result<(), HydraHarpError> {
        Ok(())
    }
    fn read_fifo(&mut self, buffer: &mut [u32], records_to_fetch: i32) -> Result<i32, HydraHarpError> {
        Ok(0)
    }
    fn get_CTC_status(&self) -> Result<CTCStatus, HydraHarpError> {
        Ok(CTCStatus::Ended)
    }
}
