use crate::flags::Flags;
use wdt_modules::RuleConfiguration;

pub struct RuleLoader(String);

impl RuleLoader {
    pub async fn load(&self) -> anyhow::Result<Vec<RuleConfiguration>> {
        let url = format!("{}/api/rules", self.0);
        wdt_logging::debug!("Loading rules from {}", &url);
        let mut res = surf::get(&url).await.unwrap();
        let modules = res.body_json().await.unwrap();

        Ok(modules)
    }
}

impl From<&Flags> for RuleLoader {
    fn from(flags: &Flags) -> Self {
        let server = flags.server.clone().unwrap_or_else(|| String::from("http://localhost:9000"));
        RuleLoader(server)
    }
}
