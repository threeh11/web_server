mod config;
mod logger;
mod core;

use bytes::Bytes;
use std::convert::Infallible;
use http_body_util::Full;

use std::net::{SocketAddr};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener};
use hyper::{Request, Response, StatusCode};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::{TokioIo};
use crate::config::config_parser::{Config, ConfigResolver, ServerByYaml, WorkerProcesses};
use std::collections::HashMap;
use std::{fs};
use log::{error, info};
use uuid::{ContextV7, Timestamp, Uuid};
use crate::core::server_manager::ServerManager;
use crate::logger::{JexusLogger, LevelsLogger};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new("src/config/config.yaml")?;
    let jexus_params: ConfigResolver = ConfigResolver::get_parameters_by_config(config);
    let mut server_manager: ServerManager = ServerManager::new(jexus_params.servers);

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(jexus_params.worker_processes)
        .enable_all()
        .build()
        .unwrap()
        .block_on(server_manager.setup_servers()).expect("TODO: panic message");

    Ok(())
}
