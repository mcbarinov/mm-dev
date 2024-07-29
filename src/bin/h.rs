use clap::Parser;
use mm_dev::{exit, shell, user_input};

/// Manage Hetzner Cloud. It's a wrapper around the `hcloud` cli.
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    #[command(alias = "l")]
    List(ListCmd),

    #[command(alias = "r")]
    Rebuild(RebuildCmd),

    #[command(alias = "d")]
    Delete(DeleteCmd),
}

#[derive(Parser)]
struct ListCmd {}

#[derive(Parser)]
struct RebuildCmd {
    /// Server name
    server: String,
}

#[derive(Parser)]
struct DeleteCmd {
    /// Server name
    server: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::List(_) => list_cmd(),
        Command::Rebuild(cmd) => rebuild_cmd(cmd.server),
        Command::Delete(cmd) => delete_cmd(cmd.server),
    }
}

fn list_cmd() {
    shell("hcloud server list -o columns=name,ipv4,private_net,datacenter,status,type,volumes");
}

fn rebuild_cmd(server: String) {
    if server != "test" {
        let confirm = user_input("Sure? Type the server name again: ");
        if server != confirm {
            exit(&format!("Confirm failed! {} != {}", server, confirm))
        }
    }
    shell(&format!("hcloud server rebuild '{}' --image=ubuntu-24.04", server));
    shell(&format!("dh {}", server));
}

fn delete_cmd(server: String) {
    if server == "test" {
        exit("Can't delete 'test' server. Do it via hcloud directly.")
    }
    let confirm = user_input("Sure? Type the server name again: ");
    if server != confirm {
        exit(&format!("Confirm failed! {} != {}", server, confirm))
    }
    shell(&format!("hcloud server delete '{}'", server));
    shell(&format!("dh {}", server));
}
