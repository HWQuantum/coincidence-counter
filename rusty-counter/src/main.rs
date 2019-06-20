extern crate hhlib_sys;

use hhlib_sys::device::Device;
use hhlib_sys::types::{HydraHarpError, MeasurementMode, ReferenceSource, CTCStatus};
use std::thread::sleep_ms;

fn main() -> Result<(), HydraHarpError> {
    let mut dev = Device::open_device(0)?;
    dev.initialise(MeasurementMode::T2, ReferenceSource::Internal)?;
    dev.calibrate()?;
    dev.set_sync_divider(1)?;
    dev.set_sync_CFD(50, 10)?;
    dev.set_sync_channel_offset(-5000)?;
    let num_channels = dev.get_number_of_input_channels()?;
    for i in (0..num_channels) {
        dev.set_input_CFD(i, 50, 10)?;
        dev.set_input_channel_offset(i, 0)?;
    }
    sleep_ms(200);
    dev.start_measurement(1000)?;
    let mut buffer = vec![0u32; 1000];
    sleep_ms(1000);
    let num_read = dev.read_fifo(&mut buffer, 128*2)? as usize;
    println!("{:?}", &buffer[..num_read]);
    Ok(())
}
