use crate::data::Config;
use std::io::Write;
use std::net::TcpStream;
use std::time::Duration;

pub fn send_command(cfg: &Config, cmd: &str) -> std::io::Result<()> {
    let addr = format!("{}:{}", cfg.addr1.ip, cfg.addr1.port);
    let timeout = Duration::from_secs(cfg.port.timeout);
    let mut stream = TcpStream::connect(&addr)?;
    stream.set_write_timeout(Some(timeout))?;

    let msg = format!("{}\n", cmd);
    stream.write_all(msg.as_bytes())?;

    println!("Command sent: {} to {}", cmd, addr);
    Ok(())
}
