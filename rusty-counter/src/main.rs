extern crate hhlib_sys;

use hhlib_sys::device::Device;
use hhlib_sys::measurement::Measurement;
use hhlib_sys::types::{CTCStatus, HydraHarpError, MeasurementMode, ReferenceSource};
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
    for i in (0..1000) {
        let results = run_measurement_and_wait_till_finished(200, &mut dev)?;
        let mut measurement = Measurement::new(0);
        let channel_times = measurement.convert_values_T2(&results);
        let (singles, coincidences) = hhlib_sys::singles_and_two_way_coincidences(100000, &channel_times);
        println!("{:?}\n{:?}", singles, coincidences);
    }
    Ok(())
}

fn run_measurement_and_wait_till_finished(time: u32, dev: &mut Device) -> Result<Vec<u32>, HydraHarpError> {
    dev.start_measurement(time as i32)?;
    sleep_ms(time);
    while dev.get_CTC_status()? != hhlib_sys::types::CTCStatus::Running {
        sleep_ms(1);
    }
    let buffer_length = 128*1000;
    let mut buffer = vec![0u32; buffer_length];
    let num_read = dev.read_fifo(&mut buffer, (buffer_length) as i32)? as usize;
    Ok(buffer[..num_read].to_vec())
}
