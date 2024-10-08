use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml;
use std::path::Path;
use hyper::Uri;
use crate::os::file_manager::FileManager;

#[derive(Debug, Deserialize, Serialize)]
pub struct JexusConfigYaml {
    #[serde(default)]
    pub main: Main,
    #[serde(default)]
    pub http: Http,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Main {
    #[serde(default, deserialize_with = "deserialize_worker_processes")]
    pub worker_processes: WorkerProcesses,
    #[serde(default)]
    pub error_log: String,
    #[serde(default)]
    pub access_log: String,
    #[serde(default)]
    pub error_log_level: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Http {
    #[serde(default)]
    pub servers: Vec<Server>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    #[serde(default)]
    pub listen: usize,
    #[serde(default)]
    pub root: String,
    #[serde(default)]
    pub locations: Vec<Location>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    #[serde(default)]
    pub uri: String,
    #[serde(default)]
    pub index: String,
    #[serde(default, deserialize_with = "deserialize_auto_index")]
    pub auto_index: AutoIndex,
    #[serde(default, deserialize_with = "deserialize_random_index")]
    pub random_index: RandomIndex,
    #[serde(default)]
    pub proxy_pass: String,
    #[serde(default)]
    pub fastcgi_pass: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum WorkerProcesses {
    Auto,
    Number(i32),
}

impl<'de> Deserialize<'de> for WorkerProcesses {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_yaml::Value = Deserialize::deserialize(deserializer)?;
        match value {
            serde_yaml::Value::String(s) if s == "auto" => Ok(WorkerProcesses::Auto),
            serde_yaml::Value::Number(n) => n
                .as_i64()
                .and_then(|n| Some(WorkerProcesses::Number(n as i32)))
                .ok_or_else(|| serde::de::Error::custom("Invalid number")),
            _ => Err(serde::de::Error::custom("Invalid type")),
        }
    }
}


fn deserialize_worker_processes<'de, D>(deserializer: D) -> Result<WorkerProcesses, D::Error>
where
    D: Deserializer<'de>,
{
    WorkerProcesses::deserialize(deserializer)
}

#[derive(Debug, PartialEq, Serialize)]
pub enum AutoIndex {
    On,
    Off
}

impl Default for AutoIndex {
    fn default() -> Self {
        AutoIndex::Off
    }
}

impl<'de> Deserialize<'de> for AutoIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let value: serde_yaml::Value = Deserialize::deserialize(deserializer)?;
        match value {
            serde_yaml::Value::String(s) if s == "off" => Ok(AutoIndex::Off),
            serde_yaml::Value::String(s) if s == "on" => Ok(AutoIndex::On),
            _ => Err(serde::de::Error::custom("Invalid type")),
        }
    }
}

fn deserialize_auto_index<'de, D>(deserializer: D) -> Result<AutoIndex, D::Error>
where
    D: Deserializer<'de>,
{
    AutoIndex::deserialize(deserializer)
}

#[derive(Debug, PartialEq, Serialize)]
pub enum RandomIndex {
    On,
    Off
}

impl Default for RandomIndex {
    fn default() -> Self {
        RandomIndex::Off
    }
}

impl<'de> Deserialize<'de> for RandomIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let value: serde_yaml::Value = Deserialize::deserialize(deserializer)?;
        match value {
            serde_yaml::Value::String(s) if s == "off" => Ok(RandomIndex::Off),
            serde_yaml::Value::String(s) if s == "on" => Ok(RandomIndex::On),
            _ => Err(serde::de::Error::custom("Invalid type")),
        }
    }
}

fn deserialize_random_index<'de, D>(deserializer: D) -> Result<RandomIndex, D::Error>
where
    D: Deserializer<'de>,
{
    RandomIndex::deserialize(deserializer)
}

impl Default for WorkerProcesses {
    fn default() -> Self {
        WorkerProcesses::Number(1)
    }
}

impl Default for JexusConfigYaml {
    fn default() -> Self {
        JexusConfigYaml {
            main: Default::default(),
            http: Default::default(),
        }
    }
}

impl Default for Main {
    fn default() -> Self {
        Main {
            worker_processes: Default::default(),
            error_log: Default::default(),
            error_log_level: Default::default(),
            access_log: Default::default(),
        }
    }
}

impl Default for Http {
    fn default() -> Self {
        Http {
            servers: Default::default(),
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Server {
            listen: 80,
            root: String::new(),
            locations: Default::default(),
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Location {
            uri: String::new(),
            index: String::from("index.html"),
            auto_index: Default::default(),
            random_index: Default::default(),
            proxy_pass: String::new(),
            fastcgi_pass: String::new(),
        }
    }
}

impl JexusConfigYaml {
    pub fn parse(path_yaml_conf: &str) -> Result<JexusConfigYaml, Box<dyn std::error::Error>>  {
        let path_yaml_conf = Path::new(path_yaml_conf);

        let file_manager = FileManager::new_by_file(path_yaml_conf);
        if !file_manager.exists_file() {
            return Err("Не удалось найти файл".into());
        }

        if !file_manager.ok_permission_read() {
            return Err("Недостаточно прав для чтения данного файла".into());
        }

        let mut file = File::open(path_yaml_conf)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let parse_config = serde_yaml::from_str::<JexusConfigYaml>(&contents)?;
        Ok(parse_config)
    }
}

pub struct JxsValidConfig {
    pub main: JxsMain,
    pub http: JxsHttp,
}

pub struct JxsMain {
    pub worker_processes: usize,
    pub error_log: String,
    pub access_log: String,
    pub error_log_level: String,
}

pub struct JxsHttp {
    pub servers: Vec<JxsServer>,
}

pub struct JxsServer {
    pub listen: usize,
    pub root: String,
    pub locations: Vec<JxsLocation>,
}

pub struct JxsLocation {
    pub uri: Uri,
    pub index: String,
    pub auto_index: AutoIndex,
    pub random_index: RandomIndex,
    pub proxy_pass: String,
    pub fastcgi_pass: String,
}

impl JxsValidConfig {
    pub fn complied(config: JexusConfigYaml) -> Self {
        let valid_main = Self::validate_main(config.main);
        let valid_http = Self::validate_http(config.http);
        match valid_http {
            Ok(valid_http) => {
                Self {
                    main: valid_main,
                    http: valid_http,
                }
            }
            Err(error) => {
                panic!("{}", error)
            }
        }

    }

    fn validate_http(http: Http) -> Result<JxsHttp, Box<dyn std::error::Error>> {
        let servers = http.servers;
        let mut valid_servers: Vec<JxsServer> = Vec::new();
        for server in servers {
            let valid_server = Self::validate_server(server);
            match valid_server {
                Ok(valid_server) => valid_servers.push(valid_server),
                Err(error) => {
                    return Err(error); // пока так хуярим))
                }
            }
        }
        Ok(JxsHttp {
            servers: valid_servers,
        })
    }

    fn validate_server(server: Server) -> Result<JxsServer, Box<dyn std::error::Error>> {
        //check ports
        //на возможность открыть порт этому пользователю будем проверять в ServerManager`e
        if server.listen <= 0 && server.listen > 65535 {
            return Err("Не валидное значение для открытия порта".into())
        }

        let valid_server_listen = server.listen;

        // елси задали root то будем валидировать)
        // if !server.root.is_empty() {
        //     let fm: FileManager = FileManager::new_by_file(Path::new(server.root.as_str()));
        //
        //     if !fm.exists_file() {
        //         return Err("Не удалось найти файл root для location".into())
        //     }
        //     // todo тут еще на права бы проверить
        // }

        let valid_server_root = server.root;

        let locations = server.locations;
        let mut valid_locations: Vec<JxsLocation> = Vec::new();
        for location in locations {
            let valid_location = Self::validate_location(location);
            match valid_location {
                Ok (location) => {
                    valid_locations.push(location);
                }
                Err(error) => {
                    return Err(error); // пока так хуярим))
                }
            }
        }
        Ok(JxsServer {
            listen: valid_server_listen,
            root: valid_server_root,
            locations: valid_locations,
        })
    }

    fn validate_location(location: Location) -> Result<JxsLocation, Box<dyn std::error::Error>> {
        let uri: Uri = location.uri.parse().unwrap();
        if uri.path().is_empty() || !uri.path().starts_with("/") {
            return Err("Не валидное значение для uri в locations".into())
        }

        let mut count_handlers: i8 = 0;
        if !location.index.is_empty() {
            count_handlers += 1;
        }
        if location.auto_index == AutoIndex::On {
            count_handlers += 1;
        }
        if location.random_index == RandomIndex::On {
            count_handlers += 1;
        }
        if !location.proxy_pass.is_empty() {
            count_handlers += 1;
        }
        if !location.fastcgi_pass.is_empty() {
            count_handlers += 1;
        }

        if count_handlers > 1 {
            return Err("Указано не валидное количество обработчиков locations".into())
        }

        // если задан index
        if !location.index.is_empty() {
            if
                !location.index.ends_with(".html")
                    && !location.index.ends_with(".php")
                    && !location.index.ends_with(".json")
                    && !location.index.ends_with(".xml")
            {
                return Err("Указано не валидное значение index locations".into())
            }
        }

        // todo проверка на валидацию proxy_pass и на fastcgi

        Ok(JxsLocation {
            uri,
            index: location.index,
            auto_index: location.auto_index,
            random_index: location.random_index,
            proxy_pass: location.proxy_pass,
            fastcgi_pass: location.fastcgi_pass,
        })
    }

    fn validate_main(main: Main) -> JxsMain {
        JxsMain {
            worker_processes: Self::get_number_threads(main.worker_processes),
            error_log: main.error_log,
            access_log: main.access_log,
            error_log_level: main.error_log_level,
            //тут потом еще добавим проверок
        }
    }

    fn get_number_threads(worker_processes: WorkerProcesses) -> usize {
        let number_cpus : usize = num_cpus::get();
        match worker_processes {
            WorkerProcesses::Auto => {
                number_cpus// auto - количество потоков
            }
            WorkerProcesses::Number(worker_processes_count ) => {
                if worker_processes_count as usize > number_cpus {
                    panic!("worker_processes - set value, exceeding the number of cores by 10");
                }
                worker_processes_count as usize
            }
        }
    }
}