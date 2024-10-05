pub mod http_proxy_module {
    use crate::jxs::http::index::HttpModuleModule;
    
    pub struct ProxyModule {
        proxy_pass: String, // todo на какой нибудь удобо читаемый тип
    }

    impl HttpModuleModule for ProxyModule {
        fn get_handler(&self) -> () {
            todo!()
        }

    }
    
    impl ProxyModule {
        pub fn set_proxy_pass(&mut self, proxy_pass: String) -> &mut ProxyModule {
            self.proxy_pass = proxy_pass;
            self
        }

        pub fn build(&self) {

        }
    }

}