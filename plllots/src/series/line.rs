use bon::Builder;
use kurbo::Stroke;
use peniko::{Brush, Color};

#[derive(Debug, Builder, Clone)]
pub struct Line {
    #[builder(default = Stroke::new(2.0))]
    pub stroke: Stroke,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x54, 0x70, 0xd6, 0xff)))]
    pub color: Brush,
    pub data: Vec<f64>,
}
