use structopt::StructOpt;
use tide;
use tide::{Body, Response};

use crate::flags::Flags;
use crate::rules::RulesReader;

mod rules;
mod flags;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let _guard = wdt_logging::setup_logging();
    let flags = Flags::from_args();
    let reader = RulesReader::from(&flags);

    let rules = reader.read()?;

    let mut app = tide::new();
    app.at("/api/rules").get(move |_| {
        let body = rules.clone();
        async move {
            let mut res = Response::new(200);
            res.set_body(Body::from_json(&body)?);
            Ok(res)
        }
    });

    // app.at("/api/nodes/deployment").post(move |res| {
    // // TODO: allow pushing of results & logs
    // });

    app.listen("0.0.0.0:9000").await?;

    Ok(())
}

