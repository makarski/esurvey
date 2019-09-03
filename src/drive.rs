use crate::config;
use crate::config::{AssessmentKind, ResponseKind};
use crate::sheets;
use crate::skill2::{Responses, Survey};

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
    texts: Vec<(AssessmentKind, Vec<Responses>)>,
    grades2: Vec<(AssessmentKind, Vec<Responses>)>,
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
        first_name: &str,
    ) -> Result<Summary, Box<dyn std_err>> {
        let templates = config::read(config_file, vec![("{name}", first_name)])?;

        let grade_survey = Survey::new(ResponseKind::Grade, &templates)?;
        let text_survey = Survey::new(ResponseKind::Text, &templates)?;

        let mut texts: Vec<(AssessmentKind, Vec<Responses>)> = Vec::new();
        let mut grades: Vec<(AssessmentKind, Vec<Responses>)> = Vec::new();

        for sheet in sheet_items.into_iter() {
            println!("> reading data from sheet tab: {}", sheet.properties.title);

            let sheet_title = sheet.properties.title;
            let feedback_kind = AssessmentKind::from_str(sheet_title.as_ref())?;

            let sheet_vals = self.sheets_client.get_batch_values(
                &self.access_token,
                spreadsheet_id,
                vec![sheet_title],
            )?;

            for val_range in sheet_vals.value_ranges.into_iter() {
                println!(">>> collecting grades!");
                grades.push((feedback_kind.clone(), grade_survey.scan(&val_range.values)?));

                println!(">>> collecting textual feedback!");
                texts.push((feedback_kind.clone(), text_survey.scan(&val_range.values)?));
            }
        }

        Ok(Summary {
            texts: texts,
            grades2: grades,
        })
    }

    pub fn save_summary(
        &self,
        range: &str,
        spreadsheet_id: &str,
        summary: Summary,
    ) -> Result<(), Box<dyn std_err>> {
        for (response_data, response_kind) in [
            (summary.grades2, ResponseKind::Grade),
            (summary.texts, ResponseKind::Text),
        ]
        .into_iter()
        {
            if response_data.len() == 0 {
                continue;
            }

            let rows: SummaryRows = process_grades_reviews(response_kind, response_data);
            let spreadsheet_values = SpreadsheetValueRange {
                range: range.to_owned(),
                major_dimension: MajorDimension::Rows,
                values: rows.rows(),
            };

            self.sheets_client.append_values(
                &self.access_token,
                spreadsheet_id.to_owned(),
                range.to_owned(),
                &spreadsheet_values,
            )?;
        }
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

        let response_body = self
            .sheets_client
            .batch_update_spreadsheet(self.access_token.as_str(), spreadsheet_id, &batch_update)
            .map_err(|err| format!("add_summary_sheet: {}", err))?;

        if let Some(reply) = response_body.replies.get(0) {
            if let Some(sheet) = &reply.add_sheet {
                if let Some(sheet_id) = sheet.properties.sheet_id {
                    return Ok(sheet_id);
                }
            }
        }

        return Err(Box::from(format!(
            "add_summary_sheet: sheet_id not available. spreadsheet_id: {}",
            spreadsheet_id
        )));
    }
}

struct SummaryRows {
    base: HashMap<String, Vec<String>>,
    ordered_keys: Vec<String>,
}

impl SummaryRows {
    fn new() -> Self {
        SummaryRows {
            base: HashMap::new(),
            ordered_keys: Vec::new(),
        }
    }

    fn add_cell(&mut self, group_key: &str, v: &str) {
        let exists = self.base.get(group_key);

        // pattern matching is used as a workaround for borrow checker
        // since we need to append to ordered_keys that needs mutable access to self
        // that results in 2 mut borrows for inserting into map and a vector
        match exists {
            Some(_) => {
                self.base
                    .entry(String::from(group_key))
                    .and_modify(|e| e.push(String::from(v)));
            }
            None => {
                self.base.insert(
                    String::from(group_key),
                    vec![String::from(group_key), String::from(v)],
                );

                self.ordered_keys.push(String::from(group_key));
            }
        };
    }

    fn rows(&self) -> Vec<Vec<String>> {
        let mut out: Vec<Vec<String>> = Vec::new();
        for key in &self.ordered_keys {
            let key_str: &str = key.as_ref();
            if let Some(v) = self.base.get(key_str) {
                out.push(v.deref().to_vec());
            }
        }

        out
    }
}

fn process_grades_reviews(
    response_kind: &ResponseKind,
    data: &Vec<(AssessmentKind, Vec<Responses>)>,
) -> SummaryRows {
    match response_kind {
        ResponseKind::Grade => process_grades(data),
        ResponseKind::Text => process_reviews(data),
    }
}

fn process_grades(grades: &Vec<(AssessmentKind, Vec<Responses>)>) -> SummaryRows {
    let mut rows = SummaryRows::new();

    for (index, (assessment_kind, by_category)) in grades.into_iter().enumerate() {
        for category in by_category {
            if index == 0 {
                rows.add_cell("Feedback Kind / Category", category.category_name.as_ref());
            }

            rows.add_cell(
                assessment_kind.to_string().as_ref(),
                ResponseKind::Grade
                    .process_data(category.read())
                    .unwrap()
                    .as_ref(),
            );
        }
    }

    rows
}

fn process_reviews(reviews: &Vec<(AssessmentKind, Vec<Responses>)>) -> SummaryRows {
    let mut rows = SummaryRows::new();

    for (assessment_kind, by_category) in reviews.into_iter() {
        rows.add_cell("Skill / Audience", assessment_kind.to_string().as_ref());

        for category in by_category {
            rows.add_cell(
                category.category_name.as_ref(),
                ResponseKind::Text
                    .process_data(category.read())
                    .unwrap()
                    .as_ref(),
            );
        }
    }

    rows
}
