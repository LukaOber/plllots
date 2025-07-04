use bon::Builder;
use kurbo::Stroke;
use parley::Alignment;
use peniko::{Brush, Color};

#[derive(Debug, Clone)]
pub enum CartesianAxis {
    Category(Vec<CategoryAxis>),
    Value(Vec<ValueAxis>),
}

impl From<CategoryAxis> for CartesianAxis {
    fn from(value: CategoryAxis) -> Self {
        Self::Category(vec![value])
    }
}

impl From<ValueAxis> for CartesianAxis {
    fn from(value: ValueAxis) -> Self {
        Self::Value(vec![value])
    }
}

#[derive(Debug, Builder, Clone)]
pub struct CategoryAxis {
    #[builder(default = true, setters(option_fn(vis = "")))]
    pub axis_show: bool,
    #[builder(default = Stroke::new(1.0))]
    pub axis_stroke: Stroke,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub axis_color: Brush,
    pub axis_position: Option<AxisPosition>,
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
    // #[builder(default = Alignment::Middle)]
    pub labels_alignment: Option<Alignment>,
    #[builder(into)]
    pub data: Vec<String>,
}

#[derive(Debug, Builder, Clone)]
pub struct ValueAxis {
    #[builder(default = false)]
    pub axis_show: bool,
    #[builder(default = Stroke::new(1.0))]
    pub axis_stroke: Stroke,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub axis_color: Brush,
    pub axis_position: Option<AxisPosition>,
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
    // #[builder(default = Alignment::End)]
    pub labels_alignment: Option<Alignment>,
}

#[derive(Debug, Clone)]
pub enum AxisPosition {
    Start,
    End,
}

#[derive(Debug, Clone)]
pub(crate) enum AxisType {
    Start,
    End,
}
