use crate::config::{AssessmentKind, EmployeeSkill, Skill};

use crate::sheets;
use sheets::basic_chart::*;
use sheets::spreadsheets::{ChartSpec, EmbeddedChart, EmbeddedObjectPosition};
use sheets::spreadsheets_batch_update::*;
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
    feedbacks: Vec<(AssessmentKind, Vec<EmployeeSkill>)>,
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

        for stmt_feedback in stmt_feedbacks {
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

// https://developers.google.com/sheets/api/samples/charts#add_a_column_chart
pub fn add_summary_chart(
    client: &sheets::Client,
    token: &str,
    spreadsheet_id: &str,
    sheet_id: u64,
) {
    let chart_spec = ChartSpec {
        title: Some("Team Feedback and Self-Assessment SCRIPT".to_owned()),
        basic_chart: Some(BasicChartSpec {
            chart_type: BasicChartType::Column,
            legend_position: BasicChartLegendPosition::RightLegend,
            axis: Vec::new(),
            domains: vec![BasicChartDomain {
                domain: ChartData {
                    source_range: ChartSourceRange {
                        sources: vec![GridRange {
                            sheet_id: sheet_id,
                            start_row_index: 0,
                            end_row_index: 1,
                            start_column_index: 0,
                            end_column_index: 13,
                        }],
                    },
                },
                reversed: false,
            }],
            series: vec![
                BasicChartSeries {
                    series: ChartData {
                        source_range: ChartSourceRange {
                            sources: vec![GridRange {
                                sheet_id: sheet_id,
                                start_row_index: 1,
                                end_row_index: 2,
                                start_column_index: 0,
                                end_column_index: 13,
                            }],
                        },
                    },
                    target_axis: BasicChartAxisPosition::LeftAxis,
                    chart_type: Some(BasicChartType::Column),
                    line_style: None,
                    color: None,
                },
                BasicChartSeries {
                    series: ChartData {
                        source_range: ChartSourceRange {
                            sources: vec![GridRange {
                                sheet_id: sheet_id,
                                start_row_index: 2,
                                end_row_index: 3,
                                start_column_index: 0,
                                end_column_index: 13,
                            }],
                        },
                    },
                    target_axis: BasicChartAxisPosition::LeftAxis,
                    chart_type: Some(BasicChartType::Column),
                    line_style: None,
                    color: None,
                },
            ],
            header_count: 1,
            three_dimensional: false,
            interpolate_nulls: false,
            stacked_type: BasicChartStackedType::NotStacked,
            line_smoothing: false,
            compare_mode: BasicChartCompareMode::Category,
        }),
        ..Default::default()
    };

    let chart_req = SpreadsheetBatchUpdate {
        requests: vec![Request {
            add_sheet: None,
            add_chart: Some(AddChartRequest {
                chart: EmbeddedChart {
                    chart_id: None,
                    spec: chart_spec,
                    position: EmbeddedObjectPosition {
                        new_sheet: true,
                        ..Default::default()
                    },
                },
            }),
        }],
        response_ranges: Vec::new(),
        response_include_grid_data: false,
        include_spreadsheet_in_response: false,
    };

    client
        .batch_update_spreadsheet(token, spreadsheet_id, &chart_req)
        .expect("failed to add chart");
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
