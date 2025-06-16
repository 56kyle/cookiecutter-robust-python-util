use std::path::PathBuf;



pub fn setup_git_remote(
    path: PathBuf,
    github_user: String,
    repo_name: String,
) -> eyre::Result<()> {
    println!("Repo not found at {:?}", path);
    Ok(())
}


