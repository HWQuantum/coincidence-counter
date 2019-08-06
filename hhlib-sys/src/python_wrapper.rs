use crate::device::Device;
use crate::measurement::{Measurement, Measureable};
use crate::types::convert_hydra_harp_result;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;

fn unwrap_or_value_error<T>(x: i32) -> PyResult<T>
where
    T: num::FromPrimitive,
{
    match num::FromPrimitive::from_i32(x) {
        Some(r) => Ok(r),
        _ => Err(exceptions::ValueError.into()),
    }
}

#[pyfunction]
pub fn open_device(id: i32) -> PyResult<Device> {
    convert_hydra_harp_result(Device::open_device(id))
}
#[pyfunction]
/// Try to close this device
pub fn close_device(d: &mut Device) -> PyResult<()> {
    convert_hydra_harp_result(d.close_device())
}

#[pyfunction]
/// Initialise the device. Should be done before other functions are run
pub fn initialise(d: &mut Device, mode: i32, ref_source: i32) -> PyResult<()> {
    convert_hydra_harp_result(d.initialise(
        unwrap_or_value_error(mode)?,
        unwrap_or_value_error(ref_source)?,
    ))
}

#[pyfunction]
pub fn get_base_resolution(d: &mut Device) -> PyResult<(f64, i32)> {
    convert_hydra_harp_result(d.get_base_resolution())
}

#[pyfunction]
pub fn get_number_of_input_channels(d: &mut Device) -> PyResult<i32> {
    convert_hydra_harp_result(d.get_number_of_input_channels())
}

#[pyfunction]
pub fn calibrate(d: &mut Device) -> PyResult<()> {
    convert_hydra_harp_result(d.calibrate())
}

#[pyfunction]
pub fn set_sync_divider(d: &mut Device, divisions: i32) -> PyResult<()> {
    convert_hydra_harp_result(d.set_sync_divider(divisions))
}

#[pyfunction]
pub fn set_sync_CFD(d: &mut Device, level: i32, zerox: i32) -> PyResult<()> {
    convert_hydra_harp_result(d.set_sync_CFD(level, zerox))
}

#[pyfunction]
pub fn set_sync_channel_offset(d: &mut Device, offset: i32) -> PyResult<()> {
    convert_hydra_harp_result(d.set_sync_channel_offset(offset))
}

#[pyfunction]
pub fn set_input_CFD(d: &mut Device, channel: i32, level: i32, zerox: i32) -> PyResult<()> {
    convert_hydra_harp_result(d.set_input_CFD(channel, level, zerox))
}

#[pyfunction]
pub fn set_input_channel_offset(d: &mut Device, channel: i32, offset: i32) -> PyResult<()> {
    convert_hydra_harp_result(d.set_input_channel_offset(channel, offset))
}

#[pyfunction]
pub fn set_input_channel_enabled(d: &mut Device, channel: i32, enabled: bool) -> PyResult<()> {
    convert_hydra_harp_result(d.set_input_channel_enabled(channel, enabled))
}

#[pyfunction]
pub fn set_stop_overflow(d: &mut Device, stop_ofl: bool, stopcount: u32) -> PyResult<()> {
    convert_hydra_harp_result(d.set_stop_overflow(stop_ofl, stopcount))
}

#[pyfunction]
pub fn set_binning(d: &mut Device, binning: i32) -> PyResult<()> {
    convert_hydra_harp_result(d.set_binning(binning))
}

#[pyfunction]
pub fn set_offset(d: &mut Device, offset: i32) -> PyResult<()> {
    convert_hydra_harp_result(d.set_offset(offset))
}

// /// Set the histogram length. Returns the actual length calculated as `1024*(2^lencode)`
// pub fn set_histogram_length(&mut self, length: i32) -> Result<i32, HydraHarpError> {
//     let mut actual_length: i32 = 0;
//     let return_val = error_enum_or_value! {
//         unsafe {
//             HH_SetHistoLen(self.id, length, &mut actual_length as *mut i32)
//         },
//         actual_length
//     };
//     if let Ok(len) = return_val {
//         self.histogram_length = Some(len as usize);
//     }
//     return_val
// }

/// Clear the histogram memory
#[pyfunction]
pub fn clear_histogram_memory(d: &mut Device) -> PyResult<()> {
    convert_hydra_harp_result(d.clear_histogram_memory())
}

// /// Set the measurement control code and edges
// pub fn set_measurement_control(
//     &mut self,
//     control: MeasurementControl,
//     start_edge: EdgeSelection,
//     stop_edge: EdgeSelection,
// ) -> Result<(), HydraHarpError> {
//     error_enum_or_value! {
//         unsafe {
//             HH_SetMeasControl(self.id, num::ToPrimitive::to_i32(&control).unwrap(),
//                               num::ToPrimitive::to_i32(&start_edge).unwrap(),
//                               num::ToPrimitive::to_i32(&stop_edge).unwrap())
//         },
//         ()
//     }
// }

#[pyfunction]
pub fn start_measurement(d: &mut Device, acquisition_time: i32) -> PyResult<()> {
    convert_hydra_harp_result(d.start_measurement(acquisition_time))
}

#[pyfunction]
pub fn stop_measurement(d: &mut Device) -> PyResult<()> {
    convert_hydra_harp_result(d.stop_measurement())
}

#[pyfunction]
pub fn get_CTC_status(d: &mut Device) -> PyResult<i32> {
    match convert_hydra_harp_result(d.get_CTC_status()) {
        Ok(x) => Ok(num::ToPrimitive::to_i32(&x).unwrap()),
        Err(e) => Err(e),
    }
}

// /// Get the histogram from the device. Returns the error `HistogramLengthNotKnown` if
// /// `self.histogram_length = None`. If clear is true then the acquisiton buffer is cleared upon reading,
// /// otherwise it isn't
// pub fn get_histogram(&mut self, channel: i32, clear: bool) -> Result<Vec<u32>, HydraHarpError> {
//     if let Some(histogram_length) = self.histogram_length {
//         // let mut histogram_data: Vec<u32> = Vec::with_capacity(histogram_length);
//         let mut histogram_data: Vec<u32> = vec![0; histogram_length];
//         error_enum_or_value! {
//             unsafe {
//                 HH_GetHistogram(self.id, histogram_data.as_mut_ptr(), channel, clear as i32)
//             },
//             histogram_data
//         }
//     } else {
//         Err(HydraHarpError::HistogramLengthNotKnown)
//     }
// }

#[pyfunction]
pub fn get_resolution(d: &mut Device) -> PyResult<f64> {
    convert_hydra_harp_result(d.get_resolution())
}

#[pyfunction]
pub fn get_sync_rate(d: &mut Device) -> PyResult<i32> {
    convert_hydra_harp_result(d.get_sync_rate())
}

#[pyfunction]
pub fn get_count_rate(d: &mut Device, channel: i32) -> PyResult<i32> {
    convert_hydra_harp_result(d.get_count_rate(channel))
}
// /// get the current count rate
// /// allow at least 100ms after initialise or set_sync_divider to get a stable meter reading
// /// wait at least 100ms to get a new reading. This is the gate time of the counters
// pub fn get_count_rate(&self, channel: i32) -> Result<i32, HydraHarpError> {
//     let mut count_rate: i32 = 0;
//     error_enum_or_value! {
//         unsafe {
//             HH_GetCountRate(self.id, channel, &mut count_rate as *mut i32)
//         },
//         count_rate
//     }
// }

// /// get the flags. Use the `FLAG_*` variables to extract the different flags
// pub fn get_flags(&self) -> Result<i32, HydraHarpError> {
//     let mut flags: i32 = 0;
//     error_enum_or_value! {
//         unsafe {
//             HH_GetFlags(self.id, &mut flags as *mut i32)
//         },
//         flags
//     }
// }

#[pyfunction]
pub fn get_elapsed_measurement_time(d: &mut Device) -> PyResult<f64> {
    convert_hydra_harp_result(d.get_elapsed_measurement_time())
}

// /// get the warnings encoded bitwise
// pub fn get_warnings(&self) -> Result<i32, HydraHarpError> {
//     let mut warnings: i32 = 0;
//     error_enum_or_value! {
//         unsafe {
//             HH_GetWarnings(self.id, &mut warnings as *mut i32)
//         },
//         warnings
//     }
// }

// /// use in TTTR mode
// /// `buffer` should be at least 128 records long
// /// `records_to_fetch` should be a multiple of 128, less than the length of `buffer` and no longer than `TTREADMAX`
// /// In the result, returns Ok(records_written), where records_written is the number of records actually written to the buffer
// pub fn read_fifo(
//     &mut self,
//     buffer: &mut Vec<u32>,
//     records_to_fetch: i32,
// ) -> Result<i32, HydraHarpError> {
//     let mut records_written: i32 = 0;
//     error_enum_or_value! {
//         unsafe {
//             HH_ReadFiFo(
//                 self.id, buffer.as_mut_ptr(), records_to_fetch,
//                 &mut records_written as *mut i32
//                 )
//         },
//         records_written
//     }
// }

// /// Use in TTTR mode
// /// set the marker edges
// pub fn set_marker_edges(
//     &mut self,
//     me1: EdgeSelection,
//     me2: EdgeSelection,
//     me3: EdgeSelection,
//     me4: EdgeSelection,
// ) -> Result<(), HydraHarpError> {
//     error_enum_or_value! {
//         unsafe {
//             HH_SetMarkerEdges(self.id,
//                               num::ToPrimitive::to_i32(&me1).unwrap(),
//                               num::ToPrimitive::to_i32(&me2).unwrap(),
//                               num::ToPrimitive::to_i32(&me3).unwrap(),
//                               num::ToPrimitive::to_i32(&me4).unwrap())
//         },
//         ()
//     }
// }

// /// Use in TTTR mode
// /// enable or disable the marker edges
// pub fn enable_marker_edges(
//     &mut self,
//     en1: bool,
//     en2: bool,
//     en3: bool,
//     en4: bool,
// ) -> Result<(), HydraHarpError> {
//     error_enum_or_value! {
//         unsafe {
//             HH_SetMarkerEnable(self.id, en1 as i32, en2 as i32, en3 as i32, en4 as i32)},
//         ()
//     }
// }

// /// Use in TTTR mode
// /// Set the marker holdoff time in ns
// pub fn set_marker_holdoff_time(&mut self, holdoff_time: i32) -> Result<(), HydraHarpError> {
//     error_enum_or_value! {
//         unsafe {
//             HH_SetMarkerHoldoffTime(self.id, holdoff_time)
//         },
//         ()
//     }
// }

#[pyfunction]
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

#[pyfunction]
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

#[pyfunction]
/// Make a measurement for `acquisition_time` ms and return a tuple containing
/// `([singles], [coincidences])`
pub fn measure_and_get_counts(
    d: &mut Device,
    acquisition_time: i32,
    coincidence_window: u64,
    sync_channel: u8,
) -> PyResult<(Vec<usize>, Vec<usize>)> {
    const buffer_length: usize = 131072;
    let mut buffer: [u32; buffer_length] = [0u32; buffer_length];
    let mut measurement = Measurement::new(0);
    let mut sync_buffer: VecDeque<u64> = VecDeque::with_capacity(buffer_length);
    convert_hydra_harp_result(d.start_measurement(acquisition_time))?;
    let mut singles = [0usize; 8];
    let mut coincidences = [0usize; 8];
    loop {
        let num_read =
            convert_hydra_harp_result(d.read_fifo(&mut buffer, (buffer_length) as i32))? as usize;
        if num_read > 0 {
            let mut channel_times = measurement.convert_values_T2(&buffer[..num_read]);
            channel_times.sort_by_key(|(_, t)| *t);
            for &(channel, time) in channel_times.iter() {
                singles[channel as usize] += 1;
                if channel == sync_channel {
                    sync_buffer.push_back(time)
                } else {
                    let mut remove_index = None; // Index of sync channel counts that are out of the coincidence window, to be removed
                    for (i, sync_time) in sync_buffer.iter().enumerate() {
                        let delta_t = time - sync_time;
                        if delta_t > coincidence_window {
                            remove_index = Some(i);
                        } else {
                            // we have a coincidence - add to the coincidences
                            coincidences[channel as usize] += 1;
                        }
                    }
                    // TODO maybe only remove the last element of the vecdeque
                    // each time round the measurement loop
                    // it could be too slow this way...
                    if let Some(i) = remove_index {
                        sync_buffer.drain(0..i);
                    }
                }
            }
        } else {
            if convert_hydra_harp_result(d.get_CTC_status())? == crate::types::CTCStatus::Ended {
                break;
            }
        }
    }
    Ok((singles.to_vec(), coincidences.to_vec()))
}

#[pymodule]
fn hhlib_sys(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(open_device))?;
    m.add_wrapped(wrap_pyfunction!(close_device))?;
    m.add_wrapped(wrap_pyfunction!(initialise))?;
    m.add_wrapped(wrap_pyfunction!(get_base_resolution))?;
    m.add_wrapped(wrap_pyfunction!(get_number_of_input_channels))?;
    m.add_wrapped(wrap_pyfunction!(calibrate))?;
    m.add_wrapped(wrap_pyfunction!(set_sync_divider))?;
    m.add_wrapped(wrap_pyfunction!(set_sync_channel_offset))?;
    m.add_wrapped(wrap_pyfunction!(set_sync_CFD))?;
    m.add_wrapped(wrap_pyfunction!(set_input_CFD))?;
    m.add_wrapped(wrap_pyfunction!(set_input_channel_offset))?;
    m.add_wrapped(wrap_pyfunction!(set_input_channel_enabled))?;
    m.add_wrapped(wrap_pyfunction!(set_stop_overflow))?;
    m.add_wrapped(wrap_pyfunction!(set_binning))?;
    m.add_wrapped(wrap_pyfunction!(set_offset))?;
    m.add_wrapped(wrap_pyfunction!(clear_histogram_memory))?;
    m.add_wrapped(wrap_pyfunction!(start_measurement))?;
    m.add_wrapped(wrap_pyfunction!(stop_measurement))?;
    m.add_wrapped(wrap_pyfunction!(get_CTC_status))?;
    m.add_wrapped(wrap_pyfunction!(get_resolution))?;
    m.add_wrapped(wrap_pyfunction!(get_sync_rate))?;
    m.add_wrapped(wrap_pyfunction!(get_count_rate))?;
    m.add_wrapped(wrap_pyfunction!(get_elapsed_measurement_time))?;
    m.add_wrapped(wrap_pyfunction!(measure_and_get_counts))?;
    m.add_wrapped(wrap_pyfunction!(coincidence_channels_to_index))?;
    m.add_wrapped(wrap_pyfunction!(index_to_coincidence_channels))?;
    Ok(())
}
