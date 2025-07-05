use bon::Builder;
use kurbo::{Point, Stroke};
use peniko::Brush;

use crate::{
    chart::{ChartHelper, Theme},
    component::{CartesianAxis, SingleCartesianAxis},
    primitives::Primitives,
};

#[derive(Debug, Builder, Clone)]
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

#[derive(Debug, Builder, Clone)]
pub struct LineData {
    #[builder(setters(option_fn(vis = "")))]
    pub primary_data_index: Option<usize>,
    #[builder(setters(option_fn(vis = "")))]
    pub secondary_data_index: Option<usize>,
    pub data: Vec<Vec<f64>>,
}

impl From<Vec<f64>> for LineData {
    fn from(value: Vec<f64>) -> Self {
        LineData {
            primary_data_index: None,
            secondary_data_index: None,
            data: vec![value],
        }
    }
}

impl From<Vec<Vec<f64>>> for LineData {
    fn from(value: Vec<Vec<f64>>) -> Self {
        LineData {
            primary_data_index: None,
            secondary_data_index: None,
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
        let mut path = crate::primitives::Path {
            stroke: self.stroke.as_ref().unwrap_or(&theme.line.stroke),
            stroke_color: self
                .color
                .as_ref()
                .unwrap_or(&theme.series_colors[series_index % theme.series_colors.len()]),
            coords: Vec::new(),
        };

        match (x_axis, y_axis) {
            (SingleCartesianAxis::Category(x_axis), SingleCartesianAxis::Category(y_axis)) => {
                todo!()
            }
            (
                SingleCartesianAxis::Category(x_axis),
                SingleCartesianAxis::Value((_y_axis, y_helper)),
            ) => {
                let x_pos = |i: usize, v: f64| {
                    let x_spacing = helper.offsets.x_span / x_axis.data.len() as f64;
                    helper.offsets.x_axis_start + (i as f64 + 0.5) * x_spacing
                };

                let y_pos = |i: usize, v: f64| {
                    let percentage_height = (v - y_helper.min) / (y_helper.max - y_helper.min);
                    helper.offsets.y_axis_start - (percentage_height * helper.offsets.y_span)
                };

                let primary_data_index = self.data.primary_data_index.unwrap_or(0);
                path.coords = Vec::with_capacity(self.data.data[primary_data_index].len());
                for (index, value) in self.data.data[primary_data_index].iter().enumerate() {
                    path.coords
                        .push(Point::new(x_pos(index, *value), y_pos(index, *value)));
                }
                primitives.push(crate::primitives::Primitives::Path(path));
            }
            (
                SingleCartesianAxis::Value((x_axis, x_helper)),
                SingleCartesianAxis::Category(y_axis),
            ) => {
                let x_pos = |i: usize, v: f64| {
                    let percentage_width = (v - x_helper.min) / (x_helper.max - x_helper.min);
                    helper.offsets.x_axis_start + (percentage_width * helper.offsets.x_span)
                };

                let y_pos = |i: usize, v: f64| {
                    let y_spacing = helper.offsets.y_span / y_axis.data.len() as f64;
                    helper.offsets.y_axis_start - (i as f64 + 0.5) * y_spacing
                };

                let primary_data_index = self.data.primary_data_index.unwrap_or(0);
                path.coords = Vec::with_capacity(self.data.data[primary_data_index].len());
                for (index, value) in self.data.data[primary_data_index].iter().enumerate() {
                    path.coords
                        .push(Point::new(x_pos(index, *value), y_pos(index, *value)));
                }
                primitives.push(crate::primitives::Primitives::Path(path));
            }
            (
                SingleCartesianAxis::Value((_x_axis, x_helper)),
                SingleCartesianAxis::Value((_y_axis, y_helper)),
            ) => {
                let x_pos = |i: usize, v: f64| {
                    let percentage_height = (v - x_helper.min) / (x_helper.max - x_helper.min);
                    helper.offsets.x_axis_start + (percentage_height * helper.offsets.x_span)
                };
                let y_pos = |i: usize, v: f64| {
                    let percentage_height = (v - y_helper.min) / (y_helper.max - y_helper.min);
                    helper.offsets.y_axis_start - (percentage_height * helper.offsets.y_span)
                };
                let primary_data_index = self.data.primary_data_index.unwrap_or(0);
                let secondary_data_index = self.data.primary_data_index.unwrap_or(1);
                path.coords = Vec::with_capacity(self.data.data[primary_data_index].len());

                for (index, (primary_value, secondary_value)) in self.data.data[primary_data_index]
                    .iter()
                    .zip(self.data.data[secondary_data_index].iter())
                    .enumerate()
                {
                    println!("{:?}, {:?}", primary_value, secondary_value);
                    path.coords.push(Point::new(
                        x_pos(index, *secondary_value),
                        y_pos(index, *primary_value),
                    ));
                }
                primitives.push(crate::primitives::Primitives::Path(path));
            }
        };
    }
}
