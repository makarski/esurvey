extern crate rust_google_oauth2 as gauth;
use anyhow::anyhow;
use clap::{load_yaml, App};

use std::str::FromStr;

mod appsscript;
mod chart;
mod cmd;
mod config;
mod drive;
mod sheets;
mod survey;

fn main() -> anyhow::Result<()> {
    let args: clap::ArgMatches;
    let run: cmd::Cmd;

    {
        let yaml = load_yaml!("cli.yml");
        args = App::from(yaml).get_matches();
        let cmd_str = args
            .value_of("CMD")
            .ok_or_else(|| anyhow!("`CMD` argument is missing"))?;

        run = cmd::Cmd::from_str(cmd_str)?;
    }

    run.run(args)
}
