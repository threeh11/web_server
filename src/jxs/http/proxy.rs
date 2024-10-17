pub mod http_proxy_module {

    #[derive(Clone)]
    pub struct ProxyModule {
        proxy_pass: String, // todo на какой нибудь удобо читаемый тип
    }


    impl ProxyModule {
        pub fn new() -> Self {
            Self {
                proxy_pass: "".to_string(),
            }
        }

        pub fn set_proxy_pass(&mut self, proxy_pass: String) -> &mut ProxyModule {
            self.proxy_pass = proxy_pass;
            self
        }

        pub fn build(&self) -> Box<Self> {
            Box::new(Self {
                proxy_pass: self.proxy_pass.clone(),
            })
        }
    }
}