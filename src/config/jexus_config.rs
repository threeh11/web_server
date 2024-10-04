use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::path::Path;
use crate::os::file_manager;

#[derive(Debug, Deserialize, Serialize)]
pub struct JexusConfigYaml {
    #[serde(default)]
    pub main: Main,
    #[serde(default)]
    pub http: Http,
    #[serde(default)]
    pub mail: Mail,
    #[serde(default)]
    pub include: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Main {
    #[serde(deserialize_with = "deserialize_worker_processes")]
    pub worker_processes: WorkerProcesses,
    #[serde(default)]
    pub worker_connections: i32,
    #[serde(default)]
    pub pid: String,
    #[serde(default)]
    pub error_log: String,
    #[serde(default)]
    pub error_log_level: String,
    #[serde(default)]
    pub access_log: String,
    #[serde(default)]
    pub events: Events,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Http {
    #[serde(default)]
    pub servers: Vec<Server>,
    #[serde(default)]
    pub upstream: Upstream,
    #[serde(default)]
    pub gzip: Gzip,
    #[serde(default)]
    pub log_format: String,
    #[serde(default)]
    pub access_log: Vec<AccessLog>,
    #[serde(default)]
    pub error_page: ErrorPage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Events {
    #[serde(default)]
    pub worker_connections: i32,
    #[serde(default)]
    pub multi_accept: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    #[serde(default)]
    pub listen: usize,
    #[serde(default)]
    pub server_name: String,
    #[serde(default)]
    pub root: String,
    #[serde(default)]
    pub index: String,
    #[serde(default)]
    pub locations: Vec<Location>,
    #[serde(default)]
    pub ssl: Ssl,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    #[serde(default)]
    pub uri: String,
    #[serde(default)]
    pub proxy_pass: String,
    #[serde(default)]
    pub fastcgi_pass: String,
    #[serde(default)]
    pub return_code: i32,
    #[serde(default)]
    pub rewrite: Vec<RewriteRule>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RewriteRule {
    #[serde(default)]
    pub regex: String,
    #[serde(default)]
    pub replacement: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Upstream {
    #[serde(default)]
    pub servers: HashMap<String, UpstreamServer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpstreamServer {
    #[serde(default)]
    pub server: String,
    #[serde(default)]
    pub weight: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Gzip {
    #[serde(default)]
    pub on: bool,
    #[serde(default)]
    pub level: i32,
    #[serde(default)]
    pub types: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessLog {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub format: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorPage {
    #[serde(default)]
    pub error_pages: HashMap<i32, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mail {
    #[serde(default)]
    pub servers: Vec<MailServer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MailServer {
    #[serde(default)]
    pub listen: String,
    #[serde(default)]
    pub protocol: String,
    #[serde(default)]
    pub auth: Auth,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    #[serde(default)]
    pub methods: Vec<String>,
    #[serde(default)]
    pub password_file: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ssl {
    #[serde(default)]
    pub certificate: String,
    #[serde(default)]
    pub certificate_key: String,
    #[serde(default)]
    pub protocols: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum WorkerProcesses {
    Auto,
    Number(i32),
}

impl Default for WorkerProcesses {
    fn default() -> Self {
        WorkerProcesses::Number(1)
    }
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

impl Default for JexusConfigYaml {
    fn default() -> Self {
        JexusConfigYaml {
            main: Default::default(),
            http: Default::default(),
            mail: Default::default(),
            include: Default::default(),
        }
    }
}

impl Default for Main {
    fn default() -> Self {
        Main {
            worker_processes: Default::default(),
            worker_connections: 1024,
            pid: String::new(),
            error_log: String::new(),
            access_log: String::new(),
            error_log_level: String::new(),
            events: Default::default(),
        }
    }
}

impl Default for Http {
    fn default() -> Self {
        Http {
            servers: Default::default(),
            upstream: Default::default(),
            gzip: Default::default(),
            log_format: String::new(),
            access_log: Default::default(),
            error_page: Default::default(),
        }
    }
}

impl Default for Events {
    fn default() -> Self {
        Events {
            worker_connections: 1024,
            multi_accept: false,
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Server {
            listen: 80,
            server_name: String::new(),
            root: String::new(),
            index: String::new(),
            locations: Default::default(),
            ssl: Default::default(),
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Location {
            uri: String::new(),
            proxy_pass: String::new(),
            fastcgi_pass: String::new(),
            return_code: 0,
            rewrite: Default::default(),
        }
    }
}

impl Default for RewriteRule {
    fn default() -> Self {
        RewriteRule {
            regex: String::new(),
            replacement: String::new(),
        }
    }
}

impl Default for Upstream {
    fn default() -> Self {
        Upstream {
            servers: Default::default(),
        }
    }
}

impl Default for UpstreamServer {
    fn default() -> Self {
        UpstreamServer {
            server: String::new(),
            weight: 1,
        }
    }
}

impl Default for Gzip {
    fn default() -> Self {
        Gzip {
            on: false,
            level: 1,
            types: Default::default(),
        }
    }
}

impl Default for AccessLog {
    fn default() -> Self {
        AccessLog {
            path: String::new(),
            format: String::new(),
        }
    }
}

impl Default for ErrorPage {
    fn default() -> Self {
        ErrorPage {
            error_pages: Default::default(),
        }
    }
}

impl Default for Mail {
    fn default() -> Self {
        Mail {
            servers: Default::default(),
        }
    }
}

impl Default for MailServer {
    fn default() -> Self {
        MailServer {
            listen: String::new(),
            protocol: String::new(),
            auth: Default::default(),
        }
    }
}

impl Default for Auth {
    fn default() -> Self {
        Auth {
            methods: Default::default(),
            password_file: String::new(),
        }
    }
}

impl Default for Ssl {
    fn default() -> Self {
        Ssl {
            certificate: String::new(),
            certificate_key: String::new(),
            protocols: Default::default(),
        }
    }
}

impl JexusConfigYaml {

    pub fn new(path_yaml_conf: &str) -> Result<JexusConfigYaml, Box<dyn std::error::Error>>  {
        let path_yaml_conf = Path::new(path_yaml_conf);

        let file_manager = file_manager::FileManager::new_by_file(path_yaml_conf);
        if !file_manager.exists_file() {
            return Err("Не удалось найти файл".into());
        }

        // todo разобраться с правами, права есть но он все равно выдает ошибку
        // if !file_manager.ok_permission_read() {
        //     return Err("Недостаточно прав для чтения данного файла".into());
        // }

        let mut file = File::open(path_yaml_conf)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let parse_config = serde_yaml::from_str::<JexusConfigYaml>(&contents)?;
        Ok(parse_config)
    }
}


pub struct JexusConfigComplied {
    pub servers: Vec<Server>,
    pub worker_processes: usize,
}

impl JexusConfigComplied {
    pub fn complied(config: JexusConfigYaml) -> Self {
        let servers: Vec<Server> = config.http.servers;
        let worker_processes: usize = Self::get_number_threads(config.main.worker_processes);
        Self {
            servers,
            worker_processes,
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