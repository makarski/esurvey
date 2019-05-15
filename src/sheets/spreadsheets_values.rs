
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
