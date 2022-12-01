use anyhow::{Context, Result};
use gray_hole_lib::server::Server;

fn main() -> Result<()> {
    let server_ip = std::env::var("BLACK_HOLE_SERVER_IP")?
        .parse()
        .context("Unable to parse ip from env.")?;

    let server_port = std::env::var("BLACK_HOLE_SERVER_PORT")?
        .parse()
        .context("Unable to parse port from env.")?;

    let mut server = Server::new(server_ip, server_port)?;

    server.run()?;
    Ok(())
}
