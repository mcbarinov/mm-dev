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

    /// Setup Ubuntu via local.mm.bootstrap_ubuntu
    Base { host: String },
}

fn main() {
    match Cli::parse().command {
        Command::Docker { host } => shell(&ansible_command(host, "local.mm.install_docker")),
        Command::Caddy { host, domain } => {
            let mut cmd = ansible_command(host, "local.mm.install_caddy");
            if domain.is_some() {
                cmd += &format!(" -e domain={}", domain.unwrap());
            }
            shell(&cmd)
        }
        Command::Base { host } => shell(&ansible_command(host, "local.mm.bootstrap_ubuntu")),
    }
}

fn ansible_command(host: String, role: &str) -> String {
    let env = "ANSIBLE_LOAD_CALLBACK_PLUGINS=true ANSIBLE_STDOUT_CALLBACK=yaml";
    format!("{env} ansible -i {host}, all -m include_role -a name={role}")
}
