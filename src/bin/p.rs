use clap::Parser;
use mm_dev::{exit, shell};
use std::env;
use std::path::Path;

/// Python dev helper
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// pip list
    #[command(alias = "l")]
    List,

    /// pip list -o
    #[command(alias = "o")]
    ListOutdated,

    /// Install packages
    #[command(alias = "i")]
    Install(InstallCmd),

    /// Uninstall packages
    #[command(alias = "d")]
    Uninstall,

    /// python3 -m venv .venv
    #[command(alias = "v")]
    Venv,
}

#[derive(Parser)]
struct InstallCmd {
    /// Install packages. If empty, install from requirements.txt
    packages: Vec<String>,
}

fn main() {
    match Cli::parse().command {
        Command::List => shell("uv pip list"),
        Command::ListOutdated => shell("pip3 list -o"),
        Command::Install(cmd) => {
            if env::var_os("VIRTUAL_ENV").is_none() {
                exit("venv is not activated")
            }
            if !cmd.packages.is_empty() {
                shell(&format!("uv pip install {}", cmd.packages.join(" ")))
            } else {
                shell("uv pip install -Ur requirements.txt")
            }
        }
        Command::Uninstall => {
            if env::var_os("VIRTUAL_ENV").is_none() {
                exit("venv is not activated")
            }
            shell("uv pip list --format freeze -e | xargs uv pip uninstall");
            shell("uv pip freeze | xargs uv pip uninstall");
        }
        Command::Venv => {
            if env::var_os("VIRTUAL_ENV").is_some() {
                exit("venv is activated already")
            }
            if Path::new(".venv").exists() {
                exit(".venv exists already")
            }
            shell("uv venv");
        }
    }
}
