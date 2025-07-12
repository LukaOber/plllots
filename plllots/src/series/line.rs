use bon::Builder;
use kurbo::{Point, Stroke};
use peniko::Brush;

use crate::{
    chart::{ChartHelper, Theme},
    component::SingleCartesianAxis,
    primitives::Primitives,
    utils::lttb::lttb_optimized_memory,
};

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Line {
    #[builder(setters(option_fn(vis = "")))]
    pub stroke: Option<Stroke>,
    #[builder(setters(option_fn(vis = "")))]
    pub color: Option<Brush>,
    #[builder(default = 0, setters(option_fn(vis = "")))]
    pub x_axis_index: usize,
    #[builder(default = 0, setters(option_fn(vis = "")))]
    pub y_axis_index: usize,
    #[builder(setters(option_fn(vis = "")))]
    pub symbol_show: Option<bool>,
    #[builder(setters(option_fn(vis = "")))]
    pub symbol_stroke: Option<Stroke>,
    #[builder(setters(option_fn(vis = "")))]
    pub symbol_stroke_color: Option<Brush>,
    #[builder(setters(option_fn(vis = "")))]
    pub symbol_fill_color: Option<Brush>,
    #[builder(setters(option_fn(vis = "")))]
    pub symbol_size: Option<f64>,
    #[builder(into)]
    pub data: LineData,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct LineData {
    #[builder(default = 0)]
    pub primary_data_index: usize,
    #[builder(default = 1)]
    pub secondary_data_index: usize,
    #[builder(setters(option_fn(vis = "")))]
    pub lttb: Option<usize>,
    pub data: Vec<Vec<f64>>,
}

impl From<Vec<f64>> for LineData {
    fn from(value: Vec<f64>) -> Self {
        LineData {
            primary_data_index: 0,
            secondary_data_index: 1,
            lttb: None,
            data: vec![value],
        }
    }
}

impl From<Vec<Vec<f64>>> for LineData {
    fn from(value: Vec<Vec<f64>>) -> Self {
        LineData {
            primary_data_index: 0,
            secondary_data_index: 1,
            lttb: None,
            data: value,
        }
    }
}

impl Line {
    pub(crate) fn draw_line<'a>(
        &'a self,
        series_index: usize,
        x_axis: &SingleCartesianAxis,
        y_axis: &SingleCartesianAxis,
        helper: &ChartHelper,
        primitives: &mut Vec<Primitives<'a>>,
        theme: &'a Theme,
    ) {
        match (x_axis, y_axis) {
            (SingleCartesianAxis::Category(_x_axis), SingleCartesianAxis::Category(_y_axis)) => {
                todo!()
            }
            (
                SingleCartesianAxis::Category(x_axis),
                SingleCartesianAxis::Value((y_axis, y_helper)),
            ) => {
                let x_pos = x_axis.pos_closure(&crate::component::AxisType::XAxis, helper);
                let y_pos =
                    y_axis.pos_closure(&crate::component::AxisType::YAxis, &y_helper, helper);

                let primary_data_index = self.data.primary_data_index;
                self.draw(
                    primitives,
                    theme,
                    series_index,
                    &x_pos,
                    &y_pos,
                    primary_data_index,
                    primary_data_index,
                );
            }
            (
                SingleCartesianAxis::Value((x_axis, x_helper)),
                SingleCartesianAxis::Category(y_axis),
            ) => {
                let x_pos =
                    x_axis.pos_closure(&crate::component::AxisType::XAxis, &x_helper, helper);
                let y_pos = y_axis.pos_closure(&crate::component::AxisType::YAxis, helper);

                let primary_data_index = self.data.primary_data_index;
                self.draw(
                    primitives,
                    theme,
                    series_index,
                    &x_pos,
                    &y_pos,
                    primary_data_index,
                    primary_data_index,
                );
            }
            (
                SingleCartesianAxis::Value((x_axis, x_helper)),
                SingleCartesianAxis::Value((y_axis, y_helper)),
            ) => {
                let x_pos =
                    x_axis.pos_closure(&crate::component::AxisType::XAxis, &x_helper, helper);
                let y_pos =
                    y_axis.pos_closure(&crate::component::AxisType::YAxis, &y_helper, helper);

                let primary_data_index = self.data.primary_data_index;
                let secondary_data_index = self.data.secondary_data_index;
                self.draw(
                    primitives,
                    theme,
                    series_index,
                    &x_pos,
                    &y_pos,
                    primary_data_index,
                    secondary_data_index,
                );
            }
        };
    }

    fn draw<'a>(
        &'a self,
        primitives: &mut Vec<Primitives<'a>>,
        theme: &'a Theme,
        series_index: usize,
        x_pos: &impl Fn(Option<usize>, Option<f64>) -> f64,
        y_pos: &impl Fn(Option<usize>, Option<f64>) -> f64,
        primary_data_index: usize,
        secondary_data_index: usize,
    ) {
        let mut path = crate::primitives::Path {
            stroke: self.stroke.as_ref().unwrap_or(&theme.line.stroke),
            stroke_color: self
                .color
                .as_ref()
                .unwrap_or(&theme.series_colors[series_index % theme.series_colors.len()]),
            coords: Vec::new(),
        };
        match self.data.lttb {
            Some(t) => {
                // TODO probably need to keep original index
                path.coords.reserve(t);
                let (primary_values, secondary_values) = lttb_optimized_memory(
                    &self.data.data[primary_data_index],
                    &self.data.data[secondary_data_index],
                    t,
                );
                for (index, (primary_value, secondary_value)) in primary_values
                    .iter()
                    .zip(secondary_values.iter())
                    .enumerate()
                {
                    path.coords.push(Point::new(
                        x_pos(Some(index), Some(*secondary_value)),
                        y_pos(Some(index), Some(*primary_value)),
                    ));
                }
            }
            None => {
                path.coords
                    .reserve(self.data.data[primary_data_index].len());
                for (index, (primary_value, secondary_value)) in self.data.data[primary_data_index]
                    .iter()
                    .zip(self.data.data[secondary_data_index].iter())
                    .enumerate()
                {
                    path.coords.push(Point::new(
                        x_pos(Some(index), Some(*secondary_value)),
                        y_pos(Some(index), Some(*primary_value)),
                    ));
                }
            }
        }

        if self.symbol_show.unwrap_or(theme.line.symbol_show) {
            let nulti_circle =
                crate::primitives::Primitives::MultiCircle(crate::primitives::MultiCircle {
                    stroke: self
                        .symbol_stroke
                        .as_ref()
                        .unwrap_or(&theme.line.symbol_stroke),
                    stroke_color: self
                        .symbol_stroke_color
                        .as_ref()
                        .unwrap_or(&theme.series_colors[series_index % theme.series_colors.len()]),
                    fill_color: self
                        .symbol_fill_color
                        .as_ref()
                        .unwrap_or(&theme.line.symbol_fill_color),
                    // TODO: find a way to remove this clone
                    coords: path.coords.clone(),
                    radius: self.symbol_size.unwrap_or(theme.line.symbol_size),
                });
            primitives.push(crate::primitives::Primitives::Path(path));
            primitives.push(nulti_circle);
        } else {
            primitives.push(crate::primitives::Primitives::Path(path));
        }
    }
}
