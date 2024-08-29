use clap::Parser;
use mm_dev::shell;

/// A helper for ansible.
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// install docker via local.mm.install_docker
    Docker { host: String },
}

fn main() {
    match Cli::parse().command {
        Command::Docker { host } => {
            let cmd = format!("ANSIBLE_LOAD_CALLBACK_PLUGINS=true ANSIBLE_STDOUT_CALLBACK=yaml ansible -i {host}, all --module-name include_role --args name=local.mm.install_docker");
            shell(&cmd)
        }
    }
}
