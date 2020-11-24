use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FirefoxPolicies {
    pub extensions: ExtensionPolicies,
    pub homepage: Option<HomepagePolicies>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExtensionPolicies {
    pub install: Vec<String>,
    pub uninstall: Vec<String>,
    pub locked: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct HomepagePolicies {
    #[serde(rename = "URL")]
    pub url: Option<String>,
    pub locked: Option<bool>,
    pub additional: Vec<String>,
    pub start_page: Option<StartPage>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StartPage {
    None,
    Homepage,
    PreviousSession,
    HomepageLocked
}
