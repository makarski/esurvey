use super::spreadsheets;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasicChartSpec {
    pub chart_type: BasicChartType,
    pub legend_position: BasicChartLegendPosition,
    pub axis: Vec<BasicChartAxis>,
    pub domains: Vec<BasicChartDomain>,
    pub series: Vec<BasicChartSeries>,
    pub header_count: u64,
    pub three_dimensional: bool,
    pub interpolate_nulls: bool,
    pub stacked_type: BasicChartStackedType,
    pub line_smoothing: bool,
    pub compare_mode: BasicChartCompareMode,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasicChartType {
    // Default value, do not use.
    BasicChartTypeUnspecified,

    //	A bar chart.
    Bar,

    // A line chart.
    Line,

    // An area chart.
    Area,

    // A column chart.
    Column,

    // A scatter chart.
    Scatter,

    // A combo chart.
    Combo,

    // A stepped area chart.
    SteppedArea,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasicChartLegendPosition {
    // Default value, do not use.
    BasicChartLegendPositionUnspecified,

    // The legend is rendered on the bottom of the chart.
    BottomLegend,
    // The legend is rendered on the left of the chart.
    LeftLegend,

    // The legend is rendered on the right of the chart.
    RightLegend,
    // The legend is rendered on the top of the chart.
    TopLegend,

    // No legend is rendered.
    NoLegend,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasicChartAxis {
    pub position: BasicChartAxisPosition,
    pub title: String,
    pub format: spreadsheets::TextFormat,
    pub title_text_position: spreadsheets::TextPosition,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasicChartAxisPosition {
    // Default value, do not use.
    BasicChartAxisPositionUnspecified,

    // The axis rendered at the bottom of a chart. For most charts, this is the standard major axis. For bar charts, this is a minor axis.
    BottomAxis,
    // The axis rendered at the left of a chart. For most charts, this is a minor axis. For bar charts, this is the standard major axis.
    LeftAxis,
    // The axis rendered at the right of a chart. For most charts, this is a minor axis. For bar charts, this is an unusual major axis.
    RightAxis,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasicChartDomain {
    // The data of the domain. For example, if charting stock prices over time, this is the data representing the dates.
    pub domain: ChartData,
    // True to reverse the order of the domain values (horizontal axis).
    pub reversed: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChartData {
    // The source ranges of the data.
    pub source_range: ChartSourceRange,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChartSourceRange {
    //  The ranges of data for a series or domain.
    // Exactly one dimension must have a length of 1, and all sources in the list must have the same dimension with length 1.
    // The domain (if it exists) & all series must have the same number of source ranges.
    // If using more than one source range, then the source range at a given offset must be in order and contiguous across the domain and series.
    //
    // For example, these are valid configurations:
    // ```
    //   domain sources: A1:A5
    //   series1 sources: B1:B5
    //   series2 sources: D6:D10
    //
    //   domain sources: A1:A5, C10:C12
    //   series1 sources: B1:B5, D10:D12
    //   series2 sources: C1:C5, E10:E12
    // ```
    pub sources: Vec<GridRange>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GridRange {
    // The sheet this range is on.
    pub sheet_id: u64,

    // The start row (inclusive) of the range, or not set if unbounded.
    pub start_row_index: u64,

    // The end row (exclusive) of the range, or not set if unbounded.
    pub end_row_index: u64,

    // The start column (inclusive) of the range, or not set if unbounded.
    pub start_column_index: u64,

    // The end column (exclusive) of the range, or not set if unbounded.
    pub end_column_index: u64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasicChartSeries {
    // The data being visualized in this chart series.
    pub series: ChartData,

    // The minor axis that will specify the range of values for this series.
    // For example, if charting stocks over time,
    // the "Volume" series may want to be pinned to the right with the prices pinned to the left,
    // because the scale of trading volume is different than the scale of prices.
    // It is an error to specify an axis that isn't a valid minor axis for the chart's type.
    pub target_axis: BasicChartAxisPosition,

    // The type of this series.
    // Valid only if the chartType is COMBO.
    // Different types will change the way the series is visualized.
    // Only LINE, AREA, and COLUMN are supported.
    #[serde(rename = "type")]
    pub chart_type: Option<BasicChartType>,

    // The line style of this series.
    // Valid only if the chartType is AREA , LINE , or SCATTER.
    // COMBO charts are also supported if the series chart type is AREA or LINE.
    pub line_style: Option<LineStyle>,

    // The color for elements (i.e. bars, lines, points) associated with this series.
    // If empty, a default color is used.
    pub color: Option<spreadsheets::Color>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LineStyle {
    // The thickness of the line, in px.
    pub width: u64,

    // The dash type of the line.
    #[serde(rename = "type")]
    pub line_type: LineDashType,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LineDashType {
    // Default value, do not use.
    LineDashTypeUnspecified,

    // No dash type, which is equivalent to a non-visible line.
    Invisible,

    // A custom dash for a line. Modifying the exact custom dash style is currently unsupported.
    Custom,

    // A solid line.
    Solid,

    // A dotted line.
    Dotted,

    // A dashed line where the dashes have "medium" length.
    MediumDashed,

    // A line that alternates between a "medium" dash and a dot.
    MediumDashedDotted,

    // A dashed line where the dashes have "long" length.
    LongDashed,

    // A line that alternates between a "long" dash and a dot.
    LongDashedDotted,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasicChartStackedType {
    // Default value, do not use.
    BasicChartStackedTypeUnspecified,

    // Series are not stacked.
    NotStacked,

    // Series values are stacked, each value is rendered vertically beginning from the top of the value below it.
    Stacked,

    // Vertical stacks are stretched to reach the top of the chart,
    // with values laid out as percentages of each other.
    PercentStacked,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasicChartCompareMode {
    // Default value, do not use.
    BasicChartCompareModeUnspecified,

    // Only the focused data element is highlighted and shown in the tooltip.
    Datum,

    // All data elements with the same category (e.g., domain value) are highlighted and shown in the tooltip.
    Category,
}
