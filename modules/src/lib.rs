use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::{from_value, to_value, Value};

use wdt_logging::{info, open_scope, options};
use wdt_module_api::Module;
use wdt_module_firefox::FirefoxModule;
use wdt_module_printer::PrinterModule;
use wdt_module_software::SoftwareModule;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct StateManager {
    pub state: HashMap<String, Value>
}

impl StateManager {
    pub fn get<TState>(&self, key: &str) -> Option<TState>
        where TState: DeserializeOwned {
        if let Some(value) = self.state.get(key).cloned() {
            from_value(value).ok()
        } else {
            None
        }
    }

    pub fn set<TState>(&mut self, key: String, value: TState)
        where TState: Serialize {
        self.state.insert(key, to_value(value).unwrap());
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RuleConfiguration {
    pub name: String,
    #[serde(flatten)]
    pub module: Modules,
    #[serde(default)]
    pub groups: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "module", rename_all = "kebab-case")]
pub enum Modules {
    Firefox(FirefoxModule),
    Software(SoftwareModule),
    Printer(PrinterModule),
    FileAssociations,
}

impl Modules {
    fn name(&self) -> &'static str {
        use Modules::*;
        match self {
            Firefox(_) => "Firefox",
            Software(_) => "Software",
            Printer(_) => "Printer",
            FileAssociations => "FileAssociations"
        }
    }
}

impl RuleConfiguration {
    pub async fn apply(&self, store: &mut StateManager) -> anyhow::Result<()> {
        open_scope(options!("rule" => self.name.clone(), "module" => self.module.name()), || {
            let identifier = self.identifier();
            info!("Applying rule {}...", identifier);
            match &self.module {
                Modules::Firefox(module) => {
                    let next = module.apply(store.get(&identifier))?;
                    store.set(identifier, next);
                },
                Modules::Software(module) => {
                    let next = module.apply(store.get(&identifier))?;
                    store.set(identifier, next);
                },
                Modules::Printer(module) => {
                    let next = module.apply(store.get(&identifier))?;
                    store.set(identifier, next);
                },
                _ => {}
            }
            info!("Done.");
            Ok(())
        })
    }

    fn identifier(&self) -> String {
        format!("{}:{}", self.module.name(), &self.name)
    }
}
