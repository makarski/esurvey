use crate::config::{AssessmentKind, ResponseKind};
use crate::sheets;
use crate::skills::{EmployeeSkill, EmployeeSkills};

use std::collections::HashMap;
use std::error::Error as std_err;
use std::ops::Deref;
use std::str::FromStr;

use sheets::spreadsheets::{Sheet, SheetProperties};
use sheets::spreadsheets_batch_update::{AddSheetRequest, Request, SpreadsheetBatchUpdate};
use sheets::spreadsheets_values::{MajorDimension, SpreadsheetValueRange};

pub struct SpreadsheetClient<'a> {
    sheets_client: &'a sheets::Client,
    access_token: String,
}

impl<'a> SpreadsheetClient<'a> {
    pub fn new(sheets_client: &'a sheets::Client, access_token: &str) -> Self {
        SpreadsheetClient {
            sheets_client: sheets_client,
            access_token: access_token.to_owned(),
        }
    }

    // todo: add error returns
    pub fn build_summary(
        &self,
        sheet_items: Vec<Sheet>,
        spreadsheet_id: &String,
        config_file: &str,
    ) {
        let mut texts = Vec::with_capacity(2);

        for (sheet_index, sheet) in sheet_items.into_iter().enumerate() {
            println!("> reading data from sheet tab: {}", sheet.properties.title);

            let sheet_title = sheet.properties.title;

            let sheet_vals = self
                .sheets_client
                .get_batch_values(
                    &self.access_token,
                    spreadsheet_id,
                    vec![sheet_title.clone()],
                )
                .expect("failed to retrieve speadsheet data");

            for val_range in sheet_vals.value_ranges.into_iter() {
                let (feedback_kind, graded_skills, statement_feedback) =
                    self.collect_data(config_file, &sheet_title, val_range);

                println!(">>> uploading grades to drive!");

                self.save_grades(
                    spreadsheet_id,
                    &graded_skills.skills,
                    &feedback_kind,
                    MajorDimension::Columns,
                    sheet_index,
                )
                .expect("failed to upload graded feedback");

                println!(">>> collecting textual feedback!");

                texts.push((feedback_kind, statement_feedback));
            }
        }

        println!(">>> uploading textual feedback!");

        self.save_text(spreadsheet_id, texts)
            .expect("failed to upload text feedback");
    }

    pub fn add_summary_sheet(&self, spreadsheet_id: &str) -> Result<u64, Box<dyn std_err>> {
        let batch_update = SpreadsheetBatchUpdate {
            requests: vec![Request {
                add_sheet: Some(AddSheetRequest {
                    properties: SheetProperties {
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

        let sheet_id = self
            .sheets_client
            .batch_update_spreadsheet(self.access_token.as_str(), spreadsheet_id, &batch_update)
            .map(|response_body| {
                // todo: find a better way to deal with the borrow checker

                let mut sheet_id: u64 = 0;
                for reply in response_body.replies.into_iter().take(1) {
                    sheet_id = reply.add_sheet.unwrap().properties.sheet_id.unwrap();
                }

                sheet_id
            })?;

        Ok(sheet_id)
    }

    fn collect_data(
        &self,
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

    fn save_text(
        &self,
        spreadsheet_id: &str,
        feedbacks: Vec<(AssessmentKind, EmployeeSkills)>,
    ) -> Result<(), Box<dyn std_err>> {
        let mut spreadsheet_values = SpreadsheetValueRange {
            range: "Chart and Summary".to_owned(),
            major_dimension: MajorDimension::Rows,
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

        self.sheets_client.append_values(
            &self.access_token,
            spreadsheet_id.to_owned(),
            "Chart and Summary".to_owned(),
            &spreadsheet_values,
        )?;

        Ok(())
    }

    fn save_grades(
        &self,
        spreadsheet_id: &str,
        questions: &Vec<EmployeeSkill>,
        feedback_kind: &AssessmentKind,
        major_dimension: MajorDimension,
        sheet_index: usize,
    ) -> Result<(), Box<dyn std_err>> {
        let mut spreadsheet_values = SpreadsheetValueRange {
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

        self.sheets_client.append_values(
            &self.access_token,
            spreadsheet_id.to_owned(),
            "Chart and Summary".to_owned(),
            &spreadsheet_values,
        )?;

        Ok(())
    }
}
