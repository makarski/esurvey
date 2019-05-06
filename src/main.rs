#![allow(dead_code)]

extern crate rust_google_oauth2 as gauth;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::env;
use std::env::args;
use std::error::Error as std_err;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{stdin, Error as io_err, ErrorKind as io_err_kind};
use std::path::PathBuf;
use std::str::FromStr;

mod sheets;

const SELF_ASSESSMENT_STR: &str = "self-assessment";
const TEAM_FEEDBACK_STR: &str = "team-feedback";

enum AssessmentKind {
    TeamFeedback,
    SelfAssessment,
}

impl AssessmentKind {
    fn config(&self) -> Vec<(Skill, u32)> {
        match self {
            AssessmentKind::SelfAssessment => vec![
                (Skill::Adaptability, 2),
                (Skill::Attitude, 2),
                (Skill::Communication, 3),
                (Skill::CrossFunctionalKnowledge, 2),
                (Skill::Dependability, 3),
                (Skill::Initiative, 2),
                (Skill::Leadership, 3),
                (Skill::Organization, 3),
                (Skill::Responsibility, 2),
                (Skill::SelfImprovement, 2),
                (Skill::Teamwork, 3),
                (Skill::TechExpertise, 2),
            ],
            AssessmentKind::TeamFeedback => vec![
                (Skill::Adaptability, 2),
                (Skill::Attitude, 2),
                (Skill::Communication, 3),
                (Skill::CrossFunctionalKnowledge, 2),
                (Skill::Dependability, 3),
                (Skill::Initiative, 2),
                (Skill::Leadership, 4),
                (Skill::Organization, 3),
                (Skill::Responsibility, 2),
                (Skill::SelfImprovement, 2),
                (Skill::Teamwork, 2),
                (Skill::TechExpertise, 2),
            ],
        }
    }
}

impl FromStr for AssessmentKind {
    type Err = io_err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            TEAM_FEEDBACK_STR => Ok(AssessmentKind::TeamFeedback),
            SELF_ASSESSMENT_STR => Ok(AssessmentKind::SelfAssessment),
            _ => Err(io_err::new(
                io_err_kind::InvalidInput,
                "AssessemntKind parse error. valid types: `team-feedback`, `self-assessment`",
            )),
        }
    }
}

impl Display for AssessmentKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AssessmentKind::SelfAssessment => write!(f, "self-assessment"),
            AssessmentKind::TeamFeedback => write!(f, "team-feedback"),
        }
    }
}

enum Skill {
    Adaptability,
    Attitude,
    Communication,
    CrossFunctionalKnowledge,
    Dependability,
    Initiative,
    Leadership,
    Organization,
    Responsibility,
    SelfImprovement,
    Teamwork,
    TechExpertise,
}

impl Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Skill::Adaptability => write!(f, "Adaptability"),
            Skill::Attitude => write!(f, "Attitude"),
            Skill::Communication => write!(f, "Communication"),
            Skill::CrossFunctionalKnowledge => write!(f, "Cross-functional Knowledge"),
            Skill::Dependability => write!(f, "Dependability"),
            Skill::Initiative => write!(f, "Initiative"),
            Skill::Leadership => write!(f, "Leadership"),
            Skill::Organization => write!(f, "Organization"),
            Skill::Responsibility => write!(f, "Responsibility"),
            Skill::SelfImprovement => write!(f, "Self-Improvement"),
            Skill::Teamwork => write!(f, "Teamwork"),
            Skill::TechExpertise => write!(f, "Tech. Expertise"),
        }
    }
}

struct EmployeeSkill {
    name: Skill,
    questions: u32,
    grades: Vec<u32>,
}

impl EmployeeSkill {
    fn new(n: Skill, q: u32) -> EmployeeSkill {
        EmployeeSkill {
            name: n,
            questions: q,
            grades: Vec::with_capacity(q as usize),
        }
    }

    fn add_grade(&mut self, v: u32) {
        self.grades.push(v)
    }

    fn avg(&self) -> f32 {
        self.grades.iter().sum::<u32>() as f32 / self.grades.len() as f32
    }
}

impl Display for EmployeeSkill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.avg())
    }
}

fn handle_auth(consent_uri: String) -> Result<String, Box<dyn std::error::Error>> {
    println!("> open the link in browser\n\n{}\n", consent_uri);
    println!("> enter the auth. code\n");

    let mut auth_code = String::new();
    stdin().read_line(&mut auth_code)?;

    Ok(auth_code)
}

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

    create_summary_sheet(&client, &token.access_token, &spreadsheet_id);

    for (sheet_index, sheet) in s.sheets.into_iter().enumerate() {
        println!("> reading data from sheet tab: {}", sheet.properties.title);

        let sheet_title = sheet.properties.title;

        let spreadsheet = client
            .get_batch_values(
                &token.access_token,
                &spreadsheet_id,
                vec![format!("{}", &sheet_title)],
            )
            .expect("failed to retrieve speadsheet data");

        for sheet_value in spreadsheet.value_ranges.into_iter() {
            println!("scanning spreadsheet range: {}", sheet_value.range);

            let feedback_kind = AssessmentKind::from_str(&sheet_title)
                .expect("failed to detect feedback kind based on sheet name");
            let cfg_questions = feedback_kind.config().into_iter();

            let mut questions: Vec<EmployeeSkill> = Vec::with_capacity(cfg_questions.len());
            for (skill, question_count) in cfg_questions {
                questions.push(EmployeeSkill::new(skill, question_count));
            }

            let mut answers = sheet_value.values.into_iter().skip(2);

            for q in &mut questions {
                let mut counter: u32 = 0;

                loop {
                    let per_category = &answers.next().unwrap();
                    let mut per_category = per_category.into_iter();

                    println!(">> scanning '{}: {}'", q.name, per_category.next().unwrap());

                    for grade_str in per_category {
                        let grade: u32 = grade_str.parse().expect("could not parse grade");
                        q.add_grade(grade);
                    }

                    counter = counter + 1;
                    if counter == q.questions {
                        break;
                    }
                }
            }

            save_to_drive(
                &client,
                &token.access_token,
                &spreadsheet_id,
                &questions,
                &feedback_kind,
                sheet_index,
            );
        }
    }
}

fn create_summary_sheet(client: &sheets::Client, token: &str, spreadsheet_id: &str) {
    let batch_update = sheets::spreadsheets_batch_update::SpreadsheetBatchUpdate {
        requests: vec![sheets::spreadsheets_batch_update::Request {
            add_sheet: Some(sheets::spreadsheets_batch_update::AddSheetRequest {
                properties: sheets::SheetProperties {
                    sheet_id: None,
                    title: "Chart and Summary".to_owned(),
                    index: None,
                    sheet_type: None,
                    grid_properties: None,
                },
            }),
        }],
        include_spreadsheet_in_response: false,
        response_ranges: Vec::new(),
        response_include_grid_data: false,
    };

    client
        .batch_update_spreadsheet(token, spreadsheet_id, &batch_update)
        .expect("could not create a summary spreadsheet tab");
}

fn save_to_drive(
    client: &sheets::Client,
    token: &str,
    spreadsheet_id: &str,
    questions: &Vec<EmployeeSkill>,
    feedback_kind: &AssessmentKind,
    sheet_index: usize,
) {
    let mut spreadsheet_values = sheets::spreadsheets_values::SpreadsheetValueRange {
        range: "Chart and Summary".to_owned(),
        major_dimension: "COLUMNS".to_owned(),
        values: Vec::with_capacity(questions.len() as usize + 1),
    };

    let generate_col_value = |sheet_index: usize, vals: Vec<String>| -> Vec<String> {
        if sheet_index == 0 {
            return vals;
        }
        return vals[1..].to_vec();
    };

    spreadsheet_values.add_value(generate_col_value(
        sheet_index,
        vec!["".to_owned(), feedback_kind.to_string()],
    ));

    for question in questions {
        spreadsheet_values.add_value(generate_col_value(
            sheet_index,
            vec![question.name.to_string(), question.avg().to_string()],
        ))
    }

    client
        .append_values(
            token,
            spreadsheet_id.to_owned(),
            "Chart and Summary".to_owned(),
            &spreadsheet_values,
        )
        .expect("could not update google sheet values");
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
