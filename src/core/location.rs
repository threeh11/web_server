use hyper::{Response, Uri};
use crate::jxs::http::proxy::http_proxy_module::ProxyModule;

#[derive(Clone)]
pub struct LocationInstance {
    uri: Uri,
    module: ProxyModule
}

impl LocationInstance {
    pub fn new(uri: Uri, module: ProxyModule) -> Self {
        Self {
            uri,
            module: module.clone()
        }
    }
}