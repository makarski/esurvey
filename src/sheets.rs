use std::error::Error;
use std::io::{Error as io_err, ErrorKind as io_err_kind};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spreadsheet {
    pub spreadsheet_id: String,
    // #[serde(skip)]
    // properties: Vec<SpreadsheetProperties>,

    // #[serde(skip)]
    pub sheets: Vec<Sheet>,

    // #[serde(skip)]
    // named_ranges: Option<Vec<NamedRange>>,

    // #[serde(skip)]
    pub spreadsheet_url: String,
    // #[serde(skip)]
    // developer_metadata: Option<Vec<DeveloperMetadata>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpreadsheetProperties {
    title: String,

    #[serde(skip)]
    locale: String,

    #[serde(skip)]
    auto_recalc: Option<RecalculationInterval>,

    #[serde(skip)]
    time_zone: String,

    #[serde(skip)]
    default_format: Option<CellFormat>,

    #[serde(skip)]
    iterative_calculation_settings: Option<IterativeCalculationSettings>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecalculationInterval {
    // Default value. This value must not be used.
    RecalculationIntervalUnspecified,
    // Volatile functions are updated on every change.
    OnChange,
    // Volatile functions are updated on every change and every minute.
    Minute,
    // Volatile functions are updated on every change and hourly.
    Hour,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CellFormat {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IterativeCalculationSettings {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sheet {
    pub properties: SheetProperties,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SheetProperties {
    pub sheet_id: Option<u64>,
    pub title: String,
    pub index: Option<u64>,
    pub sheet_type: Option<SheetType>,

    // #[serde(skip)]
    pub grid_properties: Option<GridProperties>,

    // hidden: bool,

    // #[serde(skip)]
    // tab_color: Option<Color>,
    // #[serde(skip)]
    // right_to_left: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SheetType {
    // Default value, do not use.
    SheetTypeUnspecified,
    // The sheet is a grid.
    Grid,
    // The sheet has no grid and instead has an object like a chart or image.
    Object,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GridProperties {
    row_count: Option<u64>,
    column_count: Option<u64>,
    frozen_row_count: Option<u64>,
    frozen_column_count: Option<u64>,
    hide_gridlines: Option<u64>,
    row_group_control_after: Option<bool>,
    column_group_control_after: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NamedRange {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperMetadata {}

pub struct Client {
    _http_client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Client {
            _http_client: reqwest::Client::new(),
        }
    }

    // https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/get
    // GET https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}
    pub fn get_spreadsheet<S: AsRef<str>>(
        &self,
        token: S,
        spreadsheet_id: S,
    ) -> Result<Spreadsheet, Box<dyn Error>> {
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}?access_token={}",
            spreadsheet_id.as_ref(),
            token.as_ref(),
        );

        let mut resp = self._http_client.get(url.as_str()).send()?;

        match resp.status().is_success() {
            true => Ok(resp.json::<Spreadsheet>()?),
            _ => panic!(resp.text()?), // todo: change to return the error
        }
    }

    // https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/batchUpdate
    // POST https://sheets.googleapis.com/v4/spreadsheets/spreadsheetId:batchUpdate
    pub fn batch_update_spreadsheet<S: AsRef<str>>(
        &self,
        token: S,
        spreadsheet_id: S,
        req: &spreadsheets_batch_update::SpreadsheetBatchUpdate,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}:batchUpdate?access_token={}",
            spreadsheet_id.as_ref(),
            token.as_ref()
        );
        let mut resp = self
            ._http_client
            .post(url.as_str())
            .body(serde_json::to_vec(req)?)
            .send()?;

        match resp.status().is_success() {
            true => Ok(()),
            _ => panic!(resp.text()?),
        }
    }

    // https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/batchGet
    // GET https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values:batchGet
    pub fn get_batch_values<S: AsRef<str>>(
        &self,
        token: S,
        spreadsheet_id: S,
        ranges: Vec<String>,
    ) -> Result<spreadsheets_values::SpreadsheetValues, Box<dyn Error>> {
        let range_query_str = ranges[..].join("&ranges=");

        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values:batchGet?ranges={}&majorDimension=COLUMNS&access_token={}",
            spreadsheet_id.as_ref(),
            range_query_str,
            token.as_ref(),
        );

        Ok(self
            ._http_client
            .get(url.as_str())
            .send()?
            .json::<spreadsheets_values::SpreadsheetValues>()?)
    }

    // https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/update
    // PUT https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}
    pub fn update_values<S: AsRef<str>>(
        &self,
        token: S,
        spreadsheet_id: String,
        range: String,
        v: &spreadsheets_values::SpreadsheetValueRange,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?access_token={}&valueInputOption=USER_ENTERED",
            spreadsheet_id,
            range,
            token.as_ref(),
        );

        let mut resp: reqwest::Response = self
            ._http_client
            .put(url.as_str())
            .body(serde_json::to_vec(v)?)
            .send()?;

        match resp.status().is_success() {
            true => Ok(()),
            _ => Err(Box::new(io_err::new(
                io_err_kind::InvalidData,
                resp.text()?,
            ))),
        }
    }

    // https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/append
    // POST https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}:append
    pub fn append_values<S: AsRef<str>>(
        &self,
        token: S,
        spreadsheet_id: String,
        range: String,
        v: &spreadsheets_values::SpreadsheetValueRange,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}:append?access_token={}&valueInputOption=USER_ENTERED",
            spreadsheet_id,
            range,
            token.as_ref(),
        );

        let mut resp = self
            ._http_client
            .post(url.as_str())
            .body(serde_json::to_vec(v)?)
            .send()?;

        match resp.status().is_success() {
            true => Ok(()),
            _ => Err(Box::new(io_err::new(
                io_err_kind::InvalidData,
                resp.text()?,
            ))),
        }
    }
}

mod spreadsheets_sheets {}

pub mod spreadsheets_values {
    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SpreadsheetValues {
        pub spreadsheet_id: String,
        pub value_ranges: Vec<SpreadsheetValueRange>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SpreadsheetValueRange {
        pub range: String,
        pub major_dimension: String, // todo: change to enum
        pub values: Vec<Vec<String>>,
    }

    impl SpreadsheetValueRange {
        pub fn add_value(&mut self, v: Vec<String>) {
            self.values.push(v)
        }
    }
}

pub mod spreadsheets_batch_update {
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
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct AddSheetRequest {
        pub properties: super::SheetProperties,
    }
}
