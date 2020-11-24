use crate::flags::Flags;
use wdt_modules::RuleConfiguration;
use std::fs::File;

pub struct RulesReader(String);

impl RulesReader {
    pub fn read(&self) -> anyhow::Result<Vec<RuleConfiguration>> {
        let directories = std::fs::read_dir(&self.0)?;
        let mut modules = Vec::new();
        for dir in directories {
            let dir = dir?;
            if dir.file_type()?.is_file() {
                let mut file = File::open(dir.path())?;
                let mut module = serde_yaml::from_reader(&mut file)?;
                modules.append(&mut module);
            }
        }

        Ok(modules)
    }
}

impl From<&Flags> for RulesReader {
    fn from(flags: &Flags) -> Self {
        RulesReader(flags.rules.to_string())
    }
}
