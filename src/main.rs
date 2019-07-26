extern crate rust_google_oauth2 as gauth;
#[macro_use]
extern crate serde_derive;
extern crate csv;
extern crate serde;
extern crate serde_json;

use std::env;
use std::env::args;
use std::error::Error as std_err;
use std::io::{stdin, Error as io_err, ErrorKind as io_err_kind};
use std::path::PathBuf;

mod chart;
mod config;
mod drive;
mod sheets;
mod skills;

fn main() {
    let flags = parse_flags().expect("could not parse input flags");
    println!("entered id: {}", flags.spreadsheet_id);
    println!("entered templates file: {}", flags.config_file);

    let crd_path = env::var("OAUTH_CFG_FILE").expect("failed to retrieve OAUTH_CFG_FILE from env");
    let auth_client = gauth::Auth::new(
        "probation-check",
        vec![
            "https://www.googleapis.com/auth/drive".to_owned(),
            "https://www.googleapis.com/auth/drive.readonly".to_owned(),
            "https://www.googleapis.com/auth/drive.file".to_owned(),
            "https://www.googleapis.com/auth/spreadsheets".to_owned(),
            "https://www.googleapis.com/auth/spreadsheets.readonly".to_owned(),
        ],
        PathBuf::from(crd_path),
    );

    let token = auth_client
        .access_token(handle_auth)
        .expect("failed to retrieve access token");

    let client = sheets::Client::new();

    let spreadsheet = client
        .get_spreadsheet(&token.access_token, &flags.spreadsheet_id)
        .expect("failed to retrieve spreadsheet info");

    let spreadsheet_client = drive::SpreadsheetClient::new(&client, &token.access_token);
    let summary_sheet_id = spreadsheet_client
        .add_summary_sheet(&flags.spreadsheet_id)
        .expect("failed to create summary sheet");

    spreadsheet_client.build_summary(
        spreadsheet.sheets,
        &flags.spreadsheet_id,
        &flags.config_file,
    );

    chart::add_summary_chart(
        &client,
        &token.access_token,
        &flags.spreadsheet_id,
        summary_sheet_id,
    );
}

struct Flags {
    spreadsheet_id: String,
    config_file: String,
}

fn parse_flags() -> Result<Flags, Box<dyn std_err>> {
    let mut flags = Flags {
        spreadsheet_id: String::new(),
        config_file: String::new(),
    };

    for arg in args().collect::<Vec<String>>() {
        if arg.contains("-id=") {
            flags.spreadsheet_id = arg.trim_start_matches("-id=").parse()?;
        }

        if arg.contains("-templates=") {
            flags.config_file = arg.trim_start_matches("-templates=").parse()?;
        }
    }

    for (flag_name, cfg_entry) in [
        ("-spreadsheet_id", &flags.spreadsheet_id),
        ("-templates", &flags.config_file),
    ]
    .iter()
    {
        if cfg_entry.is_empty() {
            return Err(Box::new(io_err::new(
                io_err_kind::InvalidInput,
                format!("missing required flag: {}", flag_name),
            )));
        }
    }

    Ok(flags)
}

fn handle_auth(consent_uri: String) -> Result<String, Box<dyn std::error::Error>> {
    println!("> open the link in browser\n\n{}\n", consent_uri);
    println!("> enter the auth. code\n");

    let mut auth_code = String::new();
    stdin().read_line(&mut auth_code)?;

    Ok(auth_code)
}
