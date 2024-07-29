use clap::Parser;
use num::{BigInt, Num};

/// Convert hex to decimal
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    value: String,
}

fn main() {
    let cli = Cli::parse();
    match BigInt::from_str_radix(cli.value.to_lowercase().trim_start_matches("0x"), 16) {
        Ok(res) => println!("{}", res),
        Err(e) => eprintln!("error: {}", e),
    }
}
