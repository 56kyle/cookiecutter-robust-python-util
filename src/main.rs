//! Utility commands used by cookiecutter-robust-python.
use clap::{Parser, Subcommand};
use cookiecutter_robust_python_util::setup_git_local::setup_git_local;
use cookiecutter_robust_python_util::setup_git_remote::setup_git_remote;
use cookiecutter_robust_python_util::setup_venv::setup_venv;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Sets up a local git repo and branches
    SetupGitLocal { repo_folder: PathBuf },
    /// Sets up a remote repo and syncs it with local
    SetupGitRemote {
        path: PathBuf,
        github_user: String,
        repo_name: String,
    },
    /// Sets up a uv virtual environment
    SetupVenv {
        path: PathBuf,
        python_version: String,
    },
}


fn main() -> eyre::Result<()> {
    let args: Args = Args::parse();

    let result = match args.cmd {
        Commands::SetupGitLocal { repo_folder } => setup_git_local(repo_folder),
        Commands::SetupGitRemote {
            path,
            github_user,
            repo_name,
        } => setup_git_remote(path, github_user, repo_name),
        Commands::SetupVenv {
            path,
            python_version,
        } => setup_venv(path, python_version),
    };
    result
}
