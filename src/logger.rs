use std::path::Path;
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger, WriteMode};
use crate::config::config_parser::Main;

pub enum LevelsLogger {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LevelsLogger {
    fn to_str(&self) -> &'static str {
        match *self {
            LevelsLogger::Error => "error",
            LevelsLogger::Warn => "warn",
            LevelsLogger::Info => "info",
            LevelsLogger::Debug => "debug",
            LevelsLogger::Trace => "trace",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "error" => LevelsLogger::Error,
            "warn" => LevelsLogger::Warn,
            "info" => LevelsLogger::Info,
            "debug" => LevelsLogger::Debug,
            "trace" => LevelsLogger::Trace,
            _ => LevelsLogger::Error,
        }
    }
}

struct ErrorLogger<'a> {
    level: LevelsLogger,
    path: &'a str,
}

impl ErrorLogger<'_> {
    fn new(level: LevelsLogger, path: &str) -> ErrorLogger {
        ErrorLogger {
            level,
            path,
        }
    }

    fn build_logger(&self) -> Result<(), Box<dyn std::error::Error>> {
        Logger::try_with_str(self.level.to_str())?
            .log_to_file(
                FileSpec::default()
                    .directory(self.path)
                    .basename("error")
                    .use_timestamp(false)
                    .suffix("log")
            )
            .write_mode(WriteMode::Direct)
            .format(detailed_format)
            .duplicate_to_stderr(Duplicate::Error)
            .start()?;
        Ok(())
    }

}

struct AccessLogger<'a> {
    path: &'a str
}

impl AccessLogger<'_> {
    fn new(path: &str) -> AccessLogger {
        AccessLogger {
            path,
        }
    }

    fn build_logger(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Logger::try_with_str(LevelsLogger::Info.to_str())?
            .log_to_file(
                FileSpec::default()
                    .directory(self.path)
                    .basename("access")
                    .use_timestamp(false)
                    .suffix("log")
            )
            .write_mode(WriteMode::Direct)
            .format(detailed_format)
            .duplicate_to_stderr(Duplicate::Error)
            .start()?;
        Ok(())
    }
}

pub struct JexusLogger<'a> {
    access_logger: AccessLogger<'a>,
    error_logger: ErrorLogger<'a>,
}

impl<'a> JexusLogger<'a> {
    pub fn new(
        main_config: &'a Main,
    ) -> Self {
        let path_error_log = Path::new(&main_config.error_log).to_str().unwrap();
        let path_access_log = Path::new(&main_config.access_log).to_str().unwrap();
        let level_logger = LevelsLogger::from_str(&main_config.error_log_level);
        Self {
            access_logger: AccessLogger::new(path_access_log),
            error_logger: ErrorLogger::new(level_logger, path_error_log),
        }
    }

    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.access_logger.build_logger() {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error initializing access logger: {}", e);
                return Err(e);
            }
        }

        match self.error_logger.build_logger() {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error initializing error logger: {}", e);
                return Err(e);
            }
        }

        Ok(())
    }

}