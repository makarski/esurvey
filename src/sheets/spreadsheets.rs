#[derive(Deserialize, Serialize, Debug, Default)]
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
    /// Default value. This value must not be used.
    RecalculationIntervalUnspecified,
    // Volatile functions are updated on every change.
    OnChange,
    /// Volatile functions are updated on every change and every minute.
    Minute,
    /// Volatile functions are updated on every change and hourly.
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

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SheetProperties {
    pub sheet_id: Option<u64>,
    pub title: String,
    pub index: Option<u64>,
    pub sheet_type: Option<SheetType>,
    pub grid_properties: Option<GridProperties>,
    pub hidden: Option<bool>,
    pub tab_color: Option<Color>,
    pub right_to_left: Option<bool>,
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

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    /// The amount of red in the color as a value in the interval [0, 1].
    pub red: f64,

    /// The amount of green in the color as a value in the interval [0, 1].
    pub green: f64,

    /// The amount of blue in the color as a value in the interval [0, 1].
    pub blue: f64,

    /// The fraction of this color that should be applied to the pixel. That is, the final pixel color is defined by the equation:
    ///
    /// `pixel color = alpha * (this color) + (1.0 - alpha) * (background color)`
    ///
    /// This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color.
    /// This uses a wrapper message rather than a simple float scalar so that it is possible to distinguish between a default value and the value being unset.
    /// If omitted, this color object is to be rendered as a solid color (as if the alpha value had been explicitly given with a value of 1.0).
    pub alpha: f64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HorizontalAlign {
    // The horizontal alignment is not specified. Do not use this.
    HorizontalAlignUnspecified,

    // The text is explicitly aligned to the left of the cell.
    Left,

    // The text is explicitly aligned to the center of the cell.
    Center,

    // The text is explicitly aligned to the right of the cell.
    Right,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedChart {
    pub chart_id: Option<u64>,
    pub spec: ChartSpec,
    pub position: EmbeddedObjectPosition,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChartSpec {
    pub title: Option<String>,
    pub alt_text: Option<String>,
    pub title_text_format: Option<TextFormat>,
    pub title_text_position: Option<TextPosition>,

    pub subtitle: Option<String>,
    pub subtitle_text_format: Option<TextFormat>,
    pub subtitle_text_position: Option<TextPosition>,

    pub font_name: Option<String>,
    pub maximized: Option<bool>,
    pub background_color: Option<Color>,
    pub hidden_dimension_strategy: Option<ChartHiddenDimensionStrategy>,

    pub basic_chart: Option<super::basic_chart::BasicChartSpec>,
    // todo: implement other chart types
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TextFormat {
    #[serde(default)]
    pub foreground_color: Color,

    #[serde(default)]
    pub font_family: String,

    #[serde(default)]
    pub font_size: u64,

    #[serde(default)]
    pub bold: bool,

    #[serde(default)]
    pub italic: bool,

    #[serde(default)]
    pub strikethrough: bool,

    #[serde(default)]
    pub underline: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextPosition {
    horizontal_alignment: HorizontalAlign,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChartHiddenDimensionStrategy {
    // Default value, do not use.
    ChartHiddenDimensionStrategyUnspecified,

    // Charts will skip hidden rows and columns.
    SkipHiddenRowsAndColumns,
    // Charts will skip hidden rows only.
    SkipHiddenRows,

    // Charts will skip hidden columns only.
    SkipHiddenColumns,
    // Charts will not skip any hidden rows or columns.
    ShowAll,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedObjectPosition {
    // The sheet this is on. Set only if the embedded object is on its own sheet.
    // Must be non-negative.
    pub sheet_id: Option<u64>,
    // The position at which the object is overlaid on top of a grid.
    pub overlay_position: Option<OverlayPosition>,
    // If true, the embedded object is put on a new sheet whose ID is chosen for you.
    // Used only when writing.
    #[serde(default)]
    pub new_sheet: bool,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct OverlayPosition {
    // The cell the object is anchored to.
    pub anchor_cell: GridCoordinate,
    // The horizontal offset, in pixels, that the object is offset from the anchor cell.
    pub offset_x_pixels: Option<u64>,
    // The vertical offset, in pixels, that the object is offset from the anchor cell.
    pub offset_y_pixels: Option<u64>,
    // The width of the object, in pixels. Defaults to 600.
    pub width_pixels: Option<u64>,
    // The height of the object, in pixels. Defaults to 371.
    pub height_pixels: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GridCoordinate {
    // The sheet this coordinate is on.
    pub sheet_id: u64,
    // The row index of the coordinate.
    pub row_index: u64,
    // The column index of the coordinate.
    pub column_index: u64,
}
