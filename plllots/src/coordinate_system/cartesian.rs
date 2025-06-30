use bon::Builder;
use kurbo::Point;

use crate::{
    component::{CartesianAxis, XAxes, YAxes},
    primitives::AppendPrimitives,
    series::Series,
    utils::calculate_axis_ticks,
};

#[derive(Debug, Builder, Clone)]
pub struct Cartesian {
    #[builder(field)]
    pub series: Vec<Series>,
    #[builder(name = x_axis, into)]
    pub x_axis: XAxes,
    #[builder(name = y_axis, into)]
    pub y_axis: YAxes,
}

impl<S: cartesian_builder::State> CartesianBuilder<S> {
    pub fn add_series(mut self, series: impl Into<Series>) -> Self {
        self.series.push(series.into());
        self
    }

    pub fn set_series(mut self, series: impl IntoIterator<Item: Into<Series>>) -> Self {
        self.series = series.into_iter().map(Into::into).collect();
        self
    }

    // pub fn add_y_axis(mut self, y_axis: impl Into<YAxis>) -> Self {
    //     self.y_axis.push(y_axis.into());
    //     self
    // }

    // pub fn set_y_axis(mut self, y_axis: impl IntoIterator<Item: Into<YAxis>>) -> Self {
    //     self.y_axis = y_axis.into_iter().map(Into::into).collect();
    //     self
    // }

    // pub fn add_x_axis(mut self, x_axis: impl Into<XAxis>) -> Self {
    //     self.x_axis.push(x_axis.into());
    //     self
    // }

    // pub fn set_x_axis(mut self, x_axis: impl IntoIterator<Item: Into<XAxis>>) -> Self {
    //     self.x_axis = x_axis.into_iter().map(Into::into).collect();
    //     self
    // }
}

impl<'a> AppendPrimitives<'a> for Cartesian {
    fn append_primitives(
        &'a self,
        primitives: &mut Vec<crate::primitives::Primitives<'a>>,
        helper: &mut crate::chart::ChartHelper,
    ) {
        let all_axes = match (&self.x_axis, &self.y_axis) {
            (XAxes::Single(x_axis), YAxes::Single(y_axis)) => vec![(x_axis, y_axis, 0, 0)],
            (XAxes::Single(x_axis), YAxes::Multiple(items)) => items
                .into_iter()
                .enumerate()
                .map(|(i, y_axis)| (x_axis, y_axis, 0, i))
                .collect(),
            (XAxes::Multiple(items), YAxes::Single(y_axis)) => items
                .into_iter()
                .enumerate()
                .map(|(i, x_axis)| (x_axis, y_axis, i, 0))
                .collect(),
            (XAxes::Multiple(_x_items), YAxes::Multiple(_y_items)) => panic!("unsupported"),
        };

        for (x_axis, y_axis, x_axis_index, y_axis_index) in all_axes {
            match (&x_axis.axis_type, &y_axis.axis_type) {
                (CartesianAxis::Category(_x_items), CartesianAxis::Category(_y_items)) => {
                    todo!()
                }
                (CartesianAxis::Category(x_items), CartesianAxis::Values) => {
                    // X-Axis
                    if x_axis.axis_show {
                        let line = crate::primitives::Line {
                            stroke: &x_axis.axis_stroke,
                            stroke_color: &x_axis.axis_color,
                            coords: (
                                Point::new(helper.offsets.x_axis_start, helper.offsets.y_axis_end),
                                Point::new(helper.offsets.x_axis_end, helper.offsets.y_axis_end),
                            ),
                        };
                        primitives.push(crate::primitives::Primitives::Line(line));
                    }

                    // X-Axis ticks
                    if x_axis.ticks_show {
                        let label_spacing = helper.offsets.x_span / x_items.len() as f64;
                        for label_index in 0..=x_items.len() {
                            let x_pos =
                                helper.offsets.x_axis_start + label_index as f64 * label_spacing;
                            let y_pos = helper.offsets.y_axis_end + x_axis.ticks_length;
                            let line = crate::primitives::Line {
                                stroke: &x_axis.ticks_stroke,
                                stroke_color: &x_axis.ticks_color,
                                coords: (
                                    Point::new(x_pos, helper.offsets.y_axis_end),
                                    Point::new(x_pos, y_pos),
                                ),
                            };
                            primitives.push(crate::primitives::Primitives::Line(line));
                        }
                    }
                    // X-Axis labels
                    if x_axis.labels_show {
                        for (label_index, label) in x_items.iter().enumerate() {
                            let label_spacing = helper.offsets.x_span / x_items.len() as f64;
                            let x_pos = helper.offsets.x_axis_start
                                + (label_index as f64 + 0.5) * label_spacing;
                            let y_pos = helper.offsets.y_axis_end + x_axis.labels_margin;
                            let text = crate::primitives::Text {
                                text: label.to_string(),
                                fill_color: &x_axis.labels_color,
                                font_size: 12.0,
                                text_anchor: parley::Alignment::Middle,
                                coord: Point::new(x_pos, y_pos),
                            };
                            primitives.push(crate::primitives::Primitives::Text(text));
                        }
                    }

                    let data = match self.series[0] {
                        Series::Line(ref line) => &line.data,
                    };
                    let (min, max, step_size) = calculate_axis_ticks(data);
                    let sub_tick_spacing = helper.offsets.y_span / (max / step_size);

                    // Y-Axis
                    if y_axis.axis_show {
                        let line = crate::primitives::Line {
                            stroke: &y_axis.axis_stroke,
                            stroke_color: &y_axis.axis_color,
                            coords: (
                                Point::new(
                                    helper.offsets.x_axis_start,
                                    helper.offsets.y_axis_start,
                                ),
                                Point::new(helper.offsets.x_axis_start, helper.offsets.y_axis_end),
                            ),
                        };
                        primitives.push(crate::primitives::Primitives::Line(line));
                    }

                    // Y-Axis grid lines
                    if y_axis.grid_show {
                        for sub_tick_index in 1..((max / step_size) as i32 + 1) {
                            let sub_tick_height = helper.offsets.y_axis_end
                                - (sub_tick_index as f64 * sub_tick_spacing);

                            let line = crate::primitives::Line {
                                stroke: &y_axis.grid_stroke,
                                stroke_color: &y_axis.grid_color,
                                coords: (
                                    Point::new(helper.offsets.x_axis_start, sub_tick_height),
                                    Point::new(helper.offsets.x_axis_end, sub_tick_height),
                                ),
                            };

                            primitives.push(crate::primitives::Primitives::Line(line));
                        }
                    }

                    // Y-Axis labels
                    if y_axis.labels_show {
                        for sub_tick_index in 0..((max / step_size) as i32 + 1) {
                            let sub_tick_height = helper.offsets.y_axis_end
                                - (sub_tick_index as f64 * sub_tick_spacing);
                            let text = crate::primitives::Text {
                                text: format!("{}", min + step_size * sub_tick_index as f64),
                                fill_color: &y_axis.labels_color,
                                font_size: 12.0,
                                text_anchor: parley::Alignment::End,
                                coord: Point::new(
                                    helper.offsets.x_axis_start - y_axis.labels_margin,
                                    sub_tick_height,
                                ),
                            };
                            primitives.push(crate::primitives::Primitives::Text(text));
                        }
                    }

                    match &self.series[0] {
                        Series::Line(line) => {
                            let mut path = crate::primitives::Path {
                                // TODO change stroke types
                                stroke: &line.stroke,
                                stroke_color: &line.color,
                                coords: Vec::with_capacity(line.data.len()),
                            };
                            for (index, y_item) in data.iter().enumerate() {
                                let y_pos = {
                                    let percentage_height = y_item / max;
                                    helper.offsets.y_axis_end
                                        - (percentage_height * helper.offsets.y_span)
                                };
                                let x_spacing = helper.offsets.x_span / x_items.len() as f64;
                                let x_pos =
                                    helper.offsets.x_axis_start + (index as f64 + 0.5) * x_spacing;
                                path.coords.push(Point::new(x_pos, y_pos));
                            }
                            primitives.push(crate::primitives::Primitives::Path(path));
                        }
                    }
                }
                (CartesianAxis::Values, CartesianAxis::Category(_y_items)) => {
                    todo!()
                }
                (CartesianAxis::Values, CartesianAxis::Values) => {
                    todo!()
                }
            }
        }
    }
}
