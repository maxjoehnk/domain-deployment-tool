use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "wdt-client")]
pub struct Flags {
    /// Run in user mode
    #[structopt(long)]
    pub user: bool,
    /// Install service
    #[structopt(long)]
    pub install: bool,
    /// Server from which to pull modules
    #[structopt(long)]
    pub server: Option<String>,
}
