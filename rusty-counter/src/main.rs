extern crate hhlib_sys;

use hhlib_sys::device::Device;
use hhlib_sys::types::HydraHarpError;

fn main() -> Result<(), HydraHarpError> {
    for i in 0..8 {
        if let Ok(a) = Device::open_device(i) {
            println!("{:?}", a.get_base_resolution());
        }
    }
    Ok(())
}
