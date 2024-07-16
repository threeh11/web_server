//Cache-Control — это HTTP-заголовок, который используется для управления кэшированием
//данных в промежуточных серверах и браузерах. Этот заголовок позволяет указать, как долго данные
//могут быть сохранены в кэше, и при каких условиях они могут быть использованы повторно.
//Cache-Control может использоваться как в запросах, так и в ответах.

// Пример вызова
// let cache_control = CacheControl::new()
//         .caching_method(false) // public
//         .no_cache(false)
//         .no_store(false)
//         .max_age(3600)
//         .build();
//
//     match cache_control {
//         Ok(cc) => println!("{:?}", cc),
//         Err(e) => println!("Error: {:?}", e),
//     }

// @todo подумать куда таки шутки выносить
struct CachingMethodType(u8);
pub const PRIVATE_CACHING_METHOD: CachingMethodType = 0;
pub const PUBLIC_CACHING_METHOD: CachingMethodType = 1;

#[derive(Debug, PartialEq)]
pub struct CacheControl {
    caching_method: Option<CachingMethodType>,
    no_cache: Option<bool>,
    no_store: Option<bool>,
    only_if_cached: Option<bool>,
    proxy_revalidate: Option<bool>,
    must_revalidate: Option<bool>,
    max_age: Option<u32>,
    max_stale: Option<u32>,
    min_fresh: Option<u32>,
    s_maxage: Option<u32>,
}

// @todo почитать как вообще ошибки обрабатываются
pub struct CacheControlError {
    // тут Будут ошибки...
    message: String,
}

impl CacheControl {
    pub fn new() -> Self {
        Self {
            caching_method: None,
            no_cache: None,
            no_store: None,
            only_if_cached: None,
            proxy_revalidate: None,
            must_revalidate: None,
            max_age: None,
            max_stale: None,
            min_fresh: None,
            s_maxage: None,
        }
    }

    pub fn caching_method(&mut self, value: CachingMethodType) -> &mut Self {
        self.caching_method = Some(value);
        self
    }

    pub fn no_cache(&mut self, value: bool) -> &mut Self {
        self.no_cache = Some(value);
        self
    }

    pub fn no_store(&mut self, value: bool) -> &mut Self {
        self.no_store = Some(value);
        self
    }

    pub fn only_if_cached(&mut self, value: bool) -> &mut Self {
        self.only_if_cached = Some(value);
        self
    }

    pub fn proxy_revalidate(&mut self, value: bool) -> &mut Self {
        self.proxy_revalidate = Some(value);
        self
    }

    pub fn must_revalidate(&mut self, value: bool) -> &mut Self {
        self.must_revalidate = Some(value);
        self
    }

    pub fn max_age(&mut self, value: u32) -> &mut Self {
        self.max_age = Some(value);
        self
    }

    pub fn max_stale(&mut self, value: u32) -> &mut Self {
        self.max_stale = Some(value);
        self
    }

    pub fn min_fresh(&mut self, value: u32) -> &mut Self {
        self.min_fresh = Some(value);
        self
    }

    pub fn s_maxage(&mut self, value: u32) -> &mut Self {
        self.s_maxage = Some(value);
        self
    }

    // Паттерн называется строитель)
    pub fn build(&self) -> Result<Self, CacheControlError> {
        // Тут надо будет запилить всевозможную валидацию
        if PRIVATE_CACHING_METHOD != self.caching_method && PUBLIC_CACHING_METHOD != self.caching_method {
            return Err(CacheControlError {
                message: "caching_method contains an invalid value".to_string(),
            });
        }

        if self.no_cache == Some(true) && self.no_store == Some(true) {
            return Err(CacheControlError {
                message: "no_cache and no_store cannot be both true".to_string(),
            });
        }

        Ok(Self{
            caching_method: self.caching_method,
            no_cache: self.no_cache,
            no_store: self.no_store,
            only_if_cached: self.only_if_cached,
            proxy_revalidate: self.proxy_revalidate,
            must_revalidate: self.must_revalidate,
            max_age: self.max_age,
            max_stale: self.max_stale,
            min_fresh: self.min_fresh,
            s_maxage: self.s_maxage,
        })
    }

}