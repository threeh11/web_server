//Подумать куда это говон вытащить
pub struct HttpVersion(u8);

const HTTP1_1: HttpVersion = "HTTP/1.1";
const HTTP2: HttpVersion = "HTTP2";
const HTTP3: HttpVersion = "HTTP3";

struct StatusLine {
    http_version: HttpVersion,
    status_code: u16,
    descr_status_code: HttpStatus,
}

struct Headers {

}

pub struct HttpResponse{
    status_line: StatusLine
}