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
    /// Install docker via local.mm.install_docker
    Docker { host: String },

    /// Install Caddy via local.mm.install_caddy
    Caddy { host: String, domain: Option<String> },
}

fn main() {
    match Cli::parse().command {
        Command::Docker { host } => {
            let cmd = format!("ANSIBLE_LOAD_CALLBACK_PLUGINS=true ANSIBLE_STDOUT_CALLBACK=yaml ansible -i {host}, all -m include_role -a name=local.mm.install_docker");
            shell(&cmd)
        }

        Command::Caddy { host, domain } => {
            let mut cmd = format!("ANSIBLE_LOAD_CALLBACK_PLUGINS=true ANSIBLE_STDOUT_CALLBACK=yaml ansible -i {host}, all -m include_role -a name=local.mm.install_caddy");
            if domain.is_some() {
                cmd += &format!(" -e domain={}", domain.unwrap());
            }
            shell(&cmd)
        }
    }
}
