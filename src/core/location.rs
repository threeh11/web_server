use hyper::Uri;
use crate::jxs::http::proxy::http_proxy_module;

pub struct LocationInstance<T: http_proxy_module::HttpModuleModule> {
    uri: Uri,
    http_module: T,
}

impl<T: http_proxy_module::HttpModuleModule> LocationInstance<T>{
    pub fn new(uri: Uri, http_module: T) -> Self {
        //todo кароч, сюды будем класть uri, и дефолтный обработчик из модуля, только перед этим его создадим
        Self { uri, http_module }
    }

    pub fn get_handlers(&self) {
        self.http_module.get_handlers();
    }

}