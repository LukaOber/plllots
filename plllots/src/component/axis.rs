use bon::Builder;
use kurbo::{Point, Stroke};
use parley::Alignment;
use peniko::{Brush, Color};

use crate::{chart::ChartHelper, primitives::Primitives};

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

impl From<Vec<CategoryAxis>> for CartesianAxis {
    fn from(value: Vec<CategoryAxis>) -> Self {
        Self::Category(value)
    }
}

impl From<ValueAxis> for CartesianAxis {
    fn from(value: ValueAxis) -> Self {
        Self::Value(vec![value])
    }
}

impl From<Vec<ValueAxis>> for CartesianAxis {
    fn from(value: Vec<ValueAxis>) -> Self {
        Self::Value(value)
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
    pub axis_offset: Option<f64>,
    #[builder(default = 20.0)]
    pub axis_auto_offset: f64,
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
    pub axis_offset: Option<f64>,
    #[builder(default = 20.0)]
    pub axis_auto_offset: f64,
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
    XAxis,
    YAxis,
}

pub(crate) trait DrawCartesianAxis<'a> {
    fn draw_axis_line(
        &'a self,
        index: usize,
        axis_type: &AxisType,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &ChartHelper,
    );
    fn get_position(&self, index: usize) -> &AxisPosition;
    fn get_offset(&self, index: usize, position: &AxisPosition) -> f64;
    fn get_axis_stroke(&self) -> &Stroke;
    fn get_axis_color(&self) -> &Brush;
    fn get_axis_show(&self) -> bool;
}

impl<'a> DrawCartesianAxis<'a> for CategoryAxis {
    fn draw_axis_line(
        &'a self,
        index: usize,
        axis_type: &AxisType,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &ChartHelper,
    ) {
        if self.get_axis_show() {
            let position = self.get_position(index);
            let offset = self.get_offset(index, position);

            let (start_point, end_point) = match axis_type {
                AxisType::XAxis => match position {
                    AxisPosition::Start => (
                        Point::new(
                            helper.offsets.x_axis_start,
                            helper.offsets.y_axis_end + offset,
                        ),
                        Point::new(
                            helper.offsets.x_axis_end,
                            helper.offsets.y_axis_end + offset,
                        ),
                    ),
                    AxisPosition::End => (
                        Point::new(
                            helper.offsets.x_axis_start,
                            helper.offsets.y_axis_start + offset,
                        ),
                        Point::new(
                            helper.offsets.x_axis_end,
                            helper.offsets.y_axis_start + offset,
                        ),
                    ),
                },
                AxisType::YAxis => match position {
                    AxisPosition::Start => (
                        Point::new(
                            helper.offsets.x_axis_start + offset,
                            helper.offsets.y_axis_start,
                        ),
                        Point::new(
                            helper.offsets.x_axis_start + offset,
                            helper.offsets.y_axis_end,
                        ),
                    ),
                    AxisPosition::End => (
                        Point::new(
                            helper.offsets.x_axis_end + offset,
                            helper.offsets.y_axis_start,
                        ),
                        Point::new(
                            helper.offsets.x_axis_end + offset,
                            helper.offsets.y_axis_end,
                        ),
                    ),
                },
            };
            let line = crate::primitives::Line {
                stroke: &self.get_axis_stroke(),
                stroke_color: &self.get_axis_color(),
                coords: (start_point, end_point),
            };
            primitives.push(crate::primitives::Primitives::Line(line));
        }
    }

    fn get_position(&self, index: usize) -> &AxisPosition {
        self.axis_position.as_ref().unwrap_or_else(|| {
            if index % 2 == 0 {
                &AxisPosition::Start
            } else {
                &AxisPosition::End
            }
        })
    }

    fn get_offset(&self, index: usize, position: &AxisPosition) -> f64 {
        match position {
            AxisPosition::Start => *self
                .axis_offset
                .as_ref()
                .unwrap_or(&((index / 2) as f64 * self.axis_auto_offset)),
            AxisPosition::End => -*self
                .axis_offset
                .as_ref()
                .unwrap_or(&((index / 2) as f64 * self.axis_auto_offset)),
        }
    }

    fn get_axis_stroke(&self) -> &Stroke {
        &self.axis_stroke
    }

    fn get_axis_color(&self) -> &Brush {
        &self.axis_color
    }

    fn get_axis_show(&self) -> bool {
        self.axis_show
    }
}

impl<'a> DrawCartesianAxis<'a> for ValueAxis {
    fn draw_axis_line(
        &'a self,
        index: usize,
        axis_type: &AxisType,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &ChartHelper,
    ) {
        if self.get_axis_show() {
            let position = self.get_position(index);
            let offset = self.get_offset(index, position);

            let (start_point, end_point) = match axis_type {
                AxisType::XAxis => match position {
                    AxisPosition::Start => (
                        Point::new(
                            helper.offsets.x_axis_start,
                            helper.offsets.y_axis_end + offset,
                        ),
                        Point::new(
                            helper.offsets.x_axis_end,
                            helper.offsets.y_axis_end + offset,
                        ),
                    ),
                    AxisPosition::End => (
                        Point::new(
                            helper.offsets.x_axis_start,
                            helper.offsets.y_axis_start + offset,
                        ),
                        Point::new(
                            helper.offsets.x_axis_end,
                            helper.offsets.y_axis_start + offset,
                        ),
                    ),
                },
                AxisType::YAxis => match position {
                    AxisPosition::Start => (
                        Point::new(
                            helper.offsets.x_axis_start + offset,
                            helper.offsets.y_axis_start,
                        ),
                        Point::new(
                            helper.offsets.x_axis_start + offset,
                            helper.offsets.y_axis_end,
                        ),
                    ),
                    AxisPosition::End => (
                        Point::new(
                            helper.offsets.x_axis_end + offset,
                            helper.offsets.y_axis_start,
                        ),
                        Point::new(
                            helper.offsets.x_axis_end + offset,
                            helper.offsets.y_axis_end,
                        ),
                    ),
                },
            };
            let line = crate::primitives::Line {
                stroke: &self.get_axis_stroke(),
                stroke_color: &self.get_axis_color(),
                coords: (start_point, end_point),
            };
            primitives.push(crate::primitives::Primitives::Line(line));
        }
    }

    fn get_position(&self, index: usize) -> &AxisPosition {
        self.axis_position.as_ref().unwrap_or_else(|| {
            if index % 2 == 0 {
                &AxisPosition::Start
            } else {
                &AxisPosition::End
            }
        })
    }

    fn get_offset(&self, index: usize, position: &AxisPosition) -> f64 {
        match position {
            AxisPosition::Start => *self
                .axis_offset
                .as_ref()
                .unwrap_or(&((index / 2) as f64 * self.axis_auto_offset)),
            AxisPosition::End => -*self
                .axis_offset
                .as_ref()
                .unwrap_or(&((index / 2) as f64 * self.axis_auto_offset)),
        }
    }

    fn get_axis_stroke(&self) -> &Stroke {
        &self.axis_stroke
    }

    fn get_axis_color(&self) -> &Brush {
        &self.axis_color
    }

    fn get_axis_show(&self) -> bool {
        self.axis_show
    }
}
