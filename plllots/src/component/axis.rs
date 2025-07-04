use std::f64;

use bon::Builder;
use kurbo::{Cap, Point, Stroke};
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
    #[builder(default = Stroke::new(1.0).with_start_cap(Cap::Square).with_end_cap(Cap::Square))]
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
    #[builder(default = Stroke::new(1.0).with_start_cap(Cap::Square).with_end_cap(Cap::Square))]
    pub ticks_stroke: Stroke,
    #[builder(default = false)]
    pub split_lines_show: bool,
    #[builder(default = Brush::Solid(Color::from_rgba8(0xe0, 0xe6, 0xe1, 0xff)))]
    pub split_lines_color: Brush,
    #[builder(default = Stroke::new(1.0).with_start_cap(Cap::Square).with_end_cap(Cap::Square))]
    pub split_lines_stroke: Stroke,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub ticks_color: Brush,
    #[builder(default = true)]
    pub labels_show: bool,
    #[builder(default = 14.0)]
    pub labels_margin: f64,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub labels_color: Brush,
    #[builder(default = 12.0)]
    pub labels_font_size: f64,
    pub labels_alignment: Option<Alignment>,
    pub data: Vec<String>,
}

#[derive(Debug, Builder, Clone)]
pub struct ValueAxis {
    #[builder(default = false)]
    pub axis_show: bool,
    #[builder(default = Stroke::new(1.0).with_start_cap(Cap::Square).with_end_cap(Cap::Square))]
    pub axis_stroke: Stroke,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub axis_color: Brush,
    pub axis_position: Option<AxisPosition>,
    pub axis_offset: Option<f64>,
    #[builder(default = 20.0)]
    pub axis_auto_offset: f64,
    #[builder(default = false)]
    pub ticks_show: bool,
    #[builder(default = 5.0)]
    pub ticks_length: f64,
    #[builder(default = Stroke::new(1.0).with_start_cap(Cap::Square).with_end_cap(Cap::Square))]
    pub ticks_stroke: Stroke,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub ticks_color: Brush,
    #[builder(default = true)]
    pub split_lines_show: bool,
    #[builder(default = Brush::Solid(Color::from_rgba8(0xe0, 0xe6, 0xe1, 0xff)))]
    pub split_lines_color: Brush,
    #[builder(default = Stroke::new(1.0).with_start_cap(Cap::Square).with_end_cap(Cap::Square))]
    pub split_lines_stroke: Stroke,
    #[builder(default = true)]
    pub labels_show: bool,
    #[builder(default = 8.0)]
    pub labels_margin: f64,
    #[builder(default = Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub labels_color: Brush,
    #[builder(default = 12.0)]
    pub labels_font_size: f64,
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

impl<'a> CategoryAxis {
    pub(crate) fn draw_axis_line(
        &'a self,
        index: usize,
        axis_type: &AxisType,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &ChartHelper,
    ) {
        if self.axis_show {
            let position = self.get_axis_position(index);
            let offset = self.get_axis_offset(index, position);

            let (start_point, end_point) = match axis_type {
                AxisType::XAxis => match position {
                    AxisPosition::Start => (
                        Point::new(
                            helper.offsets.x_axis_start,
                            helper.offsets.y_axis_start + offset,
                        ),
                        Point::new(
                            helper.offsets.x_axis_end,
                            helper.offsets.y_axis_start + offset,
                        ),
                    ),
                    AxisPosition::End => (
                        Point::new(
                            helper.offsets.x_axis_start,
                            helper.offsets.y_axis_end + offset,
                        ),
                        Point::new(
                            helper.offsets.x_axis_end,
                            helper.offsets.y_axis_end + offset,
                        ),
                    ),
                },
                AxisType::YAxis => match position {
                    AxisPosition::Start => (
                        Point::new(
                            helper.offsets.x_axis_start + offset,
                            helper.offsets.y_axis_end,
                        ),
                        Point::new(
                            helper.offsets.x_axis_start + offset,
                            helper.offsets.y_axis_start,
                        ),
                    ),
                    AxisPosition::End => (
                        Point::new(
                            helper.offsets.x_axis_end + offset,
                            helper.offsets.y_axis_end,
                        ),
                        Point::new(
                            helper.offsets.x_axis_end + offset,
                            helper.offsets.y_axis_start,
                        ),
                    ),
                },
            };
            let line = crate::primitives::Line {
                stroke: &self.axis_stroke,
                stroke_color: &self.axis_color,
                coords: (start_point, end_point),
            };
            primitives.push(crate::primitives::Primitives::Line(line));
        }
    }
    pub(crate) fn draw_axis_ticks(
        &'a self,
        index: usize,
        axis_type: &AxisType,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &ChartHelper,
    ) {
        if self.ticks_show {
            let position = self.get_axis_position(index);
            let offset = self.get_axis_offset(index, position);

            let tick_spacing = match axis_type {
                AxisType::XAxis => helper.offsets.x_span / self.data.len() as f64,
                AxisType::YAxis => helper.offsets.y_span / self.data.len() as f64,
            };
            for tick_index in 0..=self.data.len() {
                let (start_point, end_point) = match axis_type {
                    AxisType::XAxis => {
                        let common_x =
                            helper.offsets.x_axis_start + tick_index as f64 * tick_spacing;
                        let (start_y, end_y) = match position {
                            AxisPosition::Start => (
                                helper.offsets.y_axis_start + offset,
                                helper.offsets.y_axis_start + self.ticks_length + offset,
                            ),
                            AxisPosition::End => (
                                helper.offsets.y_axis_end + offset,
                                helper.offsets.y_axis_end - self.ticks_length + offset,
                            ),
                        };
                        (Point::new(common_x, start_y), Point::new(common_x, end_y))
                    }
                    AxisType::YAxis => {
                        let (start_x, end_x) = match position {
                            AxisPosition::Start => (
                                helper.offsets.x_axis_start + offset,
                                helper.offsets.x_axis_start + offset - self.ticks_length,
                            ),
                            AxisPosition::End => (
                                helper.offsets.x_axis_end + offset,
                                helper.offsets.x_axis_end + offset + self.ticks_length,
                            ),
                        };

                        let common_y =
                            helper.offsets.y_axis_start - tick_index as f64 * tick_spacing;
                        (Point::new(start_x, common_y), Point::new(end_x, common_y))
                    }
                };
                let line = crate::primitives::Line {
                    stroke: &self.ticks_stroke,
                    stroke_color: &self.ticks_color,
                    coords: (start_point, end_point),
                };
                primitives.push(crate::primitives::Primitives::Line(line));
            }
        }
    }

    pub(crate) fn draw_split_lines(
        &'a self,
        axis_type: &AxisType,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &ChartHelper,
    ) {
        if self.split_lines_show {
            let tick_spacing = match axis_type {
                AxisType::XAxis => helper.offsets.x_span / self.data.len() as f64,
                AxisType::YAxis => helper.offsets.y_span / self.data.len() as f64,
            };
            for tick_index in 0..=self.data.len() {
                let (start_point, end_point) = match axis_type {
                    AxisType::XAxis => {
                        let (start_y, end_y) =
                            (helper.offsets.y_axis_start, helper.offsets.y_axis_end);

                        let common_x =
                            helper.offsets.y_axis_start - (tick_index as f64 * tick_spacing);
                        (Point::new(common_x, start_y), Point::new(common_x, end_y))
                    }
                    AxisType::YAxis => {
                        let (start_x, end_x) =
                            (helper.offsets.x_axis_start, helper.offsets.x_axis_end);

                        let common_y =
                            helper.offsets.y_axis_start - (tick_index as f64 * tick_spacing);
                        (Point::new(start_x, common_y), Point::new(end_x, common_y))
                    }
                };
                let line = crate::primitives::Line {
                    stroke: &self.split_lines_stroke,
                    stroke_color: &self.split_lines_color,
                    coords: (start_point, end_point),
                };
                primitives.push(crate::primitives::Primitives::Line(line));
            }
        }
    }

    pub(crate) fn draw_labels(
        &'a self,
        index: usize,
        axis_type: &AxisType,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &ChartHelper,
    ) {
        if self.labels_show {
            let position = self.get_axis_position(index);
            let offset = self.get_axis_offset(index, position);

            let label_spacing = match axis_type {
                AxisType::XAxis => helper.offsets.x_span / self.data.len() as f64,
                AxisType::YAxis => helper.offsets.y_span / self.data.len() as f64,
            };
            for (label_index, label) in self.data.iter().enumerate() {
                let point = match axis_type {
                    AxisType::XAxis => {
                        let pos_y = match position {
                            AxisPosition::Start => {
                                helper.offsets.y_axis_start + self.labels_margin + offset
                            }
                            AxisPosition::End => {
                                helper.offsets.y_axis_end - self.labels_margin + offset
                            }
                        };

                        let pos_x = helper.offsets.x_axis_start
                            + (label_index as f64 + 0.5) * label_spacing;
                        Point::new(pos_x, pos_y)
                    }
                    AxisType::YAxis => {
                        let pos_x = match position {
                            AxisPosition::Start => {
                                helper.offsets.x_axis_start - self.labels_margin + offset
                            }
                            AxisPosition::End => {
                                helper.offsets.x_axis_end + self.labels_margin + offset
                            }
                        };
                        let pos_y = helper.offsets.y_axis_start
                            - (label_index as f64 + 0.5) * label_spacing;
                        Point::new(pos_x, pos_y)
                    }
                };

                let text_anchor = self.labels_alignment.unwrap_or(match axis_type {
                    AxisType::XAxis => Alignment::Middle,
                    AxisType::YAxis => match position {
                        AxisPosition::Start => Alignment::End,
                        AxisPosition::End => Alignment::Start,
                    },
                });

                let text = crate::primitives::Text {
                    text: label.to_string(),
                    fill_color: &self.labels_color,
                    font_size: self.labels_font_size,
                    text_anchor,
                    coord: point,
                };
                primitives.push(crate::primitives::Primitives::Text(text));
            }
        }
    }

    fn get_axis_position(&self, index: usize) -> &AxisPosition {
        self.axis_position.as_ref().unwrap_or({
            if index % 2 == 0 {
                &AxisPosition::Start
            } else {
                &AxisPosition::End
            }
        })
    }

    fn get_axis_offset(&self, index: usize, position: &AxisPosition) -> f64 {
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
}

impl<'a> ValueAxis {
    pub(crate) fn draw_axis_line(
        &'a self,
        index: usize,
        axis_type: &AxisType,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &ChartHelper,
    ) {
        if self.axis_show {
            let position = self.get_axis_position(index);
            let offset = self.get_axis_offset(index, position);

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
                stroke: &self.axis_stroke,
                stroke_color: &self.axis_color,
                coords: (start_point, end_point),
            };
            primitives.push(crate::primitives::Primitives::Line(line));
        }
    }

    pub(crate) fn draw_axis_ticks(
        &'a self,
        index: usize,
        axis_type: &AxisType,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &ChartHelper,
        min: f64,
        max: f64,
        step_size: f64,
    ) {
        if self.ticks_show {
            let position = self.get_axis_position(index);
            let offset = self.get_axis_offset(index, position);

            let tick_spacing = match axis_type {
                AxisType::XAxis => helper.offsets.x_span / ((max - min) / step_size),
                AxisType::YAxis => helper.offsets.y_span / ((max - min) / step_size),
            };
            for tick_index in 0..(((max - min) / step_size) as i32 + 1) {
                let (start_point, end_point) = match axis_type {
                    AxisType::XAxis => {
                        let (start_y, end_y) = match position {
                            AxisPosition::Start => (
                                helper.offsets.y_axis_start + offset,
                                helper.offsets.y_axis_start + offset + self.ticks_length,
                            ),

                            AxisPosition::End => (
                                helper.offsets.y_axis_end + offset,
                                helper.offsets.y_axis_end + offset - self.ticks_length,
                            ),
                        };
                        let common_x =
                            helper.offsets.x_axis_start + (tick_index as f64 * tick_spacing);
                        (Point::new(common_x, start_y), Point::new(common_x, end_y))
                    }
                    AxisType::YAxis => {
                        let (start_x, end_x) = match position {
                            AxisPosition::Start => (
                                helper.offsets.x_axis_start + offset,
                                helper.offsets.x_axis_start + offset - self.ticks_length,
                            ),
                            AxisPosition::End => (
                                helper.offsets.x_axis_end + offset,
                                helper.offsets.x_axis_end + offset + self.ticks_length,
                            ),
                        };

                        let common_y =
                            helper.offsets.y_axis_start - (tick_index as f64 * tick_spacing);
                        (Point::new(start_x, common_y), Point::new(end_x, common_y))
                    }
                };
                let line = crate::primitives::Line {
                    stroke: &self.ticks_stroke,
                    stroke_color: &self.ticks_color,
                    coords: (start_point, end_point),
                };
                primitives.push(crate::primitives::Primitives::Line(line));
            }
        }
    }

    pub(crate) fn draw_split_lines(
        &'a self,
        axis_type: &AxisType,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &ChartHelper,
        min: f64,
        max: f64,
        step_size: f64,
    ) {
        if self.split_lines_show {
            let tick_spacing = match axis_type {
                AxisType::XAxis => helper.offsets.x_span / ((max - min) / step_size),
                AxisType::YAxis => helper.offsets.y_span / ((max - min) / step_size),
            };
            for tick_index in 0..(((max - min) / step_size) as i32 + 1) {
                let (start_point, end_point) = match axis_type {
                    AxisType::XAxis => {
                        let (start_y, end_y) =
                            (helper.offsets.y_axis_start, helper.offsets.y_axis_end);

                        let common_x =
                            helper.offsets.x_axis_start + (tick_index as f64 * tick_spacing);
                        (Point::new(common_x, start_y), Point::new(common_x, end_y))
                    }
                    AxisType::YAxis => {
                        let (start_x, end_x) =
                            (helper.offsets.x_axis_start, helper.offsets.x_axis_end);

                        let common_y =
                            helper.offsets.y_axis_start - (tick_index as f64 * tick_spacing);
                        (Point::new(start_x, common_y), Point::new(end_x, common_y))
                    }
                };
                let line = crate::primitives::Line {
                    stroke: &self.split_lines_stroke,
                    stroke_color: &self.split_lines_color,
                    coords: (start_point, end_point),
                };
                primitives.push(crate::primitives::Primitives::Line(line));
            }
        }
    }

    pub(crate) fn draw_labels(
        &'a self,
        index: usize,
        axis_type: &AxisType,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &ChartHelper,
        min: f64,
        max: f64,
        step_size: f64,
    ) {
        if self.labels_show {
            let position = self.get_axis_position(index);
            let offset = self.get_axis_offset(index, position);

            let label_spacing = match axis_type {
                AxisType::XAxis => helper.offsets.x_span / ((max - min) / step_size),
                AxisType::YAxis => helper.offsets.y_span / ((max - min) / step_size),
            };
            for label_index in 0..(((max - min) / step_size) as i32 + 1) {
                let point = match axis_type {
                    AxisType::XAxis => {
                        let pos_y = match position {
                            AxisPosition::Start => {
                                helper.offsets.y_axis_start + self.labels_margin + offset
                            }
                            AxisPosition::End => {
                                helper.offsets.y_axis_end - self.labels_margin + offset
                            }
                        };

                        let pos_x =
                            helper.offsets.x_axis_start + (label_index as f64 * label_spacing);
                        Point::new(pos_x, pos_y)
                    }
                    AxisType::YAxis => {
                        let pos_x = match position {
                            AxisPosition::Start => {
                                helper.offsets.x_axis_start - self.labels_margin + offset
                            }
                            AxisPosition::End => {
                                helper.offsets.x_axis_end + self.labels_margin + offset
                            }
                        };
                        let pos_y =
                            helper.offsets.y_axis_start - (label_index as f64 * label_spacing);
                        Point::new(pos_x, pos_y)
                    }
                };
                let text_anchor = self.labels_alignment.unwrap_or(match axis_type {
                    AxisType::XAxis => Alignment::Middle,
                    AxisType::YAxis => match position {
                        AxisPosition::Start => Alignment::End,
                        AxisPosition::End => Alignment::Start,
                    },
                });

                let text = crate::primitives::Text {
                    text: format!("{}", min + step_size * label_index as f64),
                    fill_color: &self.labels_color,
                    font_size: self.labels_font_size,
                    text_anchor,
                    coord: point,
                };
                primitives.push(crate::primitives::Primitives::Text(text));
            }
        }
    }

    fn get_axis_position(&self, index: usize) -> &AxisPosition {
        self.axis_position.as_ref().unwrap_or({
            if index % 2 == 0 {
                &AxisPosition::Start
            } else {
                &AxisPosition::End
            }
        })
    }

    fn get_axis_offset(&self, index: usize, position: &AxisPosition) -> f64 {
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
}
