use anyhow::ensure;

pub mod spreadsheets;
use spreadsheets::Spreadsheet;

pub mod spreadsheets_batch_update;
pub mod spreadsheets_values;
mod spreadsheets_sheets {}

pub mod basic_chart;

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
    ) -> anyhow::Result<Spreadsheet> {
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}?access_token={}",
            spreadsheet_id.as_ref(),
            token.as_ref(),
        );

        let mut resp = self._http_client.get(url.as_str()).send()?;

        ensure!(resp.status().is_success(), resp.text()?);
        Ok(resp.json::<Spreadsheet>()?)
    }

    // https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/batchUpdate
    // POST https://sheets.googleapis.com/v4/spreadsheets/spreadsheetId:batchUpdate
    pub fn batch_update_spreadsheet<S: AsRef<str>>(
        &self,
        token: S,
        spreadsheet_id: S,
        req: &spreadsheets_batch_update::SpreadsheetBatchUpdate,
    ) -> anyhow::Result<spreadsheets_batch_update::BatchUpdateResponse> {
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

        ensure!(resp.status().is_success(), resp.text()?);
        Ok(resp.json::<spreadsheets_batch_update::BatchUpdateResponse>()?)
    }

    // https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/batchGet
    // GET https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values:batchGet
    pub fn get_batch_values<S: AsRef<str>>(
        &self,
        token: S,
        spreadsheet_id: S,
        ranges: Vec<String>,
    ) -> anyhow::Result<spreadsheets_values::SpreadsheetValues> {
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
    #[allow(dead_code)]
    pub fn update_values<S: AsRef<str>>(
        &self,
        token: S,
        spreadsheet_id: String,
        range: String,
        v: &spreadsheets_values::SpreadsheetValueRange,
    ) -> anyhow::Result<()> {
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

        ensure!(resp.status().is_success(), resp.text()?);
        Ok(())
    }

    // https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/append
    // POST https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}:append
    pub fn append_values<S: AsRef<str>>(
        &self,
        token: S,
        spreadsheet_id: String,
        range: String,
        v: &spreadsheets_values::SpreadsheetValueRange,
    ) -> anyhow::Result<()> {
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

        ensure!(resp.status().is_success(), resp.text()?);
        Ok(())
    }
}
