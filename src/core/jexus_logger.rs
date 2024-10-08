use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use chrono::Local;

use crate::config::jexus_config::JxsMain;
use crate::config::default::{ACCESS_LOG_PATH, ERROR_LOG_PATH};

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

#[derive(Debug)]
pub struct ErrorLogger {
    path: String,
}

impl ErrorLogger {
    pub fn init(mut path: String) -> Self {
        let path_ref: &Path = Path::new(&path);

        match fs::create_dir_all(path_ref) {
            Ok(_) => println!("Папка успешно создана!"),
            Err(e) => println!("Ошибка при создании папки: {}", e)
        }
        path.push_str("error.log");
        let path_ref: &Path = Path::new(&path);

        if let Err(e) = fs::File::create(path_ref) {
            println!("Ошибка при создании файла: {}", e);
        } else {
            println!("Файл успешно создан!");
        }

        Self {
            path,
        }
    }

    pub fn log_write(&self, message: &str) -> io::Result<()> {
        let path_ref: &Path = Path::new(&self.path);
        let mut file: File = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path_ref).unwrap();
        let timestamp: String = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry: String = format!("[{}] [{}] {}\n", timestamp, LevelsLogger::Error.to_str(), message);

        let _ = file.write_all(log_entry.as_bytes()).unwrap();
        Ok(())
    }
}


#[derive(Debug)]
pub struct AccessLogger {
    path: String,
}

impl AccessLogger {
    pub fn init(mut path: String) -> Self {
        let path_ref: &Path = Path::new(&path);

        match fs::create_dir_all(path_ref) {
            Ok(_) => println!("Папка успешно создана!"),
            Err(e) => println!("Ошибка при создании папки: {}", e)
        }
        path.push_str("access.log");
        let path_ref: &Path = Path::new(&path);

        if let Err(e) = fs::File::create(path_ref) {
            println!("Ошибка при создании файла: {}", e);
        } else {
            println!("Файл успешно создан!");
        }
        
        Self {
            path,
        }
    }

    pub fn log_write(&self, message: &str) -> io::Result<()> {
        let path_ref: &Path = Path::new(&self.path);
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path_ref).unwrap();
        let timestamp: String = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry: String = format!("[{}] [{}] {}\n", timestamp, LevelsLogger::Info.to_str(), message);

        let _ = file.write_all(log_entry.as_bytes()).unwrap();
        Ok(())
    }
}

#[derive(Debug)]
pub struct JxsLogger {
    error_log: ErrorLogger,
    access_log: AccessLogger,
}

impl JxsLogger {
    pub fn new(main: &JxsMain) -> Self {
        let error_log_path: String = Self::get_log_path(main.error_log.clone(), ERROR_LOG_PATH);
        let access_log_path: String = Self::get_log_path(main.access_log.clone(), ACCESS_LOG_PATH);

        Self {
            error_log: ErrorLogger::init(error_log_path),
            access_log: AccessLogger::init(access_log_path),
        }
    }

    fn get_log_path(config: String, path: &str) -> String {
        if config.is_empty() { 
            path.to_string() 
        } else {
            config
        }
    }

    pub fn log_write(&self, level: LevelsLogger, message: &str) {
        match level {
            LevelsLogger::Info => self.access_log.log_write(message).unwrap(),
            LevelsLogger::Error => self.error_log.log_write(message).unwrap(),
            LevelsLogger::Warn | LevelsLogger::Debug | LevelsLogger::Trace => todo!(),
        }
    }
}