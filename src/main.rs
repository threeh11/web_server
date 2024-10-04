mod config;
mod core;
// mod logger;
// mod core;
// use tokio::io::AsyncWriteExt;
// use hyper_util::rt::{TokioIo};
use crate::config::jexus_config::JexusConfig;
use crate::core::jexus_logger::JexusLogger;
// use crate::core::jexus_server_manager::ServerManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: JexusConfig = JexusConfig::new("src/config/config.yaml")?;
    let logger: JexusLogger<'_> = JexusLogger::new(&config.main);
    println!("{:?}", logger);
    // let jexus_params: JexusConfigReader = ConfigResolver::get_parameters_by_config(config);
    // let mut server_manager: ServerManager = ServerManager::new(jexus_params.servers);

    // tokio::runtime::Builder::new_multi_thread()
    //     .worker_threads(jexus_params.worker_processes)
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(server_manager.setup_servers()).expect("TODO: panic message");

    Ok(())
}
