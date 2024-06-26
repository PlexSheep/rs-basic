use libpt::log::{self, debug};

use diesel_demo as lib;

fn main() -> anyhow::Result<()> {
    let _logger = log::Logger::builder()
        .max_level(log::Level::TRACE)
        .uptime(true)
        .build();
    debug!("logger initialized");

    let conn = lib::establish_connection()?;
    debug!("db connection established");

    Ok(())
}
