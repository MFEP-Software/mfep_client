use crate::data;
use crate::wifi;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[pyfunction]
pub fn send_command(config_path: &str, cmd: &str) -> PyResult<String> {
    let cfg = data::load(config_path)
        .map_err(|e| PyRuntimeError::new_err(format!("Config-Fehler: {}", e)))?;

    wifi::send_command(&cfg, cmd)
        .map_err(|e| PyRuntimeError::new_err(format!("Netzwerk-Fehler: {}", e)))?;

    Ok(format!("Befehl '{}' gesendet", cmd))
}

#[pyfunction]
pub fn get_network_config(config_path: &str) -> PyResult<(String, u16)> {
    let cfg = data::load(config_path)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok((cfg.addr1.ip, cfg.addr1.port))
}

#[pyfunction]
pub fn set_network_config(config_path: &str, new_ip: String, new_port: u16) -> PyResult<String> {
    let mut cfg = data::load(config_path)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

    cfg.addr1.ip = new_ip;
    cfg.addr1.port = new_port;

    data::save(config_path, &cfg)
        .map_err(|e| PyRuntimeError::new_err(format!("Speicher-Fehler: {}", e)))?;

    Ok("Einstellungen gespeichert".to_string())
}
