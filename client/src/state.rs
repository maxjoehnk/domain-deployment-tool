use crate::flags::Flags;
use serde::{Serialize};
use serde_json::{from_reader, to_writer};
use wdt_modules::StateManager;
use std::fs::File;
use std::path::Path;
use wdt_logging::debug;

pub trait StateStore : Serialize {
    fn load(&mut self, flags: &Flags) -> anyhow::Result<()>;

    fn save(&self, flags: &Flags) -> anyhow::Result<()>;
}

impl StateStore for StateManager {
    fn load(&mut self, flags: &Flags) -> anyhow::Result<()> {
        let path = get_state_path(flags);
        let path = Path::new(&path);
        if !path.exists() {
            debug!("No state file exists at {:?}", &path);
            Ok(())
        }else {
            debug!("Loading state file {:?}", &path);
            let mut file = File::open(path)?;
            self.state = from_reader(&mut file)?;

            Ok(())
        }
    }

    fn save(&self, flags: &Flags) -> anyhow::Result<()> {
        let path = get_state_path(flags);
        debug!("Storing state in {}", &path);
        let mut file = File::create(path)?;
        to_writer(&mut file, &self)?;

        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn get_state_path(flags: &Flags) -> String {
    if flags.user {
        unimplemented!()
    }else {
        "C:\\.wdt-state.json".into()
    }
}

#[cfg(not(target_os = "windows"))]
fn get_state_path(flags: &Flags) -> String {
    if flags.user {
        unimplemented!()
    }else {
        unimplemented!()
    }
}
