//Cache-Control — это HTTP-заголовок, который используется для управления кэшированием
//данных в промежуточных серверах и браузерах. Этот заголовок позволяет указать, как долго данные
//могут быть сохранены в кэше, и при каких условиях они могут быть использованы повторно.
//Cache-Control может использоваться как в запросах, так и в ответах.
#[derive(Debug, PartialEq)]
pub struct CacheControl {
    caching_method: bool, // public - false, private - 1
    no_cache: bool,
    no_store: bool,
    only_if_cached: bool,
    proxy_revalidate: bool,
    must_revalidate: bool,
    max_age: u32,
    max_stale: u32,
    min_fresh: u32,
    s_maxage: u32,
}

pub struct CacheControlError {
    // тут Будут ошибки...
}

impl CacheControl {
    pub fn new(&self) -> Result<CacheControl, CacheControlError> {

    }

}