use bon::Builder;
use kurbo::Point;

use crate::{
    chart::Theme,
    component::{AxisType, CartesianAxis},
    primitives::AppendPrimitives,
    series::Series,
    utils::{get_raw_range, get_scale_details},
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
        theme: &'a Theme,
    ) {
        match (&self.x_axis, &self.y_axis) {
            (CartesianAxis::Category(_x_axes), CartesianAxis::Category(_y_axes)) => todo!(),
            (CartesianAxis::Category(x_axes), CartesianAxis::Value(y_axes)) => {
                for (x_axis_index, x_axis) in x_axes.iter().enumerate() {
                    x_axis.draw_split_lines(&AxisType::XAxis, primitives, helper, theme);
                    x_axis.draw_axis_ticks(
                        x_axis_index,
                        &AxisType::XAxis,
                        primitives,
                        helper,
                        theme,
                    );
                    x_axis.draw_labels(x_axis_index, &AxisType::XAxis, primitives, helper, theme);
                    for (y_axis_index, y_axis) in y_axes.iter().enumerate() {
                        let mut filtered_series = self.series.iter().filter(|s| {
                            x_axis_index == s.x_axis_index() && y_axis_index == s.y_axis_index()
                        });
                        let (mut min, mut max) = match filtered_series.next() {
                            Some(s) => s.get_raw_range(),
                            None => continue,
                        };

                        for series in filtered_series {
                            let (s_min, s_max) = series.get_raw_range();
                            min = min.min(s_min);
                            max = max.max(s_max);
                        }

                        let (min, max, step_size) = get_scale_details(min, max);

                        y_axis.draw_split_lines(
                            &AxisType::YAxis,
                            primitives,
                            helper,
                            theme,
                            min,
                            max,
                            step_size,
                        );
                        y_axis.draw_axis_ticks(
                            y_axis_index,
                            &AxisType::YAxis,
                            primitives,
                            helper,
                            theme,
                            min,
                            max,
                            step_size,
                        );
                        y_axis.draw_labels(
                            y_axis_index,
                            &AxisType::YAxis,
                            primitives,
                            helper,
                            theme,
                            min,
                            max,
                            step_size,
                        );
                        y_axis.draw_axis_line(
                            y_axis_index,
                            &AxisType::YAxis,
                            primitives,
                            helper,
                            theme,
                        );

                        self.series
                            .iter()
                            .enumerate()
                            .filter(|(_i, s)| {
                                x_axis_index == s.x_axis_index() && y_axis_index == s.y_axis_index()
                            })
                            .for_each(|(i, s)| match s {
                                Series::Line(line) => {
                                    let mut path = crate::primitives::Path {
                                        stroke: line.stroke.as_ref().unwrap_or(&theme.line.stroke),
                                        stroke_color: line.color.as_ref().unwrap_or(
                                            &theme.series_colors[i % theme.series_colors.len()],
                                        ),
                                        coords: Vec::with_capacity(line.data.len()),
                                    };
                                    let mut symbols = Vec::with_capacity(line.data.len());

                                    for (index, y_item) in line.data.iter().enumerate() {
                                        let y_pos = {
                                            let percentage_height = (y_item - min) / (max - min);
                                            helper.offsets.y_axis_start
                                                - (percentage_height * helper.offsets.y_span)
                                        };
                                        let x_spacing =
                                            helper.offsets.x_span / x_axis.data.len() as f64;
                                        let x_pos = helper.offsets.x_axis_start
                                            + (index as f64 + 0.5) * x_spacing;
                                        path.coords.push(Point::new(x_pos, y_pos));
                                        if line.symbol_show.unwrap_or(theme.line.symbol_show) {
                                            symbols.push(crate::primitives::Primitives::Circle(
                                                crate::primitives::Circle {
                                                    stroke: line
                                                        .symbol_stroke
                                                        .as_ref()
                                                        .unwrap_or(&theme.line.symbol_stroke),
                                                    stroke_color: line
                                                        .symbol_stroke_color
                                                        .as_ref()
                                                        .unwrap_or(
                                                            &theme.series_colors
                                                                [i % theme.series_colors.len()],
                                                        ),
                                                    fill_color: line
                                                        .symbol_fill_color
                                                        .as_ref()
                                                        .unwrap_or(&theme.line.symbol_fill_color),
                                                    coord: Point::new(x_pos, y_pos),
                                                    radius: line
                                                        .symbol_size
                                                        .unwrap_or(theme.line.symbol_size),
                                                },
                                            ));
                                        }
                                    }
                                    primitives.push(crate::primitives::Primitives::Path(path));
                                    for symbol in symbols {
                                        primitives.push(symbol);
                                    }
                                }
                            });
                    }
                    x_axis.draw_axis_line(
                        x_axis_index,
                        &AxisType::XAxis,
                        primitives,
                        helper,
                        theme,
                    );
                }
            }
            (CartesianAxis::Value(x_axes), CartesianAxis::Category(y_axes)) => {
                for (y_axis_index, y_axis) in y_axes.iter().enumerate() {
                    y_axis.draw_split_lines(&AxisType::YAxis, primitives, helper, theme);
                    y_axis.draw_axis_ticks(
                        y_axis_index,
                        &AxisType::YAxis,
                        primitives,
                        helper,
                        theme,
                    );
                    y_axis.draw_labels(y_axis_index, &AxisType::YAxis, primitives, helper, theme);
                    for (x_axis_index, x_axis) in x_axes.iter().enumerate() {
                        let mut filtered_series = self.series.iter().filter(|s| {
                            y_axis_index == s.x_axis_index() && x_axis_index == s.y_axis_index()
                        });
                        let (mut min, mut max) = match filtered_series.next() {
                            Some(s) => s.get_raw_range(),
                            None => todo!(),
                        };

                        for series in filtered_series {
                            let (s_min, s_max) = series.get_raw_range();
                            min = min.min(s_min);
                            max = max.max(s_max);
                        }

                        let (min, max, step_size) = get_scale_details(min, max);

                        x_axis.draw_split_lines(
                            &AxisType::XAxis,
                            primitives,
                            helper,
                            theme,
                            min,
                            max,
                            step_size,
                        );
                        x_axis.draw_axis_ticks(
                            x_axis_index,
                            &AxisType::XAxis,
                            primitives,
                            helper,
                            theme,
                            min,
                            max,
                            step_size,
                        );
                        x_axis.draw_labels(
                            x_axis_index,
                            &AxisType::XAxis,
                            primitives,
                            helper,
                            theme,
                            min,
                            max,
                            step_size,
                        );
                        x_axis.draw_axis_line(
                            x_axis_index,
                            &AxisType::XAxis,
                            primitives,
                            helper,
                            theme,
                        );
                        self.series
                            .iter()
                            .enumerate()
                            .filter(|(_i, s)| {
                                y_axis_index == s.x_axis_index() && x_axis_index == s.y_axis_index()
                            })
                            .for_each(|(i, s)| match s {
                                Series::Line(line) => {
                                    let mut path = crate::primitives::Path {
                                        stroke: line.stroke.as_ref().unwrap_or(&theme.line.stroke),
                                        stroke_color: &theme.series_colors
                                            [i % theme.series_colors.len()],
                                        coords: Vec::with_capacity(line.data.len()),
                                    };
                                    for (index, x_item) in line.data.iter().enumerate() {
                                        let x_pos = {
                                            let percentage_height = (x_item - min) / (max - min);
                                            helper.offsets.x_axis_start
                                                + (percentage_height * helper.offsets.x_span)
                                        };
                                        let y_spacing =
                                            helper.offsets.y_span / y_axis.data.len() as f64;
                                        let y_pos = helper.offsets.y_axis_start
                                            - (index as f64 + 0.5) * y_spacing;
                                        path.coords.push(Point::new(x_pos, y_pos));
                                    }
                                    primitives.push(crate::primitives::Primitives::Path(path));
                                }
                            });
                    }
                    y_axis.draw_axis_line(
                        y_axis_index,
                        &AxisType::YAxis,
                        primitives,
                        helper,
                        theme,
                    );
                }
            }
            (CartesianAxis::Value(x_axes), CartesianAxis::Value(y_axes)) => {
                for (x_axis_index, x_axis) in x_axes.iter().enumerate() {
                    let mut filtered_series = self
                        .series
                        .iter()
                        .filter(|s| x_axis_index == s.x_axis_index());
                    let (mut x_min, mut x_max) = match filtered_series.next() {
                        Some(series) => match series {
                            Series::Line(line) => get_raw_range(&line.double_data[1]),
                        },
                        None => continue,
                    };

                    for series in filtered_series {
                        let (s_min, s_max) = match series {
                            Series::Line(line) => get_raw_range(&line.double_data[1]),
                        };
                        x_min = x_min.min(s_min);
                        x_max = x_max.max(s_max);
                    }

                    let (x_min, x_max, step_size) = get_scale_details(x_min, x_max);
                    x_axis.draw_split_lines(
                        &AxisType::XAxis,
                        primitives,
                        helper,
                        theme,
                        x_min,
                        x_max,
                        step_size,
                    );
                    x_axis.draw_axis_ticks(
                        x_axis_index,
                        &AxisType::XAxis,
                        primitives,
                        helper,
                        theme,
                        x_min,
                        x_max,
                        step_size,
                    );
                    x_axis.draw_labels(
                        x_axis_index,
                        &AxisType::XAxis,
                        primitives,
                        helper,
                        theme,
                        x_min,
                        x_max,
                        step_size,
                    );
                    x_axis.draw_axis_line(
                        x_axis_index,
                        &AxisType::XAxis,
                        primitives,
                        helper,
                        theme,
                    );
                    for (y_axis_index, y_axis) in y_axes.iter().enumerate() {
                        let mut filtered_series = self
                            .series
                            .iter()
                            .filter(|s| y_axis_index == s.y_axis_index());
                        let (mut y_min, mut y_max) = match filtered_series.next() {
                            Some(series) => match series {
                                Series::Line(line) => get_raw_range(&line.double_data[0]),
                            },
                            None => continue,
                        };

                        for series in filtered_series {
                            let (s_min, s_max) = match series {
                                Series::Line(line) => get_raw_range(&line.double_data[0]),
                            };
                            y_min = y_min.min(s_min);
                            y_max = y_max.max(s_max);
                        }

                        let (y_min, y_max, step_size) = get_scale_details(y_min, y_max);
                        y_axis.draw_split_lines(
                            &AxisType::YAxis,
                            primitives,
                            helper,
                            theme,
                            y_min,
                            y_max,
                            step_size,
                        );
                        y_axis.draw_axis_ticks(
                            y_axis_index,
                            &AxisType::YAxis,
                            primitives,
                            helper,
                            theme,
                            y_min,
                            y_max,
                            step_size,
                        );
                        y_axis.draw_labels(
                            y_axis_index,
                            &AxisType::YAxis,
                            primitives,
                            helper,
                            theme,
                            y_min,
                            y_max,
                            step_size,
                        );
                        y_axis.draw_axis_line(
                            y_axis_index,
                            &AxisType::YAxis,
                            primitives,
                            helper,
                            theme,
                        );

                        self.series
                            .iter()
                            .enumerate()
                            .filter(|(_i, s)| {
                                println!("{:?}", s);
                                x_axis_index == s.x_axis_index() && y_axis_index == s.y_axis_index()
                            })
                            .for_each(|(i, s)| match s {
                                Series::Line(line) => {
                                    println!("asd");
                                    let mut path = crate::primitives::Path {
                                        stroke: line.stroke.as_ref().unwrap_or(&theme.line.stroke),
                                        stroke_color: line.color.as_ref().unwrap_or(
                                            &theme.series_colors[i % theme.series_colors.len()],
                                        ),
                                        coords: Vec::with_capacity(line.data.len()),
                                    };
                                    let mut symbols = Vec::with_capacity(line.data.len());

                                    for (y_item, x_item) in
                                        line.double_data[0].iter().zip(line.double_data[1].iter())
                                    {
                                        let y_pos = {
                                            let percentage_height =
                                                (y_item - y_min) / (y_max - y_min);
                                            helper.offsets.y_axis_start
                                                - (percentage_height * helper.offsets.y_span)
                                        };
                                        let x_pos = {
                                            let percentage_width =
                                                (x_item - x_min) / (x_max - x_min);
                                            println!("Perc: {:?}", percentage_width);
                                            helper.offsets.x_axis_start
                                                + (percentage_width * helper.offsets.x_span)
                                        };
                                        println!("X: {:?}", x_pos);
                                        path.coords.push(Point::new(x_pos, y_pos));
                                        if line.symbol_show.unwrap_or(theme.line.symbol_show) {
                                            symbols.push(crate::primitives::Primitives::Circle(
                                                crate::primitives::Circle {
                                                    stroke: line
                                                        .symbol_stroke
                                                        .as_ref()
                                                        .unwrap_or(&theme.line.symbol_stroke),
                                                    stroke_color: line
                                                        .symbol_stroke_color
                                                        .as_ref()
                                                        .unwrap_or(
                                                            &theme.series_colors
                                                                [i % theme.series_colors.len()],
                                                        ),
                                                    fill_color: line
                                                        .symbol_fill_color
                                                        .as_ref()
                                                        .unwrap_or(&theme.line.symbol_fill_color),
                                                    coord: Point::new(x_pos, y_pos),
                                                    radius: line
                                                        .symbol_size
                                                        .unwrap_or(theme.line.symbol_size),
                                                },
                                            ));
                                        }
                                    }
                                    primitives.push(crate::primitives::Primitives::Path(path));
                                    for symbol in symbols {
                                        primitives.push(symbol);
                                    }
                                }
                            });
                    }
                }
            }
        }
    }
}
