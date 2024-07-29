use clap::Parser;
use mm_dev::shell;

/// Backup a project dir
#[derive(Parser)]
struct Cli {
    src: String,
    dest: String,
}

const EXCLUDE_PATH: &[&str] = &[
    ".idea/",
    "target/",
    "node_modules/",
    ".venv/",
    "__pycache__/",
    ".mypy_cache/",
    ".pytest_cache/",
    ".ruff_cache/",
    "**/dist/*.gz",
    "**/dist/*.whl",
    ".coverage",
];

fn main() {
    let cli = Cli::parse();
    let exclude = EXCLUDE_PATH.iter().map(|d| format!("--exclude={}", d)).collect::<Vec<String>>().join(" ");
    shell(&format!("rsync -azvhP {} {} {}", exclude, cli.src, cli.dest));
}
