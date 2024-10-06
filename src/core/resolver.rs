use std::collections::HashMap;
use std::net::{SocketAddr};
use crate::config::jexus_config::{JxsLocation, JxsServer, JxsValidConfig};
use crate::core::location::LocationInstance;
use crate::core::server::ServerInstance;
use crate::jxs::http::index::{HandlerFn, HttpModuleModule};
use crate::jxs::http::proxy::http_proxy_module;
use crate::jxs::http::proxy::http_proxy_module::ProxyModule;

pub struct JxsResolver {
    jxs_valid_config: Box<JxsValidConfig>
}

impl JxsResolver {
    pub fn new(jxs_valid_config: Box<JxsValidConfig>) -> Self {
        Self { jxs_valid_config }
    }
    pub fn build(&self) -> HashMap<Box<ServerInstance>, Vec<Box<LocationInstance>>> {
        let mut result_resolve: HashMap<Box<ServerInstance>, Vec<Box<LocationInstance>>> = HashMap::new();
        let valid_servers_into_config: Vec<JxsServer> = self.jxs_valid_config.http.servers;

        for server in valid_servers_into_config {
            let sock_addr = SocketAddr::from(([127, 0, 0, 1], server.listen as u16));
            let server_instance = ServerInstance::new()
                .set_port(server.listen as u16)
                .set_socket_addr(sock_addr)
                .set_root_dir(server.root)
                .build();

            let mut hash_map_location_handler: Vec<Box<LocationInstance>> = Vec::new();

            for location in server.locations {
                if !location.proxy_pass.is_empty() {
                    let http_proxy_module = ProxyModule::new()
                        .set_proxy_pass(location.proxy_pass)
                        .build();
                    let location_instance = LocationInstance::new(location.uri, http_proxy_module.get_handler());
                    hash_map_location_handler.push(Box::new(location_instance));
                }
            }

            result_resolve.insert(Box::new(*server_instance), hash_map_location_handler);
        }

        result_resolve
    }

}