use std::path::PathBuf;
use std::process::Command;

use uv_cli::VenvArgs;

pub fn setup_venv(path: PathBuf, python_version: String) -> Result<(), Box<dyn std::error::Error>> {
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
        }
        _ => {
            panic!(stringify!(command.stderr))
        }
    }
    Ok(())
}

