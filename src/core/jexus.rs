use crate::config::default;
use crate::config::jexus_config::{JexusConfigYaml, JexusConfigParsed, Server};
use crate::core::jexus_server_manager::JexusServerManager;

pub struct Jexus {
    pub parsed_config: JexusConfigParsed<'static>,
    // pub jexus_logger: JexusLogger,
    pub jexus_server_manager: JexusServerManager<'static>,
}

impl Jexus {
    pub fn init() -> Self {
        let parsed_config: JexusConfigParsed = Self::get_parsed_config();
        let servers: &Vec<Server> = parsed_config.servers;
        let mut jexus_server_manager: JexusServerManager = JexusServerManager::new(servers);

        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(parsed_config.worker_processes)
            .enable_all()
            .build()
            .unwrap()
            .block_on(jexus_server_manager.setup_servers()).expect("TODO: panic message");

        Self {
            parsed_config,
            jexus_server_manager
        }
    }

    fn get_parsed_config() -> JexusConfigParsed<'static> {
        JexusConfigParsed::parse_by_yaml(JexusConfigYaml::new(default::CONFIG_PATH).unwrap())
    }

}