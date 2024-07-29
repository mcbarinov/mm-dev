use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct HttpbinIpResponse {
    origin: String,
}

/// Get the IP from httpbin.org/ip and that get the details from ip-api.com.
#[derive(Parser)]
#[command(version)]
struct Cli {}

fn main() -> Result<()> {
    let _cli = Cli::parse();

    // get my ip
    let my_ip = reqwest::blocking::get("https://httpbin.org/ip")
        .context("can't request httpbin.org")?
        .json::<HttpbinIpResponse>()
        .context("can't parse httpbin.org response")?
        .origin;
    println!("{}", &my_ip);

    // get ip info
    let ip_info = reqwest::blocking::get(format!("http://ip-api.com/json/{my_ip}"))
        .context("can't request ip-api.com")?
        .json::<Value>()
        .context("can't parse ip-api.com response")?;
    println!("{}", serde_json::to_string_pretty(&ip_info).unwrap());
    Ok(())
}
