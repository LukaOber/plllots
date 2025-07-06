use bon::Builder;
use kurbo::{Point, Stroke};
use peniko::Brush;

use crate::{
    chart::{ChartHelper, Theme},
    component::SingleCartesianAxis,
    primitives::Primitives,
};

#[derive(Debug, Builder, Clone)]
pub struct Scatter {
    #[builder(setters(option_fn(vis = "")))]
    pub stroke: Option<Stroke>,
    #[builder(setters(option_fn(vis = "")))]
    pub stroke_color: Option<Brush>,
    #[builder(setters(option_fn(vis = "")))]
    pub fill_color: Option<Brush>,
    #[builder(setters(option_fn(vis = "")))]
    pub symbol_size: Option<f64>,
    #[builder(default = 0, setters(option_fn(vis = "")))]
    pub x_axis_index: usize,
    #[builder(default = 0, setters(option_fn(vis = "")))]
    pub y_axis_index: usize,
    #[builder(into)]
    pub data: ScatterData,
}

#[derive(Debug, Builder, Clone)]
pub struct ScatterData {
    #[builder(setters(option_fn(vis = "")))]
    pub primary_data_index: Option<usize>,
    #[builder(setters(option_fn(vis = "")))]
    pub secondary_data_index: Option<usize>,
    #[builder(setters(option_fn(vis = "")))]
    pub symbol_size_index: Option<usize>,
    pub data: Vec<Vec<f64>>,
}

impl From<Vec<f64>> for ScatterData {
    fn from(value: Vec<f64>) -> Self {
        ScatterData {
            primary_data_index: None,
            secondary_data_index: None,
            symbol_size_index: None,
            data: vec![value],
        }
    }
}

impl From<Vec<Vec<f64>>> for ScatterData {
    fn from(value: Vec<Vec<f64>>) -> Self {
        ScatterData {
            primary_data_index: None,
            secondary_data_index: None,
            symbol_size_index: None,
            data: value,
        }
    }
}

impl Scatter {
    pub(crate) fn draw_scatter<'a>(
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

                let primary_data_index = self.data.primary_data_index.unwrap_or(0);
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

                let primary_data_index = self.data.primary_data_index.unwrap_or(0);
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
                let primary_data_index = self.data.primary_data_index.unwrap_or(0);
                let secondary_data_index = self.data.secondary_data_index.unwrap_or(1);
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
        x_pos: &impl Fn(usize, f64) -> f64,
        y_pos: &impl Fn(usize, f64) -> f64,
        primary_data_index: usize,
        secondary_data_index: usize,
    ) {
        let allocation_size = self.data.data[primary_data_index].len();
        primitives.reserve(allocation_size);
        for (index, (primary_value, secondary_value)) in self.data.data[primary_data_index]
            .iter()
            .zip(self.data.data[secondary_data_index].iter())
            .enumerate()
        {
            let radius = match self.data.symbol_size_index {
                Some(i) => self.data.data[i][index],
                None => self.symbol_size.unwrap_or(theme.scatter.symbol_size),
            };
            primitives.push(crate::primitives::Primitives::Circle(
                crate::primitives::Circle {
                    stroke: self.stroke.as_ref().unwrap_or(&theme.scatter.stroke),
                    stroke_color: self
                        .stroke_color
                        .as_ref()
                        .unwrap_or(&theme.scatter.stroke_color),
                    fill_color: self
                        .fill_color
                        .as_ref()
                        .unwrap_or(&theme.series_colors[series_index % theme.series_colors.len()]),
                    coord: Point::new(x_pos(index, *secondary_value), y_pos(index, *primary_value)),
                    radius,
                },
            ));
        }
    }
}
