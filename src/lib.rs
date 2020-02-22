extern crate chrono;
use chrono::Local;
use log::{LevelFilter, Metadata, Record};

pub struct ConsoleLogger;

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

impl ConsoleLogger {
    pub fn new(loglevel: String) {
        log::set_logger(&CONSOLE_LOGGER).unwrap();
        match loglevel.to_lowercase().as_str() {
            "trace" => log::set_max_level(LevelFilter::Trace),
            "debug" => log::set_max_level(LevelFilter::Debug),
            "info" => log::set_max_level(LevelFilter::Info),
            "warn" => log::set_max_level(LevelFilter::Warn),
            "error" => log::set_max_level(LevelFilter::Error),
            "off" => log::set_max_level(LevelFilter::Off),
            _ => {
                println!("Invalid loglevel. Expected trace, debug, info, warn, error or off. Received {:?}. Setting loglevel to info! anyway.", loglevel)
            }
        }
    }
}

impl log::Log for ConsoleLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{} {} module:{} file:{}:{} {:?}",
                Local::now().format("%d/%m/%Y:%H:%M:%S"),
                record.level().to_string(),
                record.module_path().unwrap_or("Cannot determine module"),
                record.file().unwrap_or("Cannot determine file"),
                record.line().unwrap_or(0),
                record.args()
            )
        }
    }

    fn flush(&self) {}
}

