extern crate rust_google_oauth2 as gauth;
#[macro_use]
extern crate serde_derive;
extern crate csv;
extern crate serde;
extern crate serde_json;

use std::env::args;
use std::str::FromStr;

mod appsscript;
mod chart;
mod cmd;
mod config;
mod drive;
mod sheets;
mod survey;

fn main() {
    cmd::Cmd::from_str(&args().nth(1).expect("failed to retrieve command name"))
        .map(|c| c.run().expect("failed to run the application"))
        .expect("failed to execute command");
}
