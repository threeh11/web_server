pub mod http_proxy_module {
    pub struct ProxyModule {
        proxy_pass: String, // todo на какой нибудь удобо читаемый тип
    }

    impl ProxyModule {
        pub fn set_proxy_pass(&self, proxy_pass: String) -> Self {
            self.proxy_pass = proxy_pass;
            return self
        }

        pub fn build(&self) {

        }

    }
}