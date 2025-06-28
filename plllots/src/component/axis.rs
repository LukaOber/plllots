use bon::Builder;
use kurbo::Stroke;
use peniko::{Brush, Color};

#[derive(Debug, Clone)]
pub enum CartesianAxis {
    Category(Vec<String>),
    Values,
}

#[derive(Debug, Builder, Clone)]
pub struct XAxis {
    #[builder(default = Stroke::new(1.0))]
    pub stroke: Stroke,
    #[builder(default = peniko::Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub axis_color: Brush,
    #[builder(default = peniko::Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub label_color: Brush,
    pub cartesian_axis: CartesianAxis,
}

#[derive(Debug, Builder, Clone)]
pub struct YAxis {
    #[builder(default = Stroke::new(1.0))]
    pub stroke: Stroke,
    #[builder(default = peniko::Brush::Solid(Color::from_rgba8(0xe0, 0xe6, 0xf1, 0xff)))]
    pub axis_color: Brush,
    #[builder(default = peniko::Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub label_color: Brush,
    pub cartesian_axis: CartesianAxis,
}

#[derive(Debug, Clone)]
pub enum AxisHelper {
    Category(AxisCategoryHelper),
    Values(AxisValuesHelper),
}

#[derive(Debug, Clone)]
pub struct AxisCategoryHelper {
    pub amount: usize,
}

#[derive(Debug, Clone)]
pub struct AxisValuesHelper {
    pub min: f64,
    pub max: f64,
    pub step_size: f64,
}
