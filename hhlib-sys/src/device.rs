//! The device struct and implementation

use pyo3::prelude::*;
use crate::bindings::*;
use crate::error_enum_or_value;
use crate::types::HydraHarpError::*;
use crate::types::{
    CTCStatus, EdgeSelection, HydraHarpError, MeasurementControl, MeasurementMode, ReferenceSource,
};

/// Contains the information of the device - the number it is (0 -> 7) and the serial of it.
#[pyclass]
#[derive(Debug, PartialEq)]
pub struct Device {
    pub id: i32,
    pub serial: [i8; 8],
    /// the length of the histograms returned by get_histogram in u32
    pub histogram_length: Option<usize>,
}

impl Device {
    /// Try to open a device given a device id and return a result with either the opened device or an error
    pub fn open_device(id: i32) -> Result<Device, HydraHarpError> {
        let mut serial = [0i8; 8];
        error_enum_or_value! {
            unsafe {HH_OpenDevice(id, serial.as_mut_ptr())},
            Device {id, serial, histogram_length: None}
        }
    }

    /// Try to close this device
    pub fn close_device(&mut self) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_CloseDevice(self.id)
            },
            ()
        }
    }

    /// Initialise the device. Should be done before other functions are run
    pub fn initialise(
        &mut self,
        mode: MeasurementMode,
        ref_source: ReferenceSource,
    ) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_Initialize(self.id,
                              num::ToPrimitive::to_i32(&mode).unwrap(),
                              num::ToPrimitive::to_i32(&ref_source).unwrap())
            },
            ()
        }
    }

    /// Get the base resolution of this device
    /// Returns a tuple (f64, i32) containing (resolution, bin steps) if successful
    pub fn get_base_resolution(&self) -> Result<(f64, i32), HydraHarpError> {
        let mut res = 0f64;
        let mut bin = 0i32;
        error_enum_or_value! {{
            unsafe {
                HH_GetBaseResolution(
                    self.id,
                    &mut res as *mut f64,
                    &mut bin as *mut i32
                )
            }},
            (res, bin)
        }
    }

    /// Get the number of input channels to this device
    pub fn get_number_of_input_channels(&self) -> Result<i32, HydraHarpError> {
        let mut inputs = 0i32;
        error_enum_or_value! {
            unsafe {
                HH_GetNumOfInputChannels(self.id, &mut inputs as *mut i32)
            },
            inputs
        }
    }

    /// Perform a device calibration
    pub fn calibrate(&mut self) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_Calibrate(self.id)
            },
            ()
        }
    }

    /// Set the sync divider
    pub fn set_sync_divider(&mut self, divisions: i32) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetSyncDiv(
                    self.id,
                    divisions
                )
            },
            ()
        }
    }

    /// Modify the sync CFD settings.
    /// level sets the CFD discriminator level in millivolts with bounds (DISCRMIN, DISCRMAX)
    /// zerox sets the CFD zero cross level in millivolts with bounds (ZCMIN, ZCMAX)
    pub fn set_sync_CFD(&mut self, level: i32, zerox: i32) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetSyncCFD(self.id, level, zerox)
            },
            ()
        }
    }

    /// Set the sync timing offset in ps
    /// minimum is CHANOFFSMIN, maximum is CHANOFFSMAX
    pub fn set_sync_channel_offset(&mut self, offset: i32) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetSyncChannelOffset(self.id, offset)
            },
            ()
        }
    }

    /// Modify the input CFD. Bounds are the same as the `set_sync_CFD`
    pub fn set_input_CFD(
        &mut self,
        channel: i32,
        level: i32,
        zerox: i32,
    ) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetInputCFD(self.id, channel, level, zerox)
            },
            ()
        }
    }

    /// Set the timing offset on the given channel in picoseconds
    pub fn set_input_channel_offset(
        &mut self,
        channel: i32,
        offset: i32,
    ) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetInputChannelOffset(self.id, channel, offset)
            },
            ()
        }
    }

    /// Set the enabled state of the given channel
    pub fn set_input_channel_enabled(
        &mut self,
        channel: i32,
        enabled: bool,
    ) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetInputChannelEnable(self.id, channel, enabled as i32)
            },
            ()
        }
    }

    /// This setting determines if a measurement run will stop if any channel reaches the maximum set by `stopcount`.
    /// If `stop_ofl` is `false` the measurement will continue, but counts above `STOPCNTMAX` in any bin will be clipped.
    pub fn set_stop_overflow(
        &mut self,
        stop_ofl: bool,
        stopcount: u32,
    ) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetStopOverflow(self.id, stop_ofl as i32, stopcount)
            },
            ()
        }
    }

    /// Set the binning. The binning value corresponds to powers of 2*the base resolution.
    /// eg: `binning = 0 => 1*base_resolution`
    ///     `binning = 1 => 2*base_resolution`
    ///     `binning = 2 => 4*base_resolution`
    pub fn set_binning(&mut self, binning: i32) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetBinning(self.id, binning)
            },
            ()
        }
    }

    /// Set the histogram time offset in nanoseconds
    pub fn set_offset(&mut self, offset: i32) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetOffset(self.id, offset)
            },
            ()
        }
    }

    /// Set the histogram length. Returns the actual length calculated as `1024*(2^lencode)`
    pub fn set_histogram_length(&mut self, length: i32) -> Result<i32, HydraHarpError> {
        let mut actual_length: i32 = 0;
        let return_val = error_enum_or_value! {
            unsafe {
                HH_SetHistoLen(self.id, length, &mut actual_length as *mut i32)
            },
            actual_length
        };
        if let Ok(len) = return_val {
            self.histogram_length = Some(len as usize);
        }
        return_val
    }

    /// Clear the histogram memory
    pub fn clear_histogram_memory(&mut self) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_ClearHistMem(self.id)
            },
            ()
        }
    }

    /// Set the measurement control code and edges
    pub fn set_measurement_control(
        &mut self,
        control: MeasurementControl,
        start_edge: EdgeSelection,
        stop_edge: EdgeSelection,
    ) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetMeasControl(self.id, num::ToPrimitive::to_i32(&control).unwrap(),
                                  num::ToPrimitive::to_i32(&start_edge).unwrap(),
                                  num::ToPrimitive::to_i32(&stop_edge).unwrap())
            },
            ()
        }
    }

    /// Start a measurement with acquisition time in milliseconds
    pub fn start_measurement(&mut self, acquisition_time: i32) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_StartMeas(self.id, acquisition_time)
            },
            ()
        }
    }

    /// Stop a measurement. Can be used before the acquisition time expires
    pub fn stop_measurement(&mut self) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_StopMeas(self.id)
            },
            ()
        }
    }

    /// Get the status of the device, whether the acquisiton time is still going, or if it has ended.
    pub fn get_CTC_status(&self) -> Result<CTCStatus, HydraHarpError> {
        let mut status: i32 = 0;
        error_enum_or_value! {
            unsafe {
                HH_CTCStatus(self.id, &mut status as *mut i32)
            },
            num::FromPrimitive::from_i32(status).unwrap()
        }
    }

    /// Get the histogram from the device. Returns the error `HistogramLengthNotKnown` if
    /// `self.histogram_length = None`. If clear is true then the acquisiton buffer is cleared upon reading,
    /// otherwise it isn't
    pub fn get_histogram(&mut self, channel: i32, clear: bool) -> Result<Vec<u32>, HydraHarpError> {
        if let Some(histogram_length) = self.histogram_length {
            // let mut histogram_data: Vec<u32> = Vec::with_capacity(histogram_length);
            let mut histogram_data: Vec<u32> = vec![0; histogram_length];
            error_enum_or_value! {
                unsafe {
                    HH_GetHistogram(self.id, histogram_data.as_mut_ptr(), channel, clear as i32)
                },
                histogram_data
            }
        } else {
            Err(HydraHarpError::HistogramLengthNotKnown)
        }
    }

    /// get the resolution at the current histogram bin width in picoseconds
    pub fn get_resolution(&self) -> Result<f64, HydraHarpError> {
        let mut resolution: f64 = 0.0;
        error_enum_or_value! {
            unsafe {
                HH_GetResolution(self.id, &mut resolution as *mut f64)
            },
            resolution
        }
    }

    /// get the current sync rate
    pub fn get_sync_rate(&self) -> Result<i32, HydraHarpError> {
        let mut sync_rate: i32 = 0;
        error_enum_or_value! {
            unsafe {
                HH_GetSyncRate(self.id, &mut sync_rate as *mut i32)
            },
            sync_rate
        }
    }

    /// get the current count rate
    /// allow at least 100ms after initialise or set_sync_divider to get a stable meter reading
    /// wait at least 100ms to get a new reading. This is the gate time of the counters
    pub fn get_count_rate(&self, channel: i32) -> Result<i32, HydraHarpError> {
        let mut count_rate: i32 = 0;
        error_enum_or_value! {
            unsafe {
                HH_GetCountRate(self.id, channel, &mut count_rate as *mut i32)
            },
            count_rate
        }
    }

    /// get the flags. Use the `FLAG_*` variables to extract the different flags
    pub fn get_flags(&self) -> Result<i32, HydraHarpError> {
        let mut flags: i32 = 0;
        error_enum_or_value! {
            unsafe {
                HH_GetFlags(self.id, &mut flags as *mut i32)
            },
            flags
        }
    }

    /// get the elapsed measurement time in ms
    pub fn get_elapsed_measurement_time(&self) -> Result<f64, HydraHarpError> {
        let mut time: f64 = 0.0;
        error_enum_or_value! {
            unsafe {
                HH_GetElapsedMeasTime(self.id, &mut time as *mut f64)
            },
            time
        }
    }

    /// get the warnings encoded bitwise
    pub fn get_warnings(&self) -> Result<i32, HydraHarpError> {
        let mut warnings: i32 = 0;
        error_enum_or_value! {
            unsafe {
                HH_GetWarnings(self.id, &mut warnings as *mut i32)
            },
            warnings
        }
    }

    /// use in TTTR mode
    /// `buffer` should be at least 128 records long
    /// `records_to_fetch` should be a multiple of 128, less than the length of `buffer` and no longer than `TTREADMAX`
    /// In the result, returns Ok(records_written), where records_written is the number of records actually written to the buffer
    pub fn read_fifo(
        &mut self,
        buffer: &mut Vec<u32>,
        records_to_fetch: i32,
    ) -> Result<i32, HydraHarpError> {
        let mut records_written: i32 = 0;
        error_enum_or_value! {
            unsafe {
                HH_ReadFiFo(
                    self.id, buffer.as_mut_ptr(), records_to_fetch,
                    &mut records_written as *mut i32
                    )
            },
            records_written
        }
    }

    /// Use in TTTR mode
    /// set the marker edges
    pub fn set_marker_edges(
        &mut self,
        me1: EdgeSelection,
        me2: EdgeSelection,
        me3: EdgeSelection,
        me4: EdgeSelection,
    ) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetMarkerEdges(self.id,
                                  num::ToPrimitive::to_i32(&me1).unwrap(),
                                  num::ToPrimitive::to_i32(&me2).unwrap(),
                                  num::ToPrimitive::to_i32(&me3).unwrap(),
                                  num::ToPrimitive::to_i32(&me4).unwrap())
            },
            ()
        }
    }

    /// Use in TTTR mode
    /// enable or disable the marker edges
    pub fn enable_marker_edges(
        &mut self,
        en1: bool,
        en2: bool,
        en3: bool,
        en4: bool,
    ) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetMarkerEnable(self.id, en1 as i32, en2 as i32, en3 as i32, en4 as i32)},
            ()
        }
    }

    /// Use in TTTR mode
    /// Set the marker holdoff time in ns
    pub fn set_marker_holdoff_time(&mut self, holdoff_time: i32) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetMarkerHoldoffTime(self.id, holdoff_time)
            },
            ()
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        self.close_device();
    }
}
