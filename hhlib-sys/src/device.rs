use crate::bindings::*;
use crate::error_enum_or_value;
use crate::types::HydraHarpError::*;
use crate::types::{HydraHarpError, MeasurementMode, ReferenceSource};

/// Contains the information of the device - the number it is (0 -> 7) and the serial of it.
#[derive(Debug, PartialEq)]
pub struct Device {
    pub id: i32,
    pub serial: [i8; 8],
}

impl Device {
    /// Try to open a device given a device id and return a result with either the opened device or an error
    pub fn open_device(id: i32) -> Result<Device, HydraHarpError> {
        let mut serial = [0i8; 8];
        error_enum_or_value! {
            unsafe {HH_OpenDevice(id, serial.as_mut_ptr())},
            Device {id, serial}
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
    pub fn set_input_CFD(&mut self, channel: i32, level: i32, zerox: i32) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetInputCFD(self.id, channel, level, zerox)
            },
            ()
        }
    }

    /// Set the timing offset on the given channel in picoseconds
    pub fn set_input_channel_offset(&mut self, channel: i32, offset: i32) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetInputChannelOffset(self.id, channel, offset)
            },
            ()
        }
    }

    /// Set the enabled state of the given channel
    pub fn set_input_channel_enabled(&mut self, channel: i32, enabled: bool) -> Result<(), HydraHarpError> {
        error_enum_or_value! {
            unsafe {
                HH_SetInputChannelEnable(self.id, channel, enabled as i32)
            },
            ()
        }
    }

    /// This setting determines if a measurement run will stop if any channel reaches the maximum set by `stopcount`.
    /// If `stop_ofl` is `false` the measurement will continue, but counts above `STOPCNTMAX` in any bin will be clipped.
    pub fn set_stop_overflow(&mut self, stop_ofl: bool, stopcount: u32) -> Result<(), HydraHarpError> {
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
    pub fun set_binning(&mut self, binning: i32) -> Result<(), HydraHarpError> {
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
}

impl Drop for Device {
    fn drop(&mut self) {
        self.close_device();
    }
}
