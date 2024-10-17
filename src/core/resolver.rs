use std::net::{SocketAddr};
use crate::config::jexus_config::{JxsServer, JxsValidConfig};
use crate::core::location::LocationInstance;
use crate::core::server::VirtualHost;
use crate::jxs::http::proxy::http_proxy_module::ProxyModule;

pub struct JxsResolver {
    jxs_valid_config: Box<JxsValidConfig>
}

impl JxsResolver {
    pub fn resolve(jxs_valid_config: Box<JxsValidConfig>) -> Vec<Box<VirtualHost<'static>>> {
        let mut result_resolve: Vec<Box<VirtualHost>> = Vec::new();
        let valid_servers_into_config: Vec<JxsServer> = jxs_valid_config.http.servers.clone();

        for server in valid_servers_into_config {
            let sock_addr = SocketAddr::from(([127, 0, 0, 1], server.listen as u16));
            let mut virtual_host = VirtualHost::new()
                .set_port(server.listen as u16)
                .set_socket_addr(sock_addr)
                .build();

            let mut locations: Vec<LocationInstance> = Vec::new();

            let iter_locations = server.locations.clone();

            for location in iter_locations {
                if !location.proxy_pass.is_empty() {
                    let http_proxy_module = ProxyModule::new();

                    let location_instance = LocationInstance::new(location.uri, http_proxy_module);
                    locations.push(location_instance);

                }
            }

            virtual_host.set_locations(locations);

            result_resolve.push(virtual_host);
        }

        result_resolve
    }

}