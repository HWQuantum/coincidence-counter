//! Bindings to the HydraHarp400 library from PicoQuant

#[macro_use]
extern crate num_derive;

pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod device;
pub mod measurement;
pub mod types;

use crate::bindings::*;
use crate::types::HydraHarpError::*;

/// Take a C function which returns an i32 error and return either Ok(type) or Err(ErrorCode)
#[macro_export]
macro_rules! error_enum_or_value {
    ($function:expr, $value:expr) => {
        match $function {
            0 => Ok($value),
            x => match num::FromPrimitive::from_i32(x) {
                None => Err(UnknownError),
                Some(e) => Err(e),
            },
        }
    };
}

/// Get the version of the HydraHarp dll
/// TODO: don't just return a string of the bytes
pub fn get_library_version() -> String {
    let mut version = [0i8; 8];
    unsafe {
        HH_GetLibraryVersion(version.as_mut_ptr());
    }
    return format!("{:?}", version);
}

/// convert from a coincidence channel pair (c1, c2) into an index
fn coincidence_channels_to_index(channels: (u8, u8)) -> usize {
    match channels {
        (0, 1) | (1, 0) => 0,
        (0, 2) | (2, 0) => 1,
        (0, 3) | (3, 0) => 2,
        (0, 4) | (4, 0) => 3,
        (0, 5) | (5, 0) => 4,
        (0, 6) | (6, 0) => 5,
        (0, 7) | (7, 0) => 6,
        (1, 2) | (2, 1) => 7,
        (1, 3) | (3, 1) => 8,
        (1, 4) | (4, 1) => 9,
        (1, 5) | (5, 1) => 10,
        (1, 6) | (6, 1) => 11,
        (1, 7) | (7, 1) => 12,
        (2, 3) | (3, 2) => 13,
        (2, 4) | (4, 2) => 14,
        (2, 5) | (5, 2) => 15,
        (2, 6) | (6, 2) => 16,
        (2, 7) | (7, 2) => 17,
        (3, 4) | (4, 3) => 18,
        (3, 5) | (5, 3) => 19,
        (3, 6) | (6, 3) => 20,
        (3, 7) | (7, 3) => 21,
        (4, 5) | (5, 4) => 22,
        (4, 6) | (6, 4) => 23,
        (4, 7) | (7, 4) => 24,
        (5, 6) | (6, 5) => 25,
        (5, 7) | (7, 5) => 26,
        (6, 7) | (7, 6) => 27,
        _ => 28,
    }
}

fn index_to_coincidence_channels(index: usize) -> (u8, u8) {
    match index {
        0 => (0, 1),
        1 => (0, 2),
        2 => (0, 3),
        3 => (0, 4),
        4 => (0, 5),
        5 => (0, 6),
        6 => (0, 7),
        7 => (1, 2),
        8 => (1, 3),
        9 => (1, 4),
        10 => (1, 5),
        11 => (1, 6),
        12 => (1, 7),
        13 => (2, 3),
        14 => (2, 4),
        15 => (2, 5),
        16 => (2, 6),
        17 => (2, 7),
        18 => (3, 4),
        19 => (3, 5),
        20 => (3, 6),
        21 => (3, 7),
        22 => (4, 5),
        23 => (4, 6),
        24 => (4, 7),
        25 => (5, 6),
        26 => (5, 7),
        27 => (6, 7),
        _ => (255, 255),
    }
}

/// Sort out a vector of channels and times into an array of singles and an array of coincidences
pub fn singles_and_two_way_coincidences(coincidence_window: u64, times: &[(u8, u64)]) -> ([u64; 8], [u64; 29]) {
    let mut singles = [0; 8];
    let mut coincidences = [0; 29];
    for (i, (c1, t1)) in times.iter().enumerate() {
        singles[c1] += 1;
        for (c2, t2) in times.iter().skip(i + 1) {
            if c1 != c2 {
                if t2 - t1 < coincidence_window {
                    coincidences[coincidence_channels_to_index((*c1, *c2))] += 1;
                } else {
                    break;
                }
            }
        }
    }
    (singles, coincidences)
}

#[cfg(test)]
mod tests {
    use super::bindings::*;
    #[test]
    fn it_works() {
        let devs = (0..2i32)
            .map(|x| crate::device::Device::open_device(x))
            .collect::<Vec<_>>();
        assert_eq!(
            devs,
            (0..1i32)
                .map(|_| Err::<crate::device::Device, _>(
                    crate::types::HydraHarpError::UnknownError
                ))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn get_library_version_works() {
        assert_eq!(crate::get_library_version(), String::new())
    }

}
