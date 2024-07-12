use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use serde_yaml;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub main: Main,
    pub http: Http,
    pub mail: Mail,
    pub include: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Main {
    pub worker_processes: i32,
    pub worker_connections: i32,
    pub pid: String,
    pub error_log: String,
    pub events: Events,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Http {
    pub servers: Vec<Server>,
    pub upstream: Upstream,
    pub gzip: Gzip,
    pub log_format: String,
    pub access_log: Vec<AccessLog>,
    pub error_page: ErrorPage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Events {
    pub worker_connections: i32,
    pub multi_accept: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub listen: String,
    pub server_name: String,
    pub root: String,
    pub index: String,
    pub locations: Vec<Location>,
    pub ssl: Ssl,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub uri: String,
    pub proxy_pass: String,
    pub fastcgi_pass: String,
    pub return_code: i32,
    pub rewrite: Vec<RewriteRule>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RewriteRule {
    pub regex: String,
    pub replacement: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Upstream {
    pub servers: std::collections::HashMap<String, UpstreamServer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpstreamServer {
    pub server: String,
    pub weight: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Gzip {
    pub on: bool,
    pub level: i32,
    pub types: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessLog {
    pub path: String,
    pub format: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorPage {
    pub error_pages: std::collections::HashMap<i32, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mail {
    pub servers: Vec<MailServer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MailServer {
    pub listen: String,
    pub protocol: String,
    pub auth: Auth,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    pub methods: Vec<String>,
    pub password_file: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ssl {
    pub certificate: String,
    pub certificate_key: String,
    pub protocols: Vec<String>,
}

impl Config {
    pub fn new(file_name: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let current_dir = std::env::current_dir()?;
        let config_path = current_dir.join(file_name);
        let mut file = File::open(config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = serde_yaml::from_str(&contents)?;

        Ok(config)
    }
}