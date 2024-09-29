use clap::Parser;
use mm_dev::shell;

/// A helper for git.
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// git diff
    #[command(alias = "d")]
    Diff,

    /// git log --pretty=format:"%cr / %an / %s"
    #[command(alias = "l")]
    Log,

    /// git tag --sort=-creatordate
    #[command(alias = "t")]
    Tag,

    /// git status --untracked-files --short
    #[command(alias = "s")]
    Status,

    /// git clone $repo
    #[command(alias = "c")]
    Clone(CloneCmd),

    /// git add . && git commit -m $message && git push
    #[command(alias = "p")]
    Push(PushCmd),

    /// add tag and push: git tag -a $version -m $version && git push origin $version
    #[command(alias = "at")]
    AddTag(AddTagCmd),

    /// delete tag and push: git tag -d $version && git push origin :refs/tags/$version
    #[command(alias = "dt")]
    DeleteTag(DeleteTagCmd),

    /// amend the last commit: git add . && git commit --amend --no-edit && git push --force
    #[command(alias = "amend")]
    Amend,
}

#[derive(Parser)]
struct CloneCmd {
    repo: String,
}

#[derive(Parser)]
struct PushCmd {
    #[arg(default_value = "update")]
    message: String,
}

#[derive(Parser, Debug)]
struct AddTagCmd {
    version: String,
}

#[derive(Parser, Debug)]
struct DeleteTagCmd {
    version: String,
}

fn main() {
    match Cli::parse().command {
        Command::Diff => shell("git diff"),
        Command::Log => shell(r#"git log --pretty=format:"%ar / %an / %s""#),
        Command::Tag => shell("git tag --sort=-creatordate"),
        Command::Status => shell("git status --untracked-files --short"),
        Command::Clone(cmd) => shell(&format!("git clone {}", cmd.repo)),
        Command::Push(cmd) => shell(&format!("git add . && git commit -m '{}' && git push", cmd.message)),
        Command::AddTag(cmd) => {
            shell(&format!("git tag -a '{version}' -m '{version}' && git push origin {version}", version = cmd.version))
        }
        Command::DeleteTag(cmd) => {
            shell(&format!("git tag -d '{version}' && git push origin :refs/tags/{version}", version = cmd.version))
        }
        Command::Amend => shell("git add . && git commit --amend --no-edit && git push --force"),
    }
}
