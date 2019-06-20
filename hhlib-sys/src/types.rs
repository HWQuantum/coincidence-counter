//! Definitions of the types used. Enums n that
use crate::bindings::*;

#[derive(FromPrimitive, ToPrimitive, Debug, PartialEq)]
pub enum HydraHarpError {
    DeviceFailedToOpen = HH_ERROR_DEVICE_OPEN_FAIL as isize,
    DeviceBusy = HH_ERROR_DEVICE_BUSY as isize,
    DeviceHEventFail = HH_ERROR_DEVICE_HEVENT_FAIL as isize,
    DeviceCallBSetFail = HH_ERROR_DEVICE_CALLBSET_FAIL as isize,
    DeviceBarmapFail = HH_ERROR_DEVICE_BARMAP_FAIL as isize,
    DeviceFailedToClose = HH_ERROR_DEVICE_CLOSE_FAIL as isize,
    DeviceFailedToReset = HH_ERROR_DEVICE_RESET_FAIL as isize,
    FailedToGetDeviceVersion = HH_ERROR_DEVICE_GETVERSION_FAIL as isize,
    DeviceVersionMismatch = HH_ERROR_DEVICE_VERSION_MISMATCH as isize,
    DeviceNotOpen = HH_ERROR_DEVICE_NOT_OPEN as isize,
    InstanceRunning = HH_ERROR_INSTANCE_RUNNING as isize,
    InvalidArgument = HH_ERROR_INVALID_ARGUMENT as isize,
    InvalidMode = HH_ERROR_INVALID_MODE as isize,
    InvalidOption = HH_ERROR_INVALID_OPTION as isize,
    InvalidMemory = HH_ERROR_INVALID_MEMORY as isize,
    InvalidRData = HH_ERROR_INVALID_RDATA as isize,
    NotInitialized = HH_ERROR_NOT_INITIALIZED as isize,
    NotCalibrated = HH_ERROR_NOT_CALIBRATED as isize,
    DMAFail = HH_ERROR_DMA_FAIL as isize,
    XTDeviceFail = HH_ERROR_XTDEVICE_FAIL as isize,
    FPGAConfFail = HH_ERROR_FPGACONF_FAIL as isize,
    IFConfFail = HH_ERROR_IFCONF_FAIL as isize,
    FIFOResetFail = HH_ERROR_FIFORESET_FAIL as isize,
    FailedToGetDriverVersion = HH_ERROR_USB_GETDRIVERVER_FAIL as isize,
    DriverVersionMismatch = HH_ERROR_USB_DRIVERVER_MISMATCH as isize,
    USBGetIFInfoFail = HH_ERROR_USB_GETIFINFO_FAIL as isize,
    USBHiSpeedFail = HH_ERROR_USB_HISPEED_FAIL as isize,
    USBVCMDFail = HH_ERROR_USB_VCMD_FAIL as isize,
    USBBulkRDFail = HH_ERROR_USB_BULKRD_FAIL as isize,
    USBResetFail = HH_ERROR_USB_RESET_FAIL as isize,
    LaneUpTimeout = HH_ERROR_LANEUP_TIMEOUT as isize,
    DoneAllTimeout = HH_ERROR_DONEALL_TIMEOUT as isize,
    MODACKTimeout = HH_ERROR_MODACK_TIMEOUT as isize,
    MACTIVETimeout = HH_ERROR_MACTIVE_TIMEOUT as isize,
    MEMClearFail = HH_ERROR_MEMCLEAR_FAIL as isize,
    MEMTestFail = HH_ERROR_MEMTEST_FAIL as isize,
    CALIBFail = HH_ERROR_CALIB_FAIL as isize,
    REFSELFail = HH_ERROR_REFSEL_FAIL as isize,
    StatusFail = HH_ERROR_STATUS_FAIL as isize,
    MODNUMFail = HH_ERROR_MODNUM_FAIL as isize,
    DIGMUXFail = HH_ERROR_DIGMUX_FAIL as isize,
    MODMUXFail = HH_ERROR_MODMUX_FAIL as isize,
    MODFWPCBMismatch = HH_ERROR_MODFWPCB_MISMATCH as isize,
    MODFWVERMismatch = HH_ERROR_MODFWVER_MISMATCH as isize,
    MODPropertyMismatch = HH_ERROR_MODPROPERTY_MISMATCH as isize,
    InvalidMagic = HH_ERROR_INVALID_MAGIC as isize,
    InvalidLength = HH_ERROR_INVALID_LENGTH as isize,
    RateFail = HH_ERROR_RATE_FAIL as isize,
    MODFWVERTooLow = HH_ERROR_MODFWVER_TOO_LOW as isize,
    MODFWVERTooHigh = HH_ERROR_MODFWVER_TOO_HIGH as isize,
    EEPROMF01 = HH_ERROR_EEPROM_F01 as isize,
    EEPROMF02 = HH_ERROR_EEPROM_F02 as isize,
    EEPROMF03 = HH_ERROR_EEPROM_F03 as isize,
    EEPROMF04 = HH_ERROR_EEPROM_F04 as isize,
    EEPROMF05 = HH_ERROR_EEPROM_F05 as isize,
    EEPROMF06 = HH_ERROR_EEPROM_F06 as isize,
    EEPROMF07 = HH_ERROR_EEPROM_F07 as isize,
    EEPROMF08 = HH_ERROR_EEPROM_F08 as isize,
    EEPROMF09 = HH_ERROR_EEPROM_F09 as isize,
    EEPROMF10 = HH_ERROR_EEPROM_F10 as isize,
    EEPROMF11 = HH_ERROR_EEPROM_F11 as isize,
    UnknownError = HH_ERROR_EEPROM_F11 as isize - 1,
    HistogramLengthNotKnown = HH_ERROR_EEPROM_F11 as isize - 2,
}

#[derive(FromPrimitive, Debug)]
pub enum Warning {
    SyncRateZero = WARNING_SYNC_RATE_ZERO as isize,
    SyncRateTooLow = WARNING_SYNC_RATE_TOO_LOW as isize,
    SyncRateTooHigh = WARNING_SYNC_RATE_TOO_HIGH as isize,
    InputRateZero = WARNING_INPT_RATE_ZERO as isize,
    InputRateTooHigh = WARNING_INPT_RATE_TOO_HIGH as isize,
    InputRateRation = WARNING_INPT_RATE_RATIO as isize,
    DividerGreaterThanOne = WARNING_DIVIDER_GREATER_ONE as isize,
    TimeSpanTooSmall = WARNING_TIME_SPAN_TOO_SMALL as isize,
    OffsetUnnecessary = WARNING_OFFSET_UNNECESSARY as isize,
}

#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy)]
pub enum MeasurementMode {
    Histogramming = MODE_HIST as isize,
    T2 = MODE_T2 as isize,
    T3 = MODE_T3 as isize,
    Continuous = MODE_CONT as isize,
}

#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy)]
pub enum MeasurementControl {
    SingleShotCTC = MEASCTRL_SINGLESHOT_CTC as isize,
    C1Gated = MEASCTRL_C1_GATED as isize,
    C1StartCTCStop = MEASCTRL_C1_START_CTC_STOP as isize,
    C1StartC2Stop = MEASCTRL_C1_START_C2_STOP as isize,
    ContC1Gated = MEASCTRL_CONT_C1_GATED as isize,
    ContC1StartCTCStop = MEASCTRL_CONT_C1_START_CTC_STOP as isize,
    ContCTCRestart = MEASCTRL_CONT_CTC_RESTART as isize,
}

#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy, PartialEq)]
pub enum ReferenceSource {
    Internal = 0,
    External = 1,
}

#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy, PartialEq)]
pub enum EdgeSelection {
    Falling = 0,
    Rising = 1,
}

#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy, PartialEq)]
pub enum CTCStatus {
    Running = 0,
    Ended = 1,
}
