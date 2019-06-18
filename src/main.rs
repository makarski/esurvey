extern crate rust_google_oauth2 as gauth;
#[macro_use]
extern crate serde_derive;
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
use config::{AssessmentKind, EmployeeSkill};

fn main() {
    let spreadsheet_id = parse_flags().expect("could not parse input flags");
    println!("entered id: {}", spreadsheet_id);

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
        .get_spreadsheet(&token.access_token, &spreadsheet_id)
        .expect("failed to retrieve spreadsheet info");

    let summary_sheet_id =
        drive::create_summary_sheet(&client, &token.access_token, &spreadsheet_id);

    process_sheet_vals(&client, s.sheets, &spreadsheet_id, &token.access_token);

    drive::add_summary_chart(
        &client,
        &token.access_token,
        &spreadsheet_id,
        summary_sheet_id,
    );
}

fn process_sheet_vals(
    client: &sheets::Client,
    sheet_data: Vec<sheets::spreadsheets::Sheet>,
    spreadsheet_id: &str,
    access_token: &str,
) {
    for (sheet_index, sheet) in sheet_data.into_iter().enumerate() {
        println!("> reading data from sheet tab: {}", sheet.properties.title);

        let sheet_title = sheet.properties.title;

        let sheet_vals = client
            .get_batch_values(access_token, spreadsheet_id, vec![sheet_title.clone()])
            .expect("failed to retrieve speadsheet data");

        for val_range in sheet_vals.value_ranges.into_iter() {
            let (feedback_kind, questions) = collect_data(&sheet_title, val_range);

            println!(">>> uploading to drive!");

            drive::save_to_drive(
                client,
                access_token,
                spreadsheet_id,
                &questions,
                &feedback_kind,
                MajorDimension::Columns,
                sheet_index,
            );
        }
    }
}

// fn collect_grades() {}
// fn collect_texts() {}

fn collect_data(sheet_title: &str, val_range: SpreadsheetValueRange) -> (AssessmentKind, Vec<EmployeeSkill>) {
    println!("scanning spreadsheet range: {}", val_range.range);

    let feedback_kind = AssessmentKind::from_str(sheet_title)
        .expect("failed to detect feedback kind based on sheet name");
    let cfg_questions = feedback_kind.config().into_iter();

    let mut questions: Vec<EmployeeSkill> = Vec::with_capacity(cfg_questions.len());
    for (skill, question_count) in cfg_questions {
        questions.push(EmployeeSkill::new(skill, question_count));
    }

    let mut answers = val_range.values.into_iter().skip(2);

    for q in &mut questions {
        let mut counter: u32 = 0;

        loop {
            let per_category = &answers.next().unwrap();
            let mut per_category = per_category.into_iter();

            let question_stmt = per_category.next().unwrap();
            println!(">> scanning '{}: {}'", q.name, &question_stmt);

            for grade_str in per_category {
                q.add_response(grade_str);
            }

            counter = counter + 1;
            if counter == q.questions {
                break;
            }
        }
    }

    (feedback_kind, questions)
}

fn parse_flags() -> Result<String, Box<dyn std_err>> {
    let mut spreadsheet_id = String::new();

    for arg in args().collect::<Vec<String>>() {
        if arg.contains("-id=") {
            spreadsheet_id = arg.trim_start_matches("-id=").parse()?;
        }
    }

    if spreadsheet_id.is_empty() {
        return Err(Box::new(io_err::new(
            io_err_kind::InvalidInput,
            "spreadsheet_id is not provided",
        )));
    }

    Ok(spreadsheet_id)
}

fn handle_auth(consent_uri: String) -> Result<String, Box<dyn std::error::Error>> {
    println!("> open the link in browser\n\n{}\n", consent_uri);
    println!("> enter the auth. code\n");

    let mut auth_code = String::new();
    stdin().read_line(&mut auth_code)?;

    Ok(auth_code)
}
