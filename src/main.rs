use mfep_client::{data, wifi};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = data::load("conf/config.yaml")?;
    wifi::send_command(&cfg, "TEST")?;
    Ok(())
}
