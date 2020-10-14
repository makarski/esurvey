use anyhow::anyhow;

use crate::sheets;
use crate::survey::summary::Summary;

use sheets::spreadsheets::{Sheet, SheetProperties};
use sheets::spreadsheets_batch_update::{AddSheetRequest, Request, SpreadsheetBatchUpdate};
use sheets::spreadsheets_values::{MajorDimension, SpreadsheetValueRange};

pub struct SpreadsheetClient<'a> {
    sheets_client: &'a sheets::Client,
    access_token: &'a str,
}

impl<'a> SpreadsheetClient<'a> {
    pub fn new(sheets_client: &'a sheets::Client, access_token: &'a str) -> Self {
        SpreadsheetClient {
            sheets_client,
            access_token,
        }
    }

    pub fn retrieve_sheet_data(
        &self,
        sheet_items: &[Sheet],
        spreadsheet_id: &str,
    ) -> anyhow::Result<Vec<SpreadsheetValueRange>> {
        let sheet_titles = retrieve_sheet_titles(sheet_items);
        println!("sheet titles: > {:#?}", &sheet_titles);

        Ok(self
            .sheets_client
            .get_batch_values(self.access_token, spreadsheet_id, sheet_titles)?
            .value_ranges)
    }

    pub fn save_summary(
        &self,
        range: &str,
        spreadsheet_id: &str,
        summary: Summary,
    ) -> anyhow::Result<()> {
        for rows in summary.generate_rows() {
            let spreadsheet_values = SpreadsheetValueRange {
                range: range.to_owned(),
                major_dimension: MajorDimension::Rows,
                values: rows.rows(),
            };

            self.sheets_client.append_values(
                self.access_token,
                spreadsheet_id.to_owned(),
                range.to_owned(),
                &spreadsheet_values,
            )?;
        }

        Ok(())
    }

    pub fn add_summary_sheet(&self, title: &str, spreadsheet_id: &str) -> anyhow::Result<u64> {
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
            .batch_update_spreadsheet(self.access_token, spreadsheet_id, &batch_update)
            .map_err(|err| anyhow!("add_summary_sheet: {}", err))?;

        if let Some(reply) = response_body.replies.get(0) {
            if let Some(sheet) = &reply.add_sheet {
                if let Some(sheet_id) = sheet.properties.sheet_id {
                    return Ok(sheet_id);
                }
            }
        }

        Err(anyhow!(
            "add_summary_sheet: sheet_id not available. spreadsheet_id: {}",
            spreadsheet_id
        ))
    }
}

fn retrieve_sheet_titles(sheet_items: &[Sheet]) -> Vec<String> {
    sheet_items
        .iter()
        .map(|sheet| sheet.properties.title.clone())
        .collect::<Vec<String>>()
}
