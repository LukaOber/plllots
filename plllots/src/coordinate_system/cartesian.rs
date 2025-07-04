use bon::Builder;
use itertools::iproduct;
use kurbo::Point;

use crate::{
    chart::ChartHelper,
    component::{AxisPosition, AxisType, CartesianAxis},
    primitives::AppendPrimitives,
    series::Series,
    utils::calculate_axis_ticks,
};

#[derive(Debug, Builder, Clone)]
pub struct Cartesian {
    #[builder(field)]
    pub series: Vec<Series>,
    #[builder(into)]
    pub x_axis: CartesianAxis,
    #[builder(into)]
    pub y_axis: CartesianAxis,
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
}

impl<'a> AppendPrimitives<'a> for Cartesian {
    fn append_primitives(
        &'a self,
        primitives: &mut Vec<crate::primitives::Primitives<'a>>,
        helper: &mut crate::chart::ChartHelper,
    ) {
        append_axes(&self.x_axis, &self.y_axis, &self.series, primitives, helper);
        match (&self.x_axis, &self.y_axis) {
            (CartesianAxis::Category(x_axes), CartesianAxis::Category(y_axes)) => todo!(),
            (CartesianAxis::Category(x_axes), CartesianAxis::Value(y_axes)) => {
                for ((x_axis_index, x_axis), (y_axis_index, y_axis)) in iproduct!(
                    x_axes.into_iter().enumerate(),
                    y_axes.into_iter().enumerate()
                ) {
                    // X-Axis
                    // if x_axis.axis_show {
                    //     let line = crate::primitives::Line {
                    //         stroke: &x_axis.axis_stroke,
                    //         stroke_color: &x_axis.axis_color,
                    //         coords: (
                    //             Point::new(helper.offsets.x_axis_start, helper.offsets.y_axis_end),
                    //             Point::new(helper.offsets.x_axis_end, helper.offsets.y_axis_end),
                    //         ),
                    //     };
                    //     primitives.push(crate::primitives::Primitives::Line(line));
                    // }

                    // X-Axis ticks
                    // if x_axis.ticks_show {
                    //     let label_spacing = helper.offsets.x_span / x_axis.data.len() as f64;
                    //     for label_index in 0..=x_axis.data.len() {
                    //         let x_pos =
                    //             helper.offsets.x_axis_start + label_index as f64 * label_spacing;
                    //         let y_pos = helper.offsets.y_axis_end + x_axis.ticks_length;
                    //         let line = crate::primitives::Line {
                    //             stroke: &x_axis.ticks_stroke,
                    //             stroke_color: &x_axis.ticks_color,
                    //             coords: (
                    //                 Point::new(x_pos, helper.offsets.y_axis_end),
                    //                 Point::new(x_pos, y_pos),
                    //             ),
                    //         };
                    //         primitives.push(crate::primitives::Primitives::Line(line));
                    //     }
                    // }
                    // X-Axis labels
                    // if x_axis.labels_show {
                    //     for (label_index, label) in x_axis.data.iter().enumerate() {
                    //         let label_spacing = helper.offsets.x_span / x_axis.data.len() as f64;
                    //         let x_pos = helper.offsets.x_axis_start
                    //             + (label_index as f64 + 0.5) * label_spacing;
                    //         let y_pos = helper.offsets.y_axis_end + x_axis.labels_margin;
                    //         let text = crate::primitives::Text {
                    //             text: label.to_string(),
                    //             fill_color: &x_axis.labels_color,
                    //             font_size: 12.0,
                    //             text_anchor: parley::Alignment::Middle,
                    //             coord: Point::new(x_pos, y_pos),
                    //         };
                    //         primitives.push(crate::primitives::Primitives::Text(text));
                    //     }
                    // }

                    let data = match self.series[0] {
                        Series::Line(ref line) => &line.data,
                    };
                    // let (min, max, step_size) = calculate_axis_ticks(data);
                    // let sub_tick_spacing = helper.offsets.y_span / (max / step_size);

                    // Y-Axis
                    // if y_axis.axis_show {
                    //     let line = crate::primitives::Line {
                    //         stroke: &y_axis.axis_stroke,
                    //         stroke_color: &y_axis.axis_color,
                    //         coords: (
                    //             Point::new(
                    //                 helper.offsets.x_axis_start,
                    //                 helper.offsets.y_axis_start,
                    //             ),
                    //             Point::new(helper.offsets.x_axis_start, helper.offsets.y_axis_end),
                    //         ),
                    //     };
                    //     primitives.push(crate::primitives::Primitives::Line(line));
                    // }

                    // Y-Axis grid lines
                    // if y_axis.split_lines_show {
                    //     for sub_tick_index in 1..((max / step_size) as i32 + 1) {
                    //         let sub_tick_height = helper.offsets.y_axis_start
                    //             - (sub_tick_index as f64 * sub_tick_spacing);

                    //         let line = crate::primitives::Line {
                    //             stroke: &y_axis.split_line_stroke,
                    //             stroke_color: &y_axis.split_lines_color,
                    //             coords: (
                    //                 Point::new(helper.offsets.x_axis_start, sub_tick_height),
                    //                 Point::new(helper.offsets.x_axis_end, sub_tick_height),
                    //             ),
                    //         };

                    //         primitives.push(crate::primitives::Primitives::Line(line));
                    //     }
                    // }

                    // Y-Axis labels
                    // if y_axis.labels_show {
                    //     for sub_tick_index in 0..((max / step_size) as i32 + 1) {
                    //         let sub_tick_height = helper.offsets.y_axis_start
                    //             - (sub_tick_index as f64 * sub_tick_spacing);
                    //         let text = crate::primitives::Text {
                    //             text: format!("{}", min + step_size * sub_tick_index as f64),
                    //             fill_color: &y_axis.labels_color,
                    //             font_size: 12.0,
                    //             text_anchor: parley::Alignment::End,
                    //             coord: Point::new(
                    //                 helper.offsets.x_axis_start - y_axis.labels_margin,
                    //                 sub_tick_height,
                    //             ),
                    //         };
                    //         primitives.push(crate::primitives::Primitives::Text(text));
                    //     }
                    // }

                    match &self.series[0] {
                        Series::Line(line) => {
                            let mut path = crate::primitives::Path {
                                stroke: &line.stroke,
                                stroke_color: &line.color,
                                coords: Vec::with_capacity(line.data.len()),
                            };
                            for (index, y_item) in data.iter().enumerate() {
                                let y_pos = {
                                    let percentage_height = (y_item + 200.0) / (300.0 - -200.0);
                                    println!("{percentage_height}");
                                    helper.offsets.y_axis_start
                                        - (percentage_height * helper.offsets.y_span)
                                };
                                println!("{y_pos}");
                                let x_spacing = helper.offsets.x_span / x_axis.data.len() as f64;
                                let x_pos =
                                    helper.offsets.x_axis_start + (index as f64 + 0.5) * x_spacing;
                                path.coords.push(Point::new(x_pos, y_pos));
                            }
                            primitives.push(crate::primitives::Primitives::Path(path));
                        }
                    }
                }
            }
            (CartesianAxis::Value(x_axis), CartesianAxis::Category(y_axis)) => todo!(),
            (CartesianAxis::Value(x_axis), CartesianAxis::Value(y_axis)) => todo!(),
        };
    }
}

fn append_axes<'a>(
    x_axes: &'a CartesianAxis,
    y_axes: &'a CartesianAxis,
    series: &'a Vec<Series>,
    primitives: &mut Vec<crate::primitives::Primitives<'a>>,
    helper: &ChartHelper,
) {
    match (x_axes, y_axes) {
        (CartesianAxis::Category(x_axes), CartesianAxis::Category(y_axes)) => todo!(),
        (CartesianAxis::Category(x_axes), CartesianAxis::Value(y_axes)) => {
            for (x_axis_index, x_axis) in x_axes.iter().enumerate() {
                x_axis.draw_axis_line(x_axis_index, &AxisType::XAxis, primitives, helper);
                x_axis.draw_axis_ticks(x_axis_index, &AxisType::XAxis, primitives, helper);
                for (y_axis_index, y_axis) in y_axes.iter().enumerate() {
                    y_axis.draw_axis_line(y_axis_index, &AxisType::YAxis, primitives, helper);
                    let mut filtered_series = series.into_iter().filter(|s| {
                        x_axis_index == s.x_axis_index() && y_axis_index == s.y_axis_index()
                    });
                    let (min, max, step_size) = match filtered_series.next() {
                        Some(s) => s.calculate_axis_ticks(),
                        None => todo!(),
                    };

                    y_axis.draw_axis_ticks(
                        y_axis_index,
                        &AxisType::YAxis,
                        primitives,
                        helper,
                        min,
                        max,
                        step_size,
                    );
                    y_axis.draw_split_lines(
                        &AxisType::YAxis,
                        primitives,
                        helper,
                        min,
                        max,
                        step_size,
                    );
                    y_axis.draw_labels(
                        y_axis_index,
                        &AxisType::YAxis,
                        primitives,
                        helper,
                        min,
                        max,
                        step_size,
                    );
                }
            }
        }
        (CartesianAxis::Value(x_axes), CartesianAxis::Category(y_axes)) => todo!(),
        (CartesianAxis::Value(x_axes), CartesianAxis::Value(y_axes)) => todo!(),
    }
}
