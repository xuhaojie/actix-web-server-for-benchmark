use actix_web::{get, web, App, HttpRequest, HttpServer, Responder,HttpResponse, http::header};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use anyhow::{*, Result};
use clap::{Arg,  Command};
use {log::*, dotenv};
use mime;

const DEFAULT_IP : &str = "0.0.0.0";
const DEFAULT_PORT : &str = "3080";
const DEFAULT_SSL_PORT : &str = "3443";

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
}

#[actix_web::main]
async fn main() -> Result<()> {
	env_logger::init();
	dotenv::dotenv().ok();




	let cmd = Command::new("bench_server")
						  .version("1.0")
						  .author("Xu Haojie <xuhaojie@hotmail.com>")
						  .about("A simple http(s) server for benchmark")
						  .arg(Arg::with_name("ip")
						  	.short('i')
							.value_name("ip")
							.takes_value(true)
							.default_missing_value("0.0.0.0")
						  	.help("Server bind ip, default 0.0.0.0"))
						  .arg(Arg::with_name("port")
						  	.short('p')
							.value_name("http port")
							.takes_value(true)
							.default_missing_value("3080")
						  	.help("Http server port, default 3080"))
						  .arg(Arg::with_name("https")
						  	.short('s')
							.value_name("https port")
							.takes_value(true)
						  	.help("Enable and specify the https server port"));
		

	let server = HttpServer::new(|| App::new().configure(benchmark_routes));
	
	let matches = cmd.get_matches();

	

	let ip = matches.value_of("ip").unwrap_or(DEFAULT_IP);


	let port: u16 = matches.value_of("port").unwrap_or(DEFAULT_PORT).parse()?;


	let http_address = format!("{}:{}",ip, port);

	info!("http server listen on {}", http_address);
	
	let server = server.bind(http_address)?;

	if matches.is_present("https") {				   
		let https_port: u16 = matches.value_of("https").unwrap_or(DEFAULT_SSL_PORT).parse()?;
		let https_address = format!("{}:{}",ip, https_port);
		info!("https server listen on {}", https_address);

		// load TLS keys
		// to create a self-signed temporary cert for testing:
		// `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`		
		let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
		builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
	    builder.set_certificate_chain_file("cert.pem").unwrap();
		
		server.bind_openssl(https_address, builder)?.run().await?;
	}else{
		server.run().await?;
	}
	
	Ok(())

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