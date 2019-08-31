use crate::config;
use crate::config::{AssessmentKind, ResponseKind};
use crate::sheets;
use crate::skill2::{CategoryResponse, Survey};

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
    texts: Vec<(AssessmentKind, Vec<Box<dyn CategoryResponse>>)>,
    grades2: Vec<(AssessmentKind, Vec<Box<dyn CategoryResponse>>)>,
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

        let mut texts: Vec<(AssessmentKind, Vec<Box<dyn CategoryResponse>>)> = Vec::new();
        let mut grades: Vec<(AssessmentKind, Vec<Box<dyn CategoryResponse>>)> = Vec::new();

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
        if summary.grades2.len() > 0 {
            self.save_grades2(range, spreadsheet_id, summary.grades2)?;
        }
        if summary.texts.len() > 0 {
            self.save_text(range, spreadsheet_id, summary.texts)?;
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

    fn save_grades2(
        &self,
        range: &str,
        spreadsheet_id: &str,
        grades: Vec<(AssessmentKind, Vec<Box<dyn CategoryResponse>>)>,
    ) -> Result<(), Box<dyn std_err>> {
        let mut aggregated: HashMap<String, Vec<String>> = HashMap::new();

        let header_name = String::from("header");
        aggregated.insert(
            header_name.clone(),
            vec![String::from("Feedback Kind / Category")],
        );

        for (index, (feedback_kind, by_category)) in grades.into_iter().enumerate() {
            for category in by_category {
                if index == 0 {
                    aggregated
                        .entry(header_name.clone())
                        .and_modify(|e| e.push(category.name()));
                }

                match feedback_kind {
                    AssessmentKind(ref name) => aggregated
                        .entry(name.clone())
                        .and_modify(|e| e.push(category.read().unwrap().to_string()))
                        .or_insert(vec![name.clone(), category.read().unwrap().to_string()]),
                };
            }
        }

        let header_row = aggregated
            .get(&header_name)
            .ok_or("error retrieving the header row")?
            .deref()
            .to_vec();

        let mut spreadsheet_values = SpreadsheetValueRange {
            range: range.to_owned(),
            major_dimension: MajorDimension::Rows,
            values: Vec::with_capacity(3),
        };

        spreadsheet_values.add_value(header_row);

        for (key, val) in aggregated.iter() {
            if *key == header_name {
                continue;
            }

            spreadsheet_values.add_value(val.deref().to_vec());
        }

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
        feedbacks: Vec<(AssessmentKind, Vec<Box<dyn CategoryResponse>>)>,
    ) -> Result<(), Box<dyn std_err>> {
        let mut aggregated: HashMap<String, Vec<String>> = HashMap::new();
        let mut aggreated_kinds: Vec<String> = Vec::with_capacity(feedbacks.len() + 1 as usize);

        aggreated_kinds.push("Skill / Audience".to_owned());

        for (fdb_kind, by_category) in feedbacks.into_iter() {
            aggreated_kinds.push(fdb_kind.to_string());

            for category in by_category {
                aggregated
                    .entry(category.name())
                    .and_modify(|e| e.push(category.read().unwrap().to_string()))
                    .or_insert(vec![category.name(), category.read().unwrap().to_string()]);
            }
        }

        let mut spreadsheet_values = SpreadsheetValueRange {
            range: range.to_owned(),
            major_dimension: MajorDimension::Rows,
            values: Vec::new(),
        };

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
