use clap::Parser;
use reqwest::blocking::Client;
use reqwest::Proxy;
use serde::Deserialize;
use url::Url;

/// Check a proxy
#[derive(Parser)]
#[command(version)]
struct Cli {
    proxy: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    origin: String,
}

fn main() {
    let cli = Cli::parse();
    let proxy = Proxy::all(cli.proxy.clone()).expect("Can't create a proxy. Check the URL.");
    let res = Client::builder().proxy(proxy).build().unwrap().get("https://httpbin.org/ip").send().unwrap();
    let res: Response = res.json().expect("Can't parse the response from httpbin.org/ip.");
    if res.origin == parse_proxy_host(&cli.proxy) {
        println!("ok, ip: {}", res.origin)
    } else {
        println!("fail, ip: {}", res.origin)
    }
}

fn parse_proxy_host(url: &str) -> String {
    Url::parse(url).expect("Can't parse the proxy url.").host_str().expect("There is no host in the proxy url.").to_string()
}
