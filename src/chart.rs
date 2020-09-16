use crate::sheets;

use sheets::basic_chart::*;
use sheets::spreadsheets::{ChartSpec, EmbeddedChart, EmbeddedObjectPosition};
use sheets::spreadsheets_batch_update::*;
use std::error::Error;

// https://developers.google.com/sheets/api/samples/charts#add_a_column_chart
pub fn add_summary_chart(
    client: &sheets::Client,
    token: &str,
    spreadsheet_id: &str,
    sheet_id: u64,
    title: String,
) -> Result<(), Box<dyn Error>> {
    let chart_spec = ChartSpec {
        title: Some(title),
        basic_chart: Some(BasicChartSpec {
            chart_type: BasicChartType::Column,
            legend_position: BasicChartLegendPosition::RightLegend,
            axis: Vec::new(),
            domains: vec![BasicChartDomain {
                domain: ChartData {
                    source_range: ChartSourceRange {
                        sources: vec![GridRange {
                            sheet_id,
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
                                sheet_id,
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
                                sheet_id,
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

    client.batch_update_spreadsheet(token, spreadsheet_id, &chart_req)?;
    Ok(())
}
