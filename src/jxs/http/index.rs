pub trait HttpModuleModule {
    fn get_handler(&self) -> ();
}

pub mod http_index_module {
    use crate::jxs::http::index::HttpModuleModule;

    // Возвращает статичные файлы
    pub mod index_module {
        use crate::jxs::http::index::HttpModuleModule;

        pub struct IndexModule {

        }

        impl HttpModuleModule for IndexModule {
            fn get_handler(&self) -> () {
                todo!()
            }
        }

    }

    // Возвращает список каталога
    pub mod list_index_module {
        use crate::jxs::http::index::HttpModuleModule;

        pub struct IndexListModule {

        }

        impl HttpModuleModule for IndexListModule {
            fn get_handler(&self) -> () {
                todo!()
            }
        }
    }

    // потом сюда штуки с кешем положим
}