use std::error::Error;
use crate::config::default;
use crate::config::jexus_config::{JexusConfigYaml, JxsValidConfig};
use crate::core::resolver::JxsResolver;
use crate::core::server::VirtualHost;

pub struct Jexus {
    pub jxs_valid_config: JxsValidConfig,
    pub resolver: JxsResolver,
    // pub jexus_logger: JexusLogger,
    // pub jexus_server_manager: JexusServerManager<'static>,
}

impl Jexus {
    pub fn init() {
        let parsed_config: JxsValidConfig = Self::get_complied_config().unwrap();
        let binding = JxsResolver::new(Box::new(parsed_config));
        let resolver: Vec<Box<VirtualHost>> = binding.build();
        let tmp = resolver;
        // let servers: Vec<Server> = parsed_config.servers;
        // let mut jexus_server_manager: JexusServerManager = JexusServerManager::new(servers);
        //
        // tokio::runtime::Builder::new_multi_thread()
        //     .worker_threads(parsed_config.worker_processes)
        //     .enable_all()
        //     .build()
        //     .unwrap()
        //     .block_on(jexus_server_manager.setup_servers()).expect("TODO: panic message");
    }

    fn get_complied_config() -> Result<JxsValidConfig, Box<dyn Error>> {
        match JexusConfigYaml::parse(default::CONFIG_PATH) {
            Ok(config) => Ok(JxsValidConfig::complied(config)),
            Err(err) => Err(format!("Ошибка при работе с файлом конфигурации: {}", err).into()),
        }
    }

}