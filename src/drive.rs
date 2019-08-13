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

pub struct Summary {
    texts: Vec<(AssessmentKind, EmployeeSkills)>,
    grades: Vec<(AssessmentKind, EmployeeSkills)>,
}

impl<'a> SpreadsheetClient<'a> {
    pub fn new(sheets_client: &'a sheets::Client, access_token: &str) -> Self {
        SpreadsheetClient {
            sheets_client: sheets_client,
            access_token: access_token.to_owned(),
        }
    }

    pub fn build_summary(
        &self,
        sheet_items: Vec<Sheet>,
        spreadsheet_id: &String,
        config_file: &str,
    ) -> Result<Summary, Box<dyn std_err>> {
        let mut texts: Vec<(AssessmentKind, EmployeeSkills)> = Vec::with_capacity(2);
        let mut grades: Vec<(AssessmentKind, EmployeeSkills)> = Vec::new();

        for sheet in sheet_items.into_iter() {
            println!("> reading data from sheet tab: {}", sheet.properties.title);

            let sheet_title = sheet.properties.title;

            let sheet_vals = self.sheets_client.get_batch_values(
                &self.access_token,
                spreadsheet_id,
                vec![sheet_title.clone()],
            )?;

            for val_range in sheet_vals.value_ranges.into_iter() {
                let (feedback_kind, graded_skills, statement_feedback) =
                    self.collect_data(config_file, &sheet_title, val_range)?;

                println!(">>> collecting grades!");
                grades.push((feedback_kind.clone(), graded_skills));

                println!(">>> collecting textual feedback!");
                texts.push((feedback_kind.clone(), statement_feedback));
            }
        }

        Ok(Summary {
            texts: texts,
            grades: grades,
        })
    }

    pub fn save_summary(
        &self,
        range: &str,
        spreadsheet_id: &str,
        summary: Summary,
    ) -> Result<(), Box<dyn std_err>> {
        self.save_grades(range, spreadsheet_id, summary.grades)?;
        self.save_text(range, spreadsheet_id, summary.texts)?;
        Ok(())
    }

    pub fn add_summary_sheet(
        &self,
        title: &str,
        spreadsheet_id: &str,
    ) -> Result<u64, Box<dyn std_err>> {
        let batch_update = SpreadsheetBatchUpdate {
            requests: vec![Request {
                add_sheet: Some(AddSheetRequest {
                    properties: SheetProperties {
                        title: title.to_owned(),
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
                for reply in response_body.replies.into_iter().take(1) {
                    if let Some(sheet) = reply.add_sheet {
                        if let Some(sheet_id) = sheet.properties.sheet_id {
                            return Ok(sheet_id);
                        }
                    }
                }

                return Err(Box::from(format!(
                    "error retrieving sheet id from the response. spreadsheet_id: {}",
                    spreadsheet_id
                )));
            })?;

        sheet_id
    }

    fn collect_data(
        &self,
        cfg_filename: &str,
        sheet_title: &str,
        val_range: SpreadsheetValueRange,
    ) -> Result<(AssessmentKind, EmployeeSkills, EmployeeSkills), Box<dyn std_err>> {
        println!("scanning spreadsheet range: {}", val_range.range);

        let feedback_kind = AssessmentKind::from_str(sheet_title)?;

        let mut grade_skills =
            EmployeeSkills::new(&feedback_kind.config(cfg_filename, ResponseKind::Grade)?)?;

        let mut text_skills =
            EmployeeSkills::new(&feedback_kind.config(cfg_filename, ResponseKind::Text)?)?;

        let offset = grade_skills.scan(2, &val_range.values)?;
        text_skills.scan(offset + 2, &val_range.values)?;

        Ok((feedback_kind, grade_skills, text_skills))
    }

    fn save_grades(
        &self,
        range: &str,
        spreadsheet_id: &str,
        grades: Vec<(AssessmentKind, EmployeeSkills)>,
    ) -> Result<(), Box<dyn std_err>> {
        let mut spreadsheet_values = SpreadsheetValueRange {
            range: range.to_owned(),
            major_dimension: MajorDimension::Rows,
            values: Vec::with_capacity(3),
        };

        let mut aggregated: Vec<Vec<String>> = vec![
            vec!["Feedback Kind / Category".to_owned()],
            vec![AssessmentKind::TeamFeedback.to_string()],
            vec![AssessmentKind::SelfAssessment.to_string()],
        ];

        for (index, (feedback_kind, graded_skills)) in grades.into_iter().enumerate() {
            for question in &graded_skills.skills {
                if index == 0 {
                    aggregated[0].push(question.name.to_string());
                }

                match feedback_kind {
                    AssessmentKind::TeamFeedback => {
                        aggregated[1].push(question.avg().to_string());
                    }
                    AssessmentKind::SelfAssessment => {
                        aggregated[2].push(question.avg().to_string())
                    }
                }
            }
        }

        spreadsheet_values.set_values(aggregated);

        self.sheets_client.append_values(
            &self.access_token,
            spreadsheet_id.to_owned(),
            range.to_owned(),
            &spreadsheet_values,
        )?;

        Ok(())
    }

    fn save_text(
        &self,
        range: &str,
        spreadsheet_id: &str,
        feedbacks: Vec<(AssessmentKind, EmployeeSkills)>,
    ) -> Result<(), Box<dyn std_err>> {
        let mut spreadsheet_values = SpreadsheetValueRange {
            range: range.to_owned(),
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
            range.to_owned(),
            &spreadsheet_values,
        )?;

        Ok(())
    }
}
