use std::f64;

use bon::Builder;
use kurbo::{Point, Stroke};
use parley::Alignment;
use peniko::Brush;

use crate::{
    chart::{ChartHelper, Theme},
    primitives::Primitives,
    series::Series,
    utils::{get_raw_range, get_scale_details},
};

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

pub(crate) enum SingleCartesianAxis<'a> {
    Category(&'a CategoryAxis),
    Value((&'a ValueAxis, &'a ValueAxisMeta)),
}

impl<'a> From<&'a CategoryAxis> for SingleCartesianAxis<'a> {
    fn from(value: &'a CategoryAxis) -> Self {
        SingleCartesianAxis::Category(value)
    }
}

impl<'a> From<(&'a ValueAxis, &'a ValueAxisMeta)> for SingleCartesianAxis<'a> {
    fn from(value: (&'a ValueAxis, &'a ValueAxisMeta)) -> Self {
        SingleCartesianAxis::Value(value)
    }
}

#[derive(Debug, Builder, Clone, Default)]
pub struct CategoryAxis {
    #[builder(setters(option_fn(vis = "")))]
    pub axis_show: Option<bool>,
    #[builder(setters(option_fn(vis = "")))]
    pub axis_stroke: Option<Stroke>,
    #[builder(setters(option_fn(vis = "")))]
    pub axis_color: Option<Brush>,
    #[builder(setters(option_fn(vis = "")))]
    pub axis_position: Option<AxisPosition>,
    #[builder(setters(option_fn(vis = "")))]
    pub axis_offset: Option<f64>,
    #[builder(setters(option_fn(vis = "")))]
    pub axis_auto_offset: Option<f64>,
    #[builder(setters(option_fn(vis = "")))]
    pub ticks_show: Option<bool>,
    #[builder(setters(option_fn(vis = "")))]
    pub ticks_length: Option<f64>,
    #[builder(setters(option_fn(vis = "")))]
    pub ticks_stroke: Option<Stroke>,
    #[builder(setters(option_fn(vis = "")))]
    pub ticks_color: Option<Brush>,
    #[builder(setters(option_fn(vis = "")))]
    pub split_lines_show: Option<bool>,
    #[builder(setters(option_fn(vis = "")))]
    pub split_lines_color: Option<Brush>,
    #[builder(setters(option_fn(vis = "")))]
    pub split_lines_stroke: Option<Stroke>,
    #[builder(setters(option_fn(vis = "")))]
    pub labels_show: Option<bool>,
    #[builder(setters(option_fn(vis = "")))]
    pub labels_margin: Option<f64>,
    #[builder(setters(option_fn(vis = "")))]
    pub labels_color: Option<Brush>,
    #[builder(setters(option_fn(vis = "")))]
    pub labels_font_size: Option<f64>,
    #[builder(setters(option_fn(vis = "")))]
    pub labels_alignment: Option<Alignment>,
    #[builder(setters(option_fn(vis = "")))]
    pub labels_rotation: Option<f64>,
    pub data: Vec<String>,
}

#[derive(Debug, Builder, Clone, Default)]
pub struct ValueAxis {
    #[builder(setters(option_fn(vis = "")))]
    pub axis_show: Option<bool>,
    #[builder(setters(option_fn(vis = "")))]
    pub axis_stroke: Option<Stroke>,
    #[builder(setters(option_fn(vis = "")))]
    pub axis_color: Option<Brush>,
    #[builder(setters(option_fn(vis = "")))]
    pub axis_position: Option<AxisPosition>,
    #[builder(setters(option_fn(vis = "")))]
    pub axis_offset: Option<f64>,
    #[builder(setters(option_fn(vis = "")))]
    pub axis_auto_offset: Option<f64>,
    #[builder(setters(option_fn(vis = "")))]
    pub ticks_show: Option<bool>,
    #[builder(setters(option_fn(vis = "")))]
    pub ticks_length: Option<f64>,
    #[builder(setters(option_fn(vis = "")))]
    pub ticks_stroke: Option<Stroke>,
    #[builder(setters(option_fn(vis = "")))]
    pub ticks_color: Option<Brush>,
    #[builder(setters(option_fn(vis = "")))]
    pub split_lines_show: Option<bool>,
    #[builder(setters(option_fn(vis = "")))]
    pub split_lines_color: Option<Brush>,
    #[builder(setters(option_fn(vis = "")))]
    pub split_lines_stroke: Option<Stroke>,
    #[builder(setters(option_fn(vis = "")))]
    pub labels_show: Option<bool>,
    #[builder(setters(option_fn(vis = "")))]
    pub labels_margin: Option<f64>,
    #[builder(setters(option_fn(vis = "")))]
    pub labels_color: Option<Brush>,
    #[builder(setters(option_fn(vis = "")))]
    pub labels_font_size: Option<f64>,
    #[builder(setters(option_fn(vis = "")))]
    pub labels_alignment: Option<Alignment>,
    #[builder(setters(option_fn(vis = "")))]
    pub labels_rotation: Option<f64>,
}

#[derive(Debug, Clone)]
pub(crate) struct ValueAxisMeta {
    pub min: f64,
    pub max: f64,
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
    pub(crate) fn draw_axis(
        &'a self,
        index: usize,
        axis_type: &AxisType,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &ChartHelper,
        theme: &'a Theme,
    ) {
        self.draw_split_lines(axis_type, primitives, helper, theme);
        self.draw_axis_ticks(index, axis_type, primitives, helper, theme);
        self.draw_labels(index, axis_type, primitives, helper, theme);
        self.draw_axis_line(index, axis_type, primitives, helper, theme);
    }

    pub(crate) fn draw_axis_line(
        &'a self,
        index: usize,
        axis_type: &AxisType,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &ChartHelper,
        theme: &'a Theme,
    ) {
        if self
            .axis_show
            .unwrap_or(theme.cartesian_category_axis.axis_show)
        {
            let position = self.get_axis_position(index);
            let offset = self.get_axis_offset(index, position, theme);

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
                stroke: self
                    .axis_stroke
                    .as_ref()
                    .unwrap_or(&theme.cartesian_category_axis.axis_stroke),
                stroke_color: self
                    .axis_color
                    .as_ref()
                    .unwrap_or(&theme.cartesian_category_axis.axis_color),
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
        theme: &'a Theme,
    ) {
        if self
            .ticks_show
            .unwrap_or(theme.cartesian_category_axis.ticks_show)
        {
            let position = self.get_axis_position(index);
            let offset = self.get_axis_offset(index, position, theme);

            let tick_spacing = match axis_type {
                AxisType::XAxis => helper.offsets.x_span / self.data.len() as f64,
                AxisType::YAxis => helper.offsets.y_span / self.data.len() as f64,
            };
            let ticks_length = self
                .ticks_length
                .unwrap_or(theme.cartesian_category_axis.ticks_length);
            for tick_index in 0..=self.data.len() {
                let (start_point, end_point) = match axis_type {
                    AxisType::XAxis => {
                        let common_x =
                            helper.offsets.x_axis_start + tick_index as f64 * tick_spacing;
                        let (start_y, end_y) = match position {
                            AxisPosition::Start => (
                                helper.offsets.y_axis_start + offset,
                                helper.offsets.y_axis_start + ticks_length + offset,
                            ),
                            AxisPosition::End => (
                                helper.offsets.y_axis_end + offset,
                                helper.offsets.y_axis_end - ticks_length + offset,
                            ),
                        };
                        (Point::new(common_x, start_y), Point::new(common_x, end_y))
                    }
                    AxisType::YAxis => {
                        let (start_x, end_x) = match position {
                            AxisPosition::Start => (
                                helper.offsets.x_axis_start + offset,
                                helper.offsets.x_axis_start + offset - ticks_length,
                            ),
                            AxisPosition::End => (
                                helper.offsets.x_axis_end + offset,
                                helper.offsets.x_axis_end + offset + ticks_length,
                            ),
                        };

                        let common_y =
                            helper.offsets.y_axis_start - tick_index as f64 * tick_spacing;
                        (Point::new(start_x, common_y), Point::new(end_x, common_y))
                    }
                };
                let line = crate::primitives::Line {
                    stroke: self
                        .ticks_stroke
                        .as_ref()
                        .unwrap_or(&theme.cartesian_category_axis.ticks_stroke),
                    stroke_color: self
                        .ticks_color
                        .as_ref()
                        .unwrap_or(&theme.cartesian_category_axis.ticks_color),
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
        theme: &'a Theme,
    ) {
        if self
            .split_lines_show
            .unwrap_or(theme.cartesian_category_axis.split_lines_show)
        {
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
                    stroke: self
                        .split_lines_stroke
                        .as_ref()
                        .unwrap_or(&theme.cartesian_category_axis.split_lines_stroke),
                    stroke_color: self
                        .split_lines_color
                        .as_ref()
                        .unwrap_or(&theme.cartesian_category_axis.split_lines_color),
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
        theme: &'a Theme,
    ) {
        if self
            .labels_show
            .unwrap_or(theme.cartesian_category_axis.labels_show)
        {
            let position = self.get_axis_position(index);
            let offset = self.get_axis_offset(index, position, theme);

            let label_spacing = match axis_type {
                AxisType::XAxis => helper.offsets.x_span / self.data.len() as f64,
                AxisType::YAxis => helper.offsets.y_span / self.data.len() as f64,
            };
            let labels_margin = self
                .labels_margin
                .unwrap_or(theme.cartesian_category_axis.labels_margin);

            for (label_index, label) in self.data.iter().enumerate() {
                let point = match axis_type {
                    AxisType::XAxis => {
                        let pos_y = match position {
                            AxisPosition::Start => {
                                helper.offsets.y_axis_start
                                    + labels_margin
                                    + offset
                                    + self
                                        .labels_font_size
                                        .unwrap_or(theme.cartesian_value_axis.labels_font_size)
                                        / 2.0
                            }
                            AxisPosition::End => {
                                helper.offsets.y_axis_end - labels_margin + offset
                                    - self
                                        .labels_font_size
                                        .unwrap_or(theme.cartesian_value_axis.labels_font_size)
                                        / 2.0
                            }
                        };

                        let pos_x = helper.offsets.x_axis_start
                            + (label_index as f64 + 0.5) * label_spacing;
                        Point::new(pos_x, pos_y)
                    }
                    AxisType::YAxis => {
                        let pos_x = match position {
                            AxisPosition::Start => {
                                helper.offsets.x_axis_start - labels_margin + offset
                            }
                            AxisPosition::End => helper.offsets.x_axis_end + labels_margin + offset,
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
                    fill_color: self
                        .labels_color
                        .as_ref()
                        .unwrap_or(&theme.cartesian_category_axis.labels_color),
                    font_size: self
                        .labels_font_size
                        .unwrap_or(theme.cartesian_category_axis.labels_font_size),
                    text_anchor,
                    coord: point,
                    rotation: self.labels_rotation,
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

    fn get_axis_offset(&self, index: usize, position: &AxisPosition, theme: &Theme) -> f64 {
        let axis_auto_offset = self
            .axis_auto_offset
            .as_ref()
            .unwrap_or(&theme.cartesian_category_axis.axis_auto_offset);
        match position {
            AxisPosition::Start => *self
                .axis_offset
                .as_ref()
                .unwrap_or(&((index / 2) as f64 * axis_auto_offset)),
            AxisPosition::End => -*self
                .axis_offset
                .as_ref()
                .unwrap_or(&((index / 2) as f64 * axis_auto_offset)),
        }
    }
}

impl<'a> ValueAxis {
    pub(crate) fn draw_axis(
        &'a self,
        index: usize,
        axis_type: &AxisType,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &ChartHelper,
        theme: &'a Theme,
        mut filtered_series: impl Iterator<Item = &'a Series>,
        primary: bool,
    ) -> ValueAxisMeta {
        let (mut min, mut max) = match filtered_series.next() {
            Some(s) => match s {
                Series::Line(line) => {
                    let data_index = match primary {
                        true => line.data.primary_data_index.unwrap_or(0),
                        false => line.data.secondary_data_index.unwrap_or(1),
                    };
                    get_raw_range(&line.data.data[data_index])
                }
                Series::Scatter(scatter) => {
                    let data_index = match primary {
                        true => scatter.data.primary_data_index.unwrap_or(0),
                        false => scatter.data.secondary_data_index.unwrap_or(1),
                    };
                    get_raw_range(&scatter.data.data[data_index])
                }
            },
            None => unreachable!(),
        };

        for series in filtered_series {
            let (s_min, s_max) = match series {
                Series::Line(line) => {
                    let data_index = match primary {
                        true => line.data.primary_data_index.unwrap_or(0),
                        false => line.data.secondary_data_index.unwrap_or(1),
                    };
                    get_raw_range(&line.data.data[data_index])
                }

                Series::Scatter(scatter) => {
                    let data_index = match primary {
                        true => scatter.data.primary_data_index.unwrap_or(0),
                        false => scatter.data.secondary_data_index.unwrap_or(1),
                    };
                    get_raw_range(&scatter.data.data[data_index])
                }
            };
            min = min.min(s_min);
            max = max.max(s_max);
        }

        let (min, max, step_size) = get_scale_details(min, max);

        self.draw_split_lines(axis_type, primitives, helper, theme, min, max, step_size);
        self.draw_axis_ticks(
            index, axis_type, primitives, helper, theme, min, max, step_size,
        );
        self.draw_labels(
            index, axis_type, primitives, helper, theme, min, max, step_size,
        );
        self.draw_axis_line(index, &AxisType::YAxis, primitives, helper, theme);
        ValueAxisMeta { min, max }
    }

    pub(crate) fn draw_axis_line(
        &'a self,
        index: usize,
        axis_type: &AxisType,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &ChartHelper,
        theme: &'a Theme,
    ) {
        if self
            .axis_show
            .unwrap_or(theme.cartesian_value_axis.axis_show)
        {
            let position = self.get_axis_position(index);
            let offset = self.get_axis_offset(index, position, theme);

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
                stroke: self
                    .axis_stroke
                    .as_ref()
                    .unwrap_or(&theme.cartesian_value_axis.axis_stroke),
                stroke_color: self
                    .axis_color
                    .as_ref()
                    .unwrap_or(&theme.cartesian_value_axis.axis_color),
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
        theme: &'a Theme,
        min: f64,
        max: f64,
        step_size: f64,
    ) {
        if self
            .ticks_show
            .unwrap_or(theme.cartesian_value_axis.ticks_show)
        {
            let position = self.get_axis_position(index);
            let offset = self.get_axis_offset(index, position, theme);

            let tick_spacing = match axis_type {
                AxisType::XAxis => helper.offsets.x_span / ((max - min) / step_size),
                AxisType::YAxis => helper.offsets.y_span / ((max - min) / step_size),
            };
            let ticks_length = self
                .ticks_length
                .unwrap_or(theme.cartesian_value_axis.ticks_length);
            for tick_index in 0..(((max - min) / step_size) as i32 + 1) {
                let (start_point, end_point) = match axis_type {
                    AxisType::XAxis => {
                        let (start_y, end_y) = match position {
                            AxisPosition::Start => (
                                helper.offsets.y_axis_start + offset,
                                helper.offsets.y_axis_start + offset + ticks_length,
                            ),

                            AxisPosition::End => (
                                helper.offsets.y_axis_end + offset,
                                helper.offsets.y_axis_end + offset - ticks_length,
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
                                helper.offsets.x_axis_start + offset - ticks_length,
                            ),
                            AxisPosition::End => (
                                helper.offsets.x_axis_end + offset,
                                helper.offsets.x_axis_end + offset + ticks_length,
                            ),
                        };

                        let common_y =
                            helper.offsets.y_axis_start - (tick_index as f64 * tick_spacing);
                        (Point::new(start_x, common_y), Point::new(end_x, common_y))
                    }
                };
                let line = crate::primitives::Line {
                    stroke: self
                        .ticks_stroke
                        .as_ref()
                        .unwrap_or(&theme.cartesian_value_axis.ticks_stroke),
                    stroke_color: self
                        .ticks_color
                        .as_ref()
                        .unwrap_or(&theme.cartesian_value_axis.ticks_color),
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
        theme: &'a Theme,
        min: f64,
        max: f64,
        step_size: f64,
    ) {
        if self
            .split_lines_show
            .unwrap_or(theme.cartesian_value_axis.split_lines_show)
        {
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
                    stroke: self
                        .split_lines_stroke
                        .as_ref()
                        .unwrap_or(&theme.cartesian_value_axis.split_lines_stroke),
                    stroke_color: self
                        .split_lines_color
                        .as_ref()
                        .unwrap_or(&theme.cartesian_value_axis.split_lines_color),
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
        theme: &'a Theme,
        min: f64,
        max: f64,
        step_size: f64,
    ) {
        if self
            .labels_show
            .unwrap_or(theme.cartesian_value_axis.labels_show)
        {
            let position = self.get_axis_position(index);
            let offset = self.get_axis_offset(index, position, theme);

            let label_spacing = match axis_type {
                AxisType::XAxis => helper.offsets.x_span / ((max - min) / step_size),
                AxisType::YAxis => helper.offsets.y_span / ((max - min) / step_size),
            };
            let labels_margin = self
                .labels_margin
                .unwrap_or(theme.cartesian_value_axis.labels_margin);
            for label_index in 0..(((max - min) / step_size) as i32 + 1) {
                let point = match axis_type {
                    AxisType::XAxis => {
                        let pos_y = match position {
                            AxisPosition::Start => {
                                helper.offsets.y_axis_start
                                    + labels_margin
                                    + offset
                                    + self
                                        .labels_font_size
                                        .unwrap_or(theme.cartesian_value_axis.labels_font_size)
                                        / 2.0
                            }
                            AxisPosition::End => {
                                helper.offsets.y_axis_end - labels_margin + offset
                                    - self
                                        .labels_font_size
                                        .unwrap_or(theme.cartesian_value_axis.labels_font_size)
                                        / 2.0
                            }
                        };

                        let pos_x =
                            helper.offsets.x_axis_start + (label_index as f64 * label_spacing);
                        Point::new(pos_x, pos_y)
                    }
                    AxisType::YAxis => {
                        let pos_x = match position {
                            AxisPosition::Start => {
                                helper.offsets.x_axis_start - labels_margin + offset
                            }
                            AxisPosition::End => helper.offsets.x_axis_end + labels_margin + offset,
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
                    fill_color: self
                        .labels_color
                        .as_ref()
                        .unwrap_or(&theme.cartesian_value_axis.labels_color),
                    font_size: self
                        .labels_font_size
                        .unwrap_or(theme.cartesian_value_axis.labels_font_size),
                    text_anchor,
                    coord: point,
                    rotation: self.labels_rotation,
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

    fn get_axis_offset(&self, index: usize, position: &AxisPosition, theme: &Theme) -> f64 {
        let axis_auto_offset = self
            .axis_auto_offset
            .as_ref()
            .unwrap_or(&theme.cartesian_value_axis.axis_auto_offset);
        match position {
            AxisPosition::Start => *self
                .axis_offset
                .as_ref()
                .unwrap_or(&((index / 2) as f64 * axis_auto_offset)),
            AxisPosition::End => -*self
                .axis_offset
                .as_ref()
                .unwrap_or(&((index / 2) as f64 * axis_auto_offset)),
        }
    }
}
