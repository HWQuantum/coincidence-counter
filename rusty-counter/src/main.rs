extern crate hhlib_sys;

use hhlib_sys::device::Device;
use hhlib_sys::types::{HydraHarpError, MeasurementMode, ReferenceSource};

fn main() -> Result<(), HydraHarpError> {
    for i in 0..8 {
        if let Ok(mut a) = Device::open_device(i) {
            a.initialise(MeasurementMode::Histogramming, ReferenceSource::Internal)?;
            println!("{:?}", a.get_base_resolution()?);
            println!("{}", a.set_histogram_length(0)?);
            a.close_device()?;
        }
    }
    Ok(())
}
