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
use std::str::FromStr;

mod drive;
mod sheets;
use sheets::spreadsheets_values::{MajorDimension, SpreadsheetValueRange};

mod config;
use config::{AssessmentKind, ResponseKind};

mod skills;
use skills::EmployeeSkills;

mod chart;

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

    let s = client
        .get_spreadsheet(&token.access_token, &flags.spreadsheet_id)
        .expect("failed to retrieve spreadsheet info");

    let spreadsheet_client = drive::SpreadsheetClient::new(&client, &token.access_token);
    let summary_sheet_id = spreadsheet_client
        .add_summary_sheet(&flags.spreadsheet_id)
        .expect("failed to create summary sheet");

    process_sheet_vals(
        &spreadsheet_client,
        &client,
        s.sheets,
        &flags.spreadsheet_id,
        &flags.config_file,
        &token.access_token,
    );

    chart::add_summary_chart(
        &client,
        &token.access_token,
        &flags.spreadsheet_id,
        summary_sheet_id,
    );
}

fn process_sheet_vals(
    sheets_client: &drive::SpreadsheetClient,
    client: &sheets::Client,
    sheet_data: Vec<sheets::spreadsheets::Sheet>,
    spreadsheet_id: &str,
    config_file: &str,
    access_token: &str,
) {
    let mut texts = Vec::with_capacity(2);

    for (sheet_index, sheet) in sheet_data.into_iter().enumerate() {
        println!("> reading data from sheet tab: {}", sheet.properties.title);

        let sheet_title = sheet.properties.title;

        let sheet_vals = client
            .get_batch_values(access_token, spreadsheet_id, vec![sheet_title.clone()])
            .expect("failed to retrieve speadsheet data");

        for val_range in sheet_vals.value_ranges.into_iter() {
            let (feedback_kind, graded_skills, statement_feedback) =
                collect_data(config_file, &sheet_title, val_range);

            println!(">>> uploading grades to drive!");

            drive::save_to_drive(
                client,
                access_token,
                spreadsheet_id,
                &graded_skills.skills,
                &feedback_kind,
                MajorDimension::Columns,
                sheet_index,
            );

            println!(">>> collecting textual feedback!");

            texts.push((feedback_kind, statement_feedback));
        }
    }

    println!(">>> uploading textual feedback!");

    sheets_client
        .save_text(spreadsheet_id, texts)
        .expect("failed to upload text feedback");
}

fn collect_data(
    cfg_filename: &str,
    sheet_title: &str,
    val_range: SpreadsheetValueRange,
) -> (AssessmentKind, EmployeeSkills, EmployeeSkills) {
    println!("scanning spreadsheet range: {}", val_range.range);

    let feedback_kind = AssessmentKind::from_str(sheet_title)
        .expect("failed to detect feedback kind based on sheet name");

    let mut grade_skills =
        EmployeeSkills::new(&feedback_kind.config(cfg_filename, ResponseKind::Grade));
    let mut text_skills =
        EmployeeSkills::new(&feedback_kind.config(cfg_filename, ResponseKind::Text));

    let offset = grade_skills.scan(2, &val_range.values);
    text_skills.scan(offset + 2, &val_range.values);

    (feedback_kind, grade_skills, text_skills)
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
