extern crate hhlib_sys;

use hhlib_sys::device::Device;
use hhlib_sys::measurement::{Measurement, Measureable};
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
    let sleep_time: u32 = 4000;
    sleep_ms(200);
    for i in (0..1000) {
        let results = run_measurement_and_wait_till_finished(sleep_time, &mut dev)?;
        println!("Measurement length: {}", results.len());
        // let mut measurement = Measurement::new(0);
        // let mut channel_times = measurement.convert_values_T2(&results);
        //     channel_times.sort_by_key(|(_, t)| *t);
        //     let (singles, coincidences) =
        //         hhlib_sys::singles_and_two_way_coincidences(100000, &channel_times);
        //     println!("{:?}\n{:?}", singles, coincidences);
        // }
    }
    Ok(())
}

fn run_measurement_and_wait_till_finished(
    time: u32,
    dev: &mut Device,
) -> Result<Vec<u32>, HydraHarpError> {
    dev.start_measurement(time as i32)?;
    let buffer_length = 131072;
    let mut output = vec![0u32; 2 * buffer_length];
    let mut progress = 0;
    // Should perform the coincidence finding while in this loop; having it too long is bad for the buffer
    // maybe try storing the stuff in a VecDeque, for quick popping of values which aren't in time
    'measurement_loop: loop {
        if (dev.get_flags()? & hhlib_sys::bindings::FLAG_FIFOFULL as i32) != 0 {
            return Err(hhlib_sys::types::HydraHarpError::UnknownError);
        }
        let num_read = dev.read_fifo(
            &mut output[progress..(progress + buffer_length)],
            (buffer_length) as i32,
        )? as usize;
        if num_read > 0 {
            progress += num_read;
            if output.len() - progress < buffer_length {
                output.resize(output.len() + buffer_length, 0);
            }
        } else {
            if dev.get_CTC_status()? == hhlib_sys::types::CTCStatus::Ended {
                break 'measurement_loop;
            }
        }
    }
    output.resize(progress, 0);
    Ok(output)
}
