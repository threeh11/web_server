use std::path::Path;
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger, WriteMode};
use crate::config::jexus_config::Main;

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

pub struct AppLogger<'a> {
    path: &'a Path
}

impl<'a> AppLogger<'a> {
    fn new(path: &str) -> Self {
        AppLogger {
            path: Path::new(path),
        }
    }

    fn build_logger(&self) -> Result<(), Box<dyn std::error::Error>> {
        Logger::try_with_str("error")?
            .log_to_file(
                FileSpec::default()
                    .directory(self.path.to_str())
                    .suffix("log")
            )
            .write_mode(WriteMode::Direct)
            .format(detailed_format)
            .duplicate_to_stderr(Duplicate::Error)
            .start()?;
        Ok(())
    }
}

pub struct JexusLogger {
    access_logger: AccessLogger,
    error_logger: ErrorLogger,
}

impl<'a> JexusLogger {
    pub fn new(
        main_config: &'a Main,
    ) -> Self {
        // let path_error_log = Path::new(&main_config.error_log).to_str().unwrap();
        // let path_access_log = Path::new(&main_config.access_log).to_str().unwrap();
        // let level_logger = LevelsLogger::from_str(&main_config.error_log_level);
        // Self {
        //     access_logger: AccessLogger::new(path_access_log),
        //     error_logger: ErrorLogger::new(level_logger, path_error_log),
        // }
    }

    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        //App logger (flexi_logger) - будет логировать програмные ошибки и записыват в по дефолному пути
        //todo next прокидывать это значение из конфигуратора сборки (см. https://nginx.org/ru/docs/configure.html)
        match AppLogger::new("./logs/error.log").build_logger() {
            Ok(_) => {},
            Err(e) => {
                panic!("Error initializing app logger: {}", e);
            }
        }

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