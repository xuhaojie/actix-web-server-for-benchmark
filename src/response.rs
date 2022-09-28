pub const INDEX_RESPONSE: &str =
r#"<!DOCTYPE html>
<html>
<head>
	<meta charset='utf-8'>
	<title>HTTP(S) Benchmark Server</title>
	<link rel='stylesheet' type='text/css' media='screen' href='main.css'>
	<script src='main.js'></script>
</head>
<body>
	
	<div style="text-align:center;">
		<H1>HTTP(S) Benchmark Server</H1>
	</div>
</body>
</html>"#;

pub const GET_RESPONSE: &str =
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
}"#;

pub const POST_RESPONSE: &str =
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
}"#;

pub const PUT_RESPONSE: &str =
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
}"#;

pub const DELETE_RESPONSE: &str =
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
}"#;
