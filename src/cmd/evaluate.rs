use std::env::args;
use std::error::Error;
use std::io::{Error as io_err, ErrorKind as io_err_kind};

use crate::chart;
use crate::drive;
use crate::sheets;

const SUMMARY_SHEET_NAME: &str = "Chart and Summary";
const CHART_NAME: &str = "Chart Results";

pub struct Evaluator {
    _auth_client: gauth::Auth,
}

impl Evaluator {
    pub fn new(auth_client: gauth::Auth) -> Self {
        Evaluator {
            _auth_client: auth_client,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let token = self._auth_client.access_token(super::handle_auth)?;

        let flags = parse_flags()?;
        println!("entered id: {}", flags.spreadsheet_id);
        println!("entered templates file: {}", flags.config_file);

        let client = sheets::Client::new();
        let spreadsheet = client.get_spreadsheet(&token.access_token, &flags.spreadsheet_id)?;
        let spreadsheet_client = drive::SpreadsheetClient::new(&client, &token.access_token);

        let summary = spreadsheet_client.build_summary(
            spreadsheet.sheets,
            &flags.spreadsheet_id,
            &flags.config_file,
        )?;

        let summary_sheet_id =
            spreadsheet_client.add_summary_sheet(SUMMARY_SHEET_NAME, &flags.spreadsheet_id)?;

        spreadsheet_client.save_summary(SUMMARY_SHEET_NAME, &flags.spreadsheet_id, summary)?;

        chart::add_summary_chart(
            &client,
            &token.access_token,
            &flags.spreadsheet_id,
            summary_sheet_id,
            String::from(CHART_NAME),
        )?;

        Ok(())
    }
}

struct Flags {
    spreadsheet_id: String,
    config_file: String,
}

fn parse_flags() -> Result<Flags, Box<dyn Error>> {
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
