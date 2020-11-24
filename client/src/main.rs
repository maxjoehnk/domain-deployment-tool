mod rule_loader;
mod flags;
#[cfg(target_os = "windows")]
mod windows;
mod state;

#[cfg(target_os = "windows")]
use windows::install_service;
use crate::flags::Flags;
use structopt::StructOpt;
use crate::rule_loader::RuleLoader;
use wdt_logging::setup_logging;
use wdt_modules::StateManager;
use state::StateStore;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let _scope_guard = setup_logging();

    let flags = Flags::from_args();
    if flags.install {
        install_service(flags)?;
        return Ok(())
    }
    wdt_logging::info!("Starting...");
    let mut store = StateManager::default();
    store.load(&flags)?;
    let loader = RuleLoader::from(&flags);
    let rules = loader.load().await?;

    for rule in rules {
        if let Err(err) = rule.apply(&mut store).await {
            wdt_logging::error!("Couldn't apply rule '{}': {}", &rule.name, err);
        }
    }
    store.save(&flags)?;

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn install_service(_flags: Flags) -> anyhow::Result<()> {}

