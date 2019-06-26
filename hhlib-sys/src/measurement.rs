//! Contains a struct which is one measurement run of the coincidence counter
//! should be used to extract the time information from the fifo data
use std::thread;
use std::time::Duration;

const OVERFLOW_PERIOD: u64 = 33554432;
const OVERFLOW_MASK: u32 = (63 << 25);
const TIME_MASK: u32 = (1<<24)-1;

/// Describes the different types a T2 value can have
pub enum T2Value {
    Time(u32),
    Overflow(u32),
    InternalSync(u8),
    ExternalSync(u8),
}

/// Converts a single 4 byte phrase into a T2Value
pub fn convert_T2_value(v: &u32) -> T2Value {
    use crate::measurement::T2Value::*;
    match (v & (1 << 31)) {
        0 => Time(v & TIME_MASK),
        _ => match (v & OVERFLOW_MASK) {
                OVERFLOW_MASK => Overflow(v & ((1<<24)-1)),
                _ => InternalSync(0)
            }
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
            time_overflow: overflow
        }
    }

    /// Convert a set of fifo outputs in T2 mode into a vector of times
    pub fn convert_values_T2(&mut self, input: &[u32]) -> Vec<u64> {
        use crate::measurement::T2Value::*;
        let mut times: Vec<u64> = Vec::with_capacity(input.len());
        for i in input {
            match convert_T2_value(i) {
                Time(t) => times.push(t as u64 + self.time_overflow),
                Overflow(t) => self.time_overflow += (t as u64) *OVERFLOW_PERIOD,
                _ => ()
            }
        }
        times
    }
}
