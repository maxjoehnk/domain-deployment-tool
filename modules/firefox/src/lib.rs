use wdt_module_api::{Module, ValidationError};
use thiserror::Error;
use serde::{Serialize, Deserialize};

use crate::policies::*;
use std::fs::File;
use std::path::{PathBuf, Path};

mod policies;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FirefoxModule {
    extensions: Vec<String>,
    home: Option<String>,
    start_page: Option<StartPage>
}

impl Module for FirefoxModule {
    type Error = FirefoxModuleError;
    type State = FirefoxPolicies;

    fn apply(&self, previous_state: Option<Self::State>) -> Result<Self::State, Self::Error> {
        let installation_dir = FirefoxModule::find_installation_dir();
        if let Some(installation_dir) = installation_dir {

            if !Path::new(&installation_dir).exists() {
                return Err(FirefoxModuleError::UnknownInstallationDirectory);
            }
            let policies = self.persist_policies(&installation_dir)?;

            Ok(policies)
        }else {
            Err(FirefoxModuleError::UnknownInstallationDirectory)
        }
    }

    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        unimplemented!()
    }
}

impl FirefoxModule {
    fn persist_policies(&self, installation_dir: &str) -> Result<FirefoxPolicies, FirefoxModuleError> {
        let file_path: PathBuf = [installation_dir, "distribution", "policies.json"].iter().collect();
        let file = FirefoxModule::open_file(&file_path)?;
        wdt_logging::debug!("Writing policies to {:?}...", &file_path);
        self.write_policies(file)
    }

    fn open_file(file_path: &PathBuf) -> Result<File, FirefoxModuleError> {
        let distribution_dir = file_path.parent().unwrap();
        let file = if !file_path.exists() {
            if !distribution_dir.exists() {
                std::fs::create_dir_all(distribution_dir)?;
            }
            File::create(&file_path)?
        } else {
            File::open(&file_path)?
        };
        Ok(file)
    }

    fn write_policies(&self, file: File) -> Result<FirefoxPolicies, FirefoxModuleError> {
        let policies: FirefoxPolicies = self.clone().into();
        let policies_content = serde_json::json!({
            "policies": &policies
        });
        serde_json::to_writer(file, &policies_content)?;

        Ok(policies)
    }

    #[cfg(target_os = "windows")]
    fn find_installation_dir() -> Option<String> {
        Some("C:\\Program Files\\Mozilla Firefox".into())
    }

    #[cfg(not(target_os = "windows"))]
    fn find_installation_dir() -> Option<String> {
        None
    }

    fn get_home(&self) -> Option<HomepagePolicies> {
        if self.home.is_none() && self.start_page.is_none() {
            None
        }else {
            let mut policies = HomepagePolicies::default();
            if let Some(ref homepage) = self.home {
                policies.url = Some(homepage.clone());
            }
            if let Some(ref start_page) = self.start_page {
                policies.start_page = Some(start_page.clone());
            }
            Some(policies)
        }
    }
}

impl From<FirefoxModule> for FirefoxPolicies {
    fn from(module: FirefoxModule) -> Self {
        FirefoxPolicies {
            homepage: module.get_home(),
            extensions: ExtensionPolicies {
                install: module.extensions,
                locked: Default::default(),
                uninstall: Default::default(),
            },
        }
    }
}

#[derive(Error, Debug)]
pub enum FirefoxModuleError {
    #[error("Serialization of policies failed")]
    SerializationError(#[from] serde_json::Error),
    #[error("IO error {0}")]
    IOError(#[from] std::io::Error),
    #[error("Unable to find Firefox installation directory")]
    UnknownInstallationDirectory
}
