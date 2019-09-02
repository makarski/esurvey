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
    pub major_dimension: MajorDimension,
    pub values: Vec<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MajorDimension {
    Columns,
    Rows,
}
