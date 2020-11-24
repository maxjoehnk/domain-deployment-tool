use std::process::Command;
use crate::SoftwareModuleError;

pub fn install_msi(msi_path: &str) -> Result<(), SoftwareModuleError> {
    Command::new("msiexec")
        .args(&["/package", msi_path, "/quiet", "/norestart"])
        .output()
        .unwrap();

    Ok(())
}
