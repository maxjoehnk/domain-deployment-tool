use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "wdt-server")]
pub struct Flags {
    /// Path to rules
    #[structopt(short, long)]
    pub rules: String
}
