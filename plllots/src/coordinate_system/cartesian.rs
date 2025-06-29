use bon::Builder;
use kurbo::Point;

use crate::{
    component::{CartesianAxis, XAxis, YAxis},
    primitives::AppendPrimitives,
    utils::calculate_axis_ticks,
};

#[derive(Debug, Builder, Clone)]
pub struct Cartesian {
    pub x_axis: XAxis,
    pub y_axis: YAxis,
    pub data: Vec<f64>,
}

impl<'a> AppendPrimitives<'a> for Cartesian {
    fn append_primitives(
        &'a self,
        primitives: &mut Vec<crate::primitives::Primitives<'a>>,
        helper: &mut crate::chart::ChartHelper,
    ) {
        match (&self.x_axis.axis_type, &self.y_axis.axis_type) {
            (CartesianAxis::Category(_x_items), CartesianAxis::Category(_y_items)) => {
                todo!()
            }
            (CartesianAxis::Category(x_items), CartesianAxis::Values) => {
                // X-Axis
                if self.x_axis.axis_show {
                    let line = crate::primitives::Line {
                        stroke: &self.x_axis.axis_stroke,
                        stroke_color: &self.x_axis.axis_color,
                        coords: (
                            Point::new(helper.offsets.x_axis_start, helper.offsets.y_axis_end),
                            Point::new(helper.offsets.x_axis_end, helper.offsets.y_axis_end),
                        ),
                    };
                    primitives.push(crate::primitives::Primitives::Line(line));
                }

                // X-Axis ticks
                if self.x_axis.ticks_show {
                    let label_spacing = helper.offsets.x_span / x_items.len() as f64;
                    for label_index in 0..=x_items.len() {
                        let x_pos =
                            helper.offsets.x_axis_start + label_index as f64 * label_spacing;
                        let y_pos = helper.offsets.y_axis_end + self.x_axis.ticks_length;
                        let line = crate::primitives::Line {
                            stroke: &self.x_axis.ticks_stroke,
                            stroke_color: &self.x_axis.ticks_color,
                            coords: (
                                Point::new(x_pos, helper.offsets.y_axis_end),
                                Point::new(x_pos, y_pos),
                            ),
                        };
                        primitives.push(crate::primitives::Primitives::Line(line));
                    }
                }
                // X-Axis labels
                if self.x_axis.labels_show {
                    for (label_index, label) in x_items.iter().enumerate() {
                        let label_spacing = helper.offsets.x_span / x_items.len() as f64;
                        let x_pos = helper.offsets.x_axis_start
                            + (label_index as f64 + 0.5) * label_spacing;
                        let y_pos = helper.offsets.y_axis_end + self.x_axis.labels_margin;
                        let text = crate::primitives::Text {
                            text: label.to_string(),
                            fill_color: &self.x_axis.labels_color,
                            font_size: 12.0,
                            text_anchor: parley::Alignment::Middle,
                            coord: Point::new(x_pos, y_pos),
                        };
                        primitives.push(crate::primitives::Primitives::Text(text));
                    }
                }

                let (min, max, step_size) = calculate_axis_ticks(&self.data);
                let sub_tick_spacing = helper.offsets.y_span / (max / step_size);

                // Y-Axis
                if self.y_axis.axis_show {
                    let line = crate::primitives::Line {
                        stroke: &self.y_axis.axis_stroke,
                        stroke_color: &self.y_axis.axis_color,
                        coords: (
                            Point::new(helper.offsets.x_axis_start, helper.offsets.y_axis_start),
                            Point::new(helper.offsets.x_axis_start, helper.offsets.y_axis_end),
                        ),
                    };
                    primitives.push(crate::primitives::Primitives::Line(line));
                }

                // Y-Axis grid lines
                if self.y_axis.grid_show {
                    for sub_tick_index in 1..((max / step_size) as i32 + 1) {
                        let sub_tick_height =
                            helper.offsets.y_axis_end - (sub_tick_index as f64 * sub_tick_spacing);

                        let line = crate::primitives::Line {
                            stroke: &self.y_axis.grid_stroke,
                            stroke_color: &self.y_axis.grid_color,
                            coords: (
                                Point::new(helper.offsets.x_axis_start, sub_tick_height),
                                Point::new(helper.offsets.x_axis_end, sub_tick_height),
                            ),
                        };

                        primitives.push(crate::primitives::Primitives::Line(line));
                    }
                }

                // Y-Axis labels
                if self.y_axis.labels_show {
                    for sub_tick_index in 0..((max / step_size) as i32 + 1) {
                        let sub_tick_height =
                            helper.offsets.y_axis_end - (sub_tick_index as f64 * sub_tick_spacing);
                        let text = crate::primitives::Text {
                            text: format!("{}", min + step_size * sub_tick_index as f64),
                            fill_color: &self.y_axis.labels_color,
                            font_size: 12.0,
                            text_anchor: parley::Alignment::End,
                            coord: Point::new(
                                helper.offsets.x_axis_start - self.y_axis.labels_margin,
                                sub_tick_height,
                            ),
                        };
                        primitives.push(crate::primitives::Primitives::Text(text));
                    }
                }

                let mut path = crate::primitives::Path {
                    // TODO change stroke types
                    stroke: &self.y_axis.axis_stroke,
                    stroke_color: &self.y_axis.axis_color,
                    coords: Vec::with_capacity(self.data.len()),
                };
                for (index, y_item) in self.data.iter().enumerate() {
                    let y_pos = {
                        let percentage_height = y_item / max;
                        helper.offsets.y_axis_end - (percentage_height * helper.offsets.y_span)
                    };
                    let x_spacing = helper.offsets.x_span / x_items.len() as f64;
                    let x_pos = helper.offsets.x_axis_start + (index as f64 + 0.5) * x_spacing;
                    path.coords.push(Point::new(x_pos, y_pos));
                }
                primitives.push(crate::primitives::Primitives::Path(path));
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
