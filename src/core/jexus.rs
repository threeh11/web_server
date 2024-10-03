use crate::config::jexus_config::{JexusConfigReader};
use crate::core::jexus_server_manager::JexusServerManager;
use crate::logger::JexusLogger;

pub struct Jexus {
    pub jexus_config_reader: JexusConfigReader,
    pub jexus_logger: JexusLogger,
    pub jexus_server_manager: JexusServerManager,
}

impl Jexus {
    pub fn init() -> Self {

    }

}