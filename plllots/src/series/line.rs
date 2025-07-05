use bon::Builder;
use kurbo::Stroke;
use peniko::Brush;

#[derive(Debug, Builder, Clone)]
pub struct Line {
    #[builder(setters(option_fn(vis = "")))]
    pub stroke: Option<Stroke>,
    #[builder(setters(option_fn(vis = "")))]
    pub color: Option<Brush>,
    #[builder(default = 0, setters(option_fn(vis = "")))]
    pub x_axis_index: usize,
    #[builder(default = 0, setters(option_fn(vis = "")))]
    pub y_axis_index: usize,
    #[builder(setters(option_fn(vis = "")))]
    pub symbol_show: Option<bool>,
    #[builder(setters(option_fn(vis = "")))]
    pub symbol_stroke: Option<Stroke>,
    #[builder(setters(option_fn(vis = "")))]
    pub symbol_stroke_color: Option<Brush>,
    #[builder(setters(option_fn(vis = "")))]
    pub symbol_fill_color: Option<Brush>,
    #[builder(setters(option_fn(vis = "")))]
    pub symbol_size: Option<f64>,
    #[builder(into)]
    pub data: LineData,
}

#[derive(Debug, Builder, Clone)]
pub struct LineData {
    #[builder(setters(option_fn(vis = "")))]
    pub primary_data_index: Option<usize>,
    #[builder(setters(option_fn(vis = "")))]
    pub secondary_data_index: Option<usize>,
    pub data: Vec<Vec<f64>>,
}

impl From<Vec<f64>> for LineData {
    fn from(value: Vec<f64>) -> Self {
        LineData {
            primary_data_index: None,
            secondary_data_index: None,
            data: vec![value],
        }
    }
}

impl From<Vec<Vec<f64>>> for LineData {
    fn from(value: Vec<Vec<f64>>) -> Self {
        LineData {
            primary_data_index: None,
            secondary_data_index: None,
            data: value,
        }
    }
}
