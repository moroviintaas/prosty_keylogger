use std::path::Path;
use std::time::SystemTime;
use log::LevelFilter;

pub fn setup_logger(level: LevelFilter, path: Option<impl AsRef<Path>>) -> Result<(), fern::InitError> {
    let mut d = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(level);
    if let Some(p) = path{
        d = d.chain(fern::log_file(p)?);
    }
    d.chain(std::io::stdout())
        .apply()?;
    Ok(())
}