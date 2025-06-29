use bon::Builder;
use kurbo::Stroke;
use parley::Alignment;
use peniko::{Brush, Color};

#[derive(Debug, Clone)]
pub enum CartesianAxis {
    Category(Vec<String>),
    Values,
}

#[derive(Debug, Builder, Clone)]
pub struct XAxis {
    #[builder(default = true)]
    pub axis_show: bool,
    #[builder(default = Stroke::new(1.0))]
    pub axis_stroke: Stroke,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub axis_color: Brush,
    #[builder(default = true)]
    pub ticks_show: bool,
    #[builder(default = 5.0)]
    pub ticks_length: f64,
    #[builder(default = Stroke::new(1.0))]
    pub ticks_stroke: Stroke,
    #[builder(default = false)]
    pub grid_show: bool,
    #[builder(default = Brush::Solid(Color::from_rgba8(0xe0, 0xe6, 0xe1, 0xff)))]
    pub grid_color: Brush,
    #[builder(default = Stroke::new(1.0))]
    pub grid_stroke: Stroke,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub ticks_color: Brush,
    #[builder(default = true)]
    pub labels_show: bool,
    #[builder(default = 14.0)]
    pub labels_margin: f64,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub labels_color: Brush,
    #[builder(default = Alignment::Middle)]
    pub labels_alignment: Alignment,
    pub axis_type: CartesianAxis,
}

#[derive(Debug, Builder, Clone)]
pub struct YAxis {
    #[builder(default = false)]
    pub axis_show: bool,
    #[builder(default = Stroke::new(1.0))]
    pub axis_stroke: Stroke,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub axis_color: Brush,
    #[builder(default = true)]
    pub ticks_show: bool,
    #[builder(default = 5.0)]
    pub ticks_length: f64,
    #[builder(default = Stroke::new(1.0))]
    pub ticks_stroke: Stroke,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub ticks_color: Brush,
    #[builder(default = true)]
    pub grid_show: bool,
    #[builder(default = Brush::Solid(Color::from_rgba8(0xe0, 0xe6, 0xe1, 0xff)))]
    pub grid_color: Brush,
    #[builder(default = Stroke::new(1.0))]
    pub grid_stroke: Stroke,
    #[builder(default = true)]
    pub labels_show: bool,
    #[builder(default = 8.0)]
    pub labels_margin: f64,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub labels_color: Brush,
    #[builder(default = Alignment::Middle)]
    pub labels_alignment: Alignment,
    pub axis_type: CartesianAxis,
}
