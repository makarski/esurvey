use crate::config::AssessmentKind;
use crate::skills::{EmployeeSkill, EmployeeSkills};

use crate::sheets;
use sheets::spreadsheets_values;

pub fn save_to_drive(
    client: &sheets::Client,
    token: &str,
    spreadsheet_id: &str,
    questions: &Vec<EmployeeSkill>,
    feedback_kind: &AssessmentKind,
    major_dimension: spreadsheets_values::MajorDimension,
    sheet_index: usize,
) {
    let mut spreadsheet_values = sheets::spreadsheets_values::SpreadsheetValueRange {
        range: "Chart and Summary".to_owned(),
        major_dimension: major_dimension,
        values: Vec::with_capacity(questions.len() as usize + 1),
    };

    let generate_col_value = |sheet_index: usize, vals: Vec<String>| -> Vec<String> {
        match sheet_index {
            0 => vals,
            _ => vals[1..].to_vec(),
        }
    };

    spreadsheet_values.add_value(generate_col_value(
        sheet_index,
        vec!["".to_owned(), feedback_kind.to_string()],
    ));

    for question in questions {
        let response_cell: String = match question.name.is_graded() {
            true => question.avg().to_string(),
            false => question.txt(),
        };

        spreadsheet_values.add_value(generate_col_value(
            sheet_index,
            vec![question.name.to_string(), response_cell],
        ));
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

use std::collections::HashMap;
use std::ops::Deref;

pub fn save_text_drive(
    client: &sheets::Client,
    token: &str,
    spreadsheet_id: &str,
    feedbacks: Vec<(AssessmentKind, EmployeeSkills)>,
) {
    let mut spreadsheet_values = sheets::spreadsheets_values::SpreadsheetValueRange {
        range: "Chart and Summary".to_owned(),
        major_dimension: spreadsheets_values::MajorDimension::Rows,
        values: Vec::new(),
    };

    let mut aggregated: HashMap<String, Vec<String>> = HashMap::new();
    let mut aggreated_kinds: Vec<String> = Vec::with_capacity(feedbacks.len() + 1 as usize);

    aggreated_kinds.push("Skill / Audience".to_owned());

    for (fdb_kind, stmt_feedbacks) in feedbacks {
        aggreated_kinds.push(fdb_kind.to_string());

        for stmt_feedback in &stmt_feedbacks.skills {
            aggregated
                .entry(stmt_feedback.name.to_string())
                .and_modify(|e| e.push(stmt_feedback.txt()))
                .or_insert(vec![stmt_feedback.name.to_string(), stmt_feedback.txt()]);
        }
    }

    spreadsheet_values.add_value(aggreated_kinds);
    for item in aggregated.values() {
        spreadsheet_values.add_value(item.deref().to_vec());
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

pub fn create_summary_sheet(client: &sheets::Client, token: &str, spreadsheet_id: &str) -> u64 {
    let batch_update = sheets::spreadsheets_batch_update::SpreadsheetBatchUpdate {
        requests: vec![sheets::spreadsheets_batch_update::Request {
            add_sheet: Some(sheets::spreadsheets_batch_update::AddSheetRequest {
                properties: sheets::spreadsheets::SheetProperties {
                    title: "Chart and Summary".to_owned(),
                    ..Default::default()
                },
            }),
            add_chart: None,
        }],
        include_spreadsheet_in_response: true,
        response_ranges: Vec::new(),
        response_include_grid_data: false,
    };

    let u = client
        .batch_update_spreadsheet(token, spreadsheet_id, &batch_update)
        .map(|response_body| {
            // todo: find a better way to deal with the borrow checker

            let mut sheet_id: u64 = 0;
            for reply in response_body.replies.into_iter().take(1) {
                sheet_id = reply.add_sheet.unwrap().properties.sheet_id.unwrap();
            }

            sheet_id
        });

    u.expect("could not retrieve sheet id")
}
