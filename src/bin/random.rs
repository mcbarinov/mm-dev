use clap::Parser;
use rand::distributions::Alphanumeric;
use rand::Rng;

/// Generate a random string
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(long, short, default_value = "16")]
    length: u8,
}

fn main() {
    let cli = Cli::parse();
    let s: String = rand::thread_rng().sample_iter(&Alphanumeric).take(cli.length as usize).map(char::from).collect();
    println!("{}", s);
}
