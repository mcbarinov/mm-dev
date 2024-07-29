use clap::Parser;
use dns_lookup::lookup_host;
use mm_dev::shell;

/// Delete records from .ssh/known_hosts
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    /// List of servers
    #[arg(required = true)]
    servers: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    for s in cli.servers {
        delete_host(&s);
    }
}

fn delete_host(host: &str) {
    shell(&format!("ssh-keygen -R {}", host));
    if let Ok(ips) = lookup_host(host) {
        for h in ips.iter() {
            shell(&format!("ssh-keygen -R {}", h));
        }
    }
}
