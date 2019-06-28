use crate::device::Device;
use crate::types::convert_hydra_harp_result;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
pub fn open_device(id: i32) -> PyResult<Device> {
    convert_hydra_harp_result(Device::open_device(id))
}

#[pymodule]
fn hhlib_sys(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(open_device))?;
    Ok(())
}
