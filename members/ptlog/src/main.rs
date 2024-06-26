use std::any::Any;

use libpt::log;
use log::{debug, error, info, trace, warn};

fn main() -> anyhow::Result<()> {
    let logger = log::Logger::builder()
        .show_time(false)
        .max_level(log::Level::TRACE)
        .build()?;
    warn!("logger is now enabled");
    info!("logger was initialized");
    debug!("logger: {logger:#?}");
    trace!("Type id of libpt::log::Logger is '{:#?}'", logger.type_id());
    let u = 19;
    error!(some_value=u, "No error, this is just a test");
    Ok(())
}
