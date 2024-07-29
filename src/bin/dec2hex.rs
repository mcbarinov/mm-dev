use clap::Parser;

/// Convert decimal to hex
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    value: u128,
}

fn main() {
    let cli = Cli::parse();
    println!("0x{:x}", cli.value);
}
