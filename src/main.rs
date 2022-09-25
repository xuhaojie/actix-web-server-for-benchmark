use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::header};
use mime::Mime;
//将 async main 函数标记为 actix 系统的入口点。 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //创建 http 服务器
    HttpServer::new(|| {
        App::new()//新建一个应用
			.configure(benchmark_routes)
    })
    .bind("0.0.0.0:3000")?//绑定到指定的套接字地址
    .run()//开始监听
    .await
}

pub fn benchmark_routes(cfg: &mut web::ServiceConfig) {
	cfg
	.route("/get", web::get().to(bench_get))
	.route("/post", web::post().to(bench_post))
	.route("/put", web::put().to(bench_put))
	.route("/delete", web::delete().to(bench_delete));
}
pub async fn bench_get() -> HttpResponse  {
	HttpResponse::Ok()
	.insert_header(header::ContentType(mime::APPLICATION_JSON))
	.body(
r#"{
"args": {},
"headers": {
	"Accept": "application/json",
	"Accept-Encoding": "gzip, deflate",
	"Accept-Language": "en-US,en;q=0.5",
	"Host": "www.httpbin.org",
	"Referer": "http://www.httpbin.org/",
	"User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:104.0) Gecko/20100101 Firefox/104.0",
	"X-Amzn-Trace-Id": "Root=1-632dce82-279a47540dd200b652a8cb02"
},
"origin": "113.200.214.222",
"url": "http://www.httpbin.org/get"
}"#)
}

pub async fn bench_post() -> HttpResponse  {
	HttpResponse::Ok()
	.insert_header(header::ContentType(mime::APPLICATION_JSON))
	.body(
r#"{
	"args": {},
	"data": "",
	"files": {},
	"form": {},
	"headers": {
		"Accept": "application/json",
		"Accept-Encoding": "gzip, deflate",
		"Accept-Language": "zh-cn",
		"Content-Length": "0",
		"Host": "httpbin.org",
		"Origin": "http://httpbin.org",
		"Referer": "http://httpbin.org/",
		"User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.2 Safari/605.1.15",
		"X-Amzn-Trace-Id": "Root=1-632dd138-2243642a76ba30163e857a96"
	},
	"json": null,
	"origin": "113.200.214.222",
	"url": "http://httpbin.org/put"
	}"#)
}

pub async fn bench_put() -> HttpResponse  {
	HttpResponse::Ok()
	.insert_header(header::ContentType(mime::APPLICATION_JSON))
	.body(
r#"{
	"args": {},
	"data": "",
	"files": {},
	"form": {},
	"headers": {
		"Accept": "application/json",
		"Accept-Encoding": "gzip, deflate",
		"Accept-Language": "zh-cn",
		"Content-Length": "0",
		"Host": "httpbin.org",
		"Origin": "http://httpbin.org",
		"Referer": "http://httpbin.org/",
		"User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.2 Safari/605.1.15",
		"X-Amzn-Trace-Id": "Root=1-632dd138-2243642a76ba30163e857a96"
	},
	"json": null,
	"origin": "113.200.214.222",
	"url": "http://httpbin.org/put"
	}"#)
}

pub async fn bench_delete() -> HttpResponse  {
	HttpResponse::Ok()
	.insert_header(header::ContentType(mime::APPLICATION_JSON))
	.body(
r#"{
	"args": {},
	"data": "",
	"files": {},
	"form": {},
	"headers": {
		"Accept": "application/json",
		"Accept-Encoding": "gzip, deflate",
		"Accept-Language": "zh-cn",
		"Content-Length": "0",
		"Host": "httpbin.org",
		"Origin": "http://httpbin.org",
		"Referer": "http://httpbin.org/",
		"User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.2 Safari/605.1.15",
		"X-Amzn-Trace-Id": "Root=1-632dd138-2243642a76ba30163e857a96"
	},
	"json": null,
	"origin": "113.200.214.222",
	"url": "http://httpbin.org/put"
	}"#)
}