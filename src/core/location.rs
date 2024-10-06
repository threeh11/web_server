use hyper::Uri;
use crate::jxs::http::index::HandlerFn;
use crate::jxs::http::proxy::http_proxy_module;

pub struct LocationInstance {
    uri: Uri,
    pub get_handler: HandlerFn,
}

impl LocationInstance{
    pub fn new(uri: Uri, get_handler: HandlerFn) -> Self {
        Self { uri, get_handler }
    }

}