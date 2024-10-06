use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response};
use hyper::body::Body;

pub type HandlerFn = Box<dyn Fn(Request<Body>)
    -> Pin<Box<dyn Future<Output = Result<Response<Full<Bytes>>, Infallible>> + Send>>>;

pub trait HttpModuleModule {
    fn get_handler(&self,) -> HandlerFn {
        let handler = self.clone();
        Box::new(move |req: Request<Body>| {
            Box::pin(handler.handler(req))
        })
    }
    async fn handler(&self, request: Request<Body>) -> Result<Response<Full<Bytes>>, Infallible>;
}

// Возвращает статичные файлы
pub mod http_index_module {
    use std::convert::Infallible;
    use bytes::Bytes;
    use http_body_util::Full;
    use hyper::{Request, Response};
    use hyper::body::Body;
    use crate::jxs::http::index::{HandlerFn, HttpModuleModule};

    pub struct IndexModule {

    }

    impl HttpModuleModule for IndexModule {
        async fn handler(&self, request: Request<Body>) -> Result<Response<Full<Bytes>>, Infallible> {
            todo!()
        }
    }

}

// Возвращает список каталога
pub mod http_list_index_module {
    use crate::jxs::http::index::HttpModuleModule;
    use std::convert::Infallible;
    use bytes::Bytes;
    use http_body_util::Full;
    use hyper::{Request, Response};
    use hyper::body::Body;

    pub struct IndexListModule {

    }

    impl HttpModuleModule for IndexListModule {

        async fn handler(&self, request: Request<Body>) -> Result<Response<Full<Bytes>>, Infallible> {
            todo!()
        }
    }
}

// потом сюда штуки с кешем положим
