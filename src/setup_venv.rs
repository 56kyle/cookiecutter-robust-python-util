use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn setup_venv(path: PathBuf, python_version: String) -> eyre::Result<()> {
    let venv_path = path.join(".venv");
    if venv_path.exists() {
        fs::remove_dir_all(&venv_path).expect("Failed to remove existing .venv");
    }

    let command = Command::new("uv")
        .arg("venv")
        .arg(".venv")
        .arg("--python")
        .arg(python_version)
        .output()
        .expect("Failed to execute command");

    match command.status.code() {
        Some(0) => {
            println!("Created venv at: {:?}", path);
            Ok(())
        }
        _ => {
            let msg = String::from_utf8(command.stderr).expect("Failed to parse stderr");
            Err(eyre::eyre!(msg))
        }
    }
}
