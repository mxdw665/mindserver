use dirs::home_dir;
use flexi_logger::{DeferredNow, FileSpec, LogSpecification, Logger, Record};
use std::io::{Error, Write};

fn log_format(
    write: &mut dyn Write,
    now: &mut DeferredNow,
    record: &Record<'_>,
) -> Result<(), Error> {
    let time = now.format("%Y-%m-%d %H:%M");
    write!(write, "[{time} {}] {}", record.level(), record.args())
}

pub fn init() {
    let spec = LogSpecification::trace();
    let file = FileSpec::try_from(home_dir().unwrap().join(".msi.log")).unwrap();
    Logger::with(spec)
        .log_to_file(file)
        .format(log_format)
        .start()
        .unwrap();
}
