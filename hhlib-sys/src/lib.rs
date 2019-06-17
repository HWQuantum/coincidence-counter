#[macro_use]
extern crate num_derive;

pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod device;
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

#[cfg(test)]
mod tests {
    use super::bindings::*;
    #[test]
    fn it_works() {
        let f = crate::device::Device::open_device(7);
        assert_eq!(
            f,
            Err::<crate::device::Device, _>(crate::types::HydraHarpError::UnknownError)
        );
    }

    #[test]
    fn get_library_version_works() {
        assert_eq!(crate::get_library_version(), String::new())
    }

}
