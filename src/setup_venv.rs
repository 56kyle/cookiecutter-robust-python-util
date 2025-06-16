use std::path::PathBuf;
use std::process::Command;

use uv_cli::VenvArgs;

fn setup_venv(path: PathBuf, python_version: String) -> Result<(), Box<dyn std::error::Error>> {
    let command = Command::new("uv")
        .arg("venv")
        .arg(".venv")
        .output()
        .expect("Failed to execute command");

    match command.status.code() {
        Some(0) => {}
        _ => {}
    }
    Ok(())
}

fn _create_venv(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    uv_virtualenv::create_venv(&path).expect("Failed to create virtual environment");

    Ok(())
}
