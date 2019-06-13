#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpreadsheetBatchUpdate {
    pub requests: Vec<Request>,
    pub include_spreadsheet_in_response: bool,
    pub response_ranges: Vec<String>,
    pub response_include_grid_data: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub add_sheet: Option<AddSheetRequest>,
    pub add_chart: Option<AddChartRequest>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddSheetRequest {
    pub properties: super::spreadsheets::SheetProperties,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddChartRequest {
    pub chart: super::spreadsheets::EmbeddedChart,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// If successful, the response body contains data with the following structure:
pub struct BatchUpdateResponse {
    /// The spreadsheet the updates were applied to.
    pub spreadsheet_id: String,
    /// The reply of the updates.
    /// This maps 1:1 with the updates, although replies to some requests may be empty.
    pub replies: Vec<Response>,
    /// The spreadsheet after updates were applied.
    /// This is only set if [BatchUpdateSpreadsheetRequest.include_spreadsheet_in_response] is `true`.
    pub updated_spreadsheet: super::Spreadsheet,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    // TODO: implement remaining responses
    // https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/response#Response

    // Union field kind can be only one of the following:
    /// A reply from adding a sheet.
    pub add_sheet: Option<AddSheetResponse>,
    /// A reply from adding a chart.
    pub add_chart: Option<AddChartResponse>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
/// The result of adding a sheet.
pub struct AddSheetResponse {
    /// The properties of the newly added sheet.
    pub properties: super::spreadsheets::SheetProperties,
}

#[derive(Deserialize, Serialize, Debug, Default)]
/// The result of adding a chart to a spreadsheet.
pub struct AddChartResponse {
    /// The newly added chart.
    pub chart: super::spreadsheets::EmbeddedChart,
}
