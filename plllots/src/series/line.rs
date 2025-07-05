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
    pub data: Vec<f64>,
}
