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
#[builder(finish_fn(vis = "", name = build_internal), derive(Debug))]
pub struct XAxis {
    #[builder(default = CartesianAxisLine { show:true, stroke: Stroke::new(1.0), color: Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)) })]
    pub axis_line: CartesianAxisLine,
    #[builder(default = CartesianAxisLabels { show: true, margin: 8.0, color: Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)), alignment: Alignment::Middle })]
    pub axis_labels: CartesianAxisLabels,
    #[builder(default = CartesianAxisTicks { show: true, length: 5.0, stroke: Stroke::new(1.0), color: Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)) })]
    pub axis_ticks: CartesianAxisTicks,
    pub axis_type: CartesianAxis,
}

// impl<S: x_axis_builder::IsComplete> XAxisBuilder<S> {
//     pub fn build(self) -> XAxis {
//         // let mut x_axis = &self.build_internal();
//         // println!("{:#?}", x_axis);

//         todo!()
//     }
// }

#[derive(Debug, Builder, Clone)]
pub struct YAxis {
    #[builder(default = CartesianAxisLine { show: true, stroke: Stroke::new(1.0), color: Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)) })]
    pub axis_line: CartesianAxisLine,
    #[builder(default = CartesianAxisLabels { show: true, margin: 8.0, color: Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)), alignment: Alignment::End })]
    pub axis_labels: CartesianAxisLabels,
    #[builder(default = CartesianAxisTicks { show: false, length: 5.0, stroke: Stroke::new(1.0), color: Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)) })]
    pub axis_ticks: CartesianAxisTicks,
    pub axis_type: CartesianAxis,
}

#[derive(Debug, Builder, Clone, Default)]
pub struct CartesianAxisLine {
    pub show: bool,
    pub stroke: Stroke,
    pub color: Brush,
}

#[derive(Debug, Builder, Clone, Default)]
pub struct CartesianAxisTicks {
    #[builder(default = true)]
    pub show: bool,
    #[builder(default = 5.0)]
    pub length: f64,
    #[builder(default = Stroke::new(1.0))]
    pub stroke: Stroke,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub color: Brush,
}
#[derive(Debug, Builder, Clone, Default)]
pub struct CartesianAxisLabels {
    #[builder(default = true)]
    pub show: bool,
    #[builder(default = 8.0)]
    pub margin: f64,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub color: Brush,
    #[builder(default = Alignment::Middle)]
    pub alignment: Alignment,
}

#[derive(Debug, Builder, Clone, Default)]
pub struct CartesianXAxisLabels {
    #[builder(default = true)]
    pub show: bool,
    #[builder(default = 8.0)]
    pub margin: f64,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub color: Brush,
    #[builder(default = Alignment::Middle)]
    pub alignment: Alignment,
}

impl From<CartesianXAxisLabels> for CartesianAxisLabels {
    fn from(value: CartesianXAxisLabels) -> Self {
        Self {
            show: value.show,
            margin: value.margin,
            color: value.color,
            alignment: value.alignment,
        }
    }
}
