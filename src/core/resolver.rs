use std::convert::Infallible;
use std::fs;
use std::net::{SocketAddr};
use std::path::Path;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Body, Request, Response, StatusCode};
use crate::config::jexus_config::{JxsServer, JxsValidConfig};
use crate::core::location::LocationInstance;
use crate::core::server::VirtualHost;
use crate::jxs::http::proxy::http_proxy_module::ProxyModule;

pub struct JxsResolver {
    jxs_valid_config: Box<JxsValidConfig>
}

impl JxsResolver {
    pub fn new(jxs_valid_config: Box<JxsValidConfig>) -> Self {
        Self { jxs_valid_config }
    }

    pub fn build(&self) -> Vec<Box<VirtualHost>> {
        let mut result_resolve: Vec<Box<VirtualHost>> = Vec::new();
        let valid_servers_into_config: Vec<JxsServer> = self.jxs_valid_config.http.servers.clone();

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

    async fn handler(&self, request: Request<Body>) -> Result<Response<Full<Bytes>>, Infallible> {
        let user_agent = request.headers().get("User-Agent").map(|value| value.to_str().unwrap_or("Unknown")).unwrap_or("Unknown");
        // info!("Request: {} {} - User-Agent: {}", request.method(), request.uri(), user_agent);
        let base_path = Path::new("/home/threeh/test");
        let request_path = Path::new(request.uri().path());

        let request_path_str = request_path.to_str().expect("Invalid Unicode in path");

        let request_path_str = if request_path_str.starts_with('/') {
            &request_path_str[1..]
        } else {
            request_path_str
        };

        let full_path = base_path.join(Path::new(request_path_str));

        match fs::read(full_path) {
            Ok(contents) => Ok(Response::new(Full::from(Bytes::from(contents)))),
            Err(_) => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::from("File not found"))
                .unwrap()),
        }
    }

}