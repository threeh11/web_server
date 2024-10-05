use std::net::{SocketAddr};
use crate::config::jexus_config::{JxsServer, JxsValidConfig};
use crate::core::location::LocationInstance;
use crate::core::server::ServerInstance;

pub struct JxsResolver {
    jxs_valid_config: Box<JxsValidConfig>
}

impl JxsResolver {
    pub fn new(jxs_valid_config: Box<JxsValidConfig>) -> Self {
        Self { jxs_valid_config }
    }
    // -> HashMap<Box<Vec<JxsServer>>, HashMap<Box<Vec<JxsLocation>>, Self/*тут будет обработчик*/>>
    pub fn build(&self)  {
        let valid_servers_into_config: Vec<JxsServer> = self.jxs_valid_config.http.servers;
        let mut servers_instance: Vec<Box<ServerInstance>> = Vec::new();
        for server in valid_servers_into_config {
            let sock_addr = SocketAddr::from(([127, 0, 0, 1], server.listen));
            // todo -> server_instancr -> Result
            // потом match {}
            let server_instance = ServerInstance::new()
                .set_port(server.listen as u16)
                .set_socket_addr(sock_addr)
                .set_root_dir(server.root)
                .build();
            servers_instance.push(server_instance);
            for location in server.locations {
                // тут генерим обработчик из модулей
                let location_instance = LocationInstance::new(location.uri, /*здеся будет обработчик*/)
            }
        }
    }

}