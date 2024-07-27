use flexi_logger::{detailed_format, FileSpec, Logger, WriteMode};

pub enum LevelsLogger {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub struct ErrorLogger<'a> {
    level: LevelsLogger,
    path: &'a str,
}

impl ErrorLogger {
    pub fn new(level: LevelsLogger, path: &str) -> ErrorLogger {
        ErrorLogger {
            level,
            path,
        }
    }

    pub fn build_logger(&self) -> Result<(), Box<dyn std::error::Error>> {
        Logger::try_with_str("info")?
            .log_to_file(FileSpec::default().directory(self.path))
            .write_mode(WriteMode::Direct)
            .format(detailed_format)
            .start()?;
        Ok(())
    }

}