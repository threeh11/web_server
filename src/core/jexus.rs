use std::error::Error;
use crate::config::default;
use crate::config::jexus_config::{JexusConfigYaml, JexusConfigComplied};
// use crate::core::jexus_server_manager::JexusServerManager;

pub struct Jexus {
    pub parsed_config: JexusConfigComplied,
    // pub jexus_logger: JexusLogger,
    // pub jexus_server_manager: JexusServerManager<'static>,
}

impl Jexus {
    pub fn init() -> Self {
        let parsed_config: JexusConfigComplied = Self::get_complied_config().unwrap();
        // let servers: Vec<Server> = parsed_config.servers;
        // let mut jexus_server_manager: JexusServerManager = JexusServerManager::new(servers);
        //
        // tokio::runtime::Builder::new_multi_thread()
        //     .worker_threads(parsed_config.worker_processes)
        //     .enable_all()
        //     .build()
        //     .unwrap()
        //     .block_on(jexus_server_manager.setup_servers()).expect("TODO: panic message");

        Self {
            parsed_config,
            // jexus_server_manager
        }
    }

    fn get_complied_config() -> Result<JexusConfigComplied, Box<dyn Error>> {
        match JexusConfigYaml::new(default::CONFIG_PATH) {
            Ok(config) => Ok(JexusConfigComplied::complied(config)),
            Err(err) => Err(format!("Ошибка при работе с файлом конфигурации: {}", err).into()),
        }
    }

}