use std::error::Error;
use crate::config::default;
use crate::config::jexus_config::{JexusConfigYaml, JxsValidConfig};
use crate::core::jexus_logger::{JxsLogger, LevelsLogger};

pub struct Jexus {
    jexus_valid_config: JxsValidConfig,
    jexus_logger: JxsLogger,
    // pub jexus_server_manager: JexusServerManager<'static>,
}

impl Jexus {
    pub fn init() -> Self {
        let parsed_config: JxsValidConfig = Self::get_complied_config().unwrap();
        let logger: JxsLogger = JxsLogger::new(&parsed_config.main);
        // println!("{:?}", logger);
        logger.log_write(LevelsLogger::Error, "Request FastCGI is not correct");
        logger.log_write(LevelsLogger::Info, "Proxy successfull");
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
            jexus_valid_config: parsed_config,
            jexus_logger: logger,
            // jexus_server_manager
        }
    }

    fn get_complied_config() -> Result<JxsValidConfig, Box<dyn Error>> {
        match JexusConfigYaml::parse(default::CONFIG_PATH) {
            Ok(config) => Ok(JxsValidConfig::complied(config)),
            Err(err) => Err(format!("Ошибка при работе с файлом конфигурации: {}", err).into()),
        }
    }

}