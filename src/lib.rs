use pyo3::prelude::*;

pub mod connector;
pub mod data;
pub mod wifi;

#[pymodule]
fn mfep_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(connector::send_command, m)?)?;
    m.add_function(wrap_pyfunction!(connector::get_network_config, m)?)?;
    m.add_function(wrap_pyfunction!(connector::set_network_config, m)?)?;
    Ok(())
}
