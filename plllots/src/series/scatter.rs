use bon::Builder;
use kurbo::{Point, Stroke};
use peniko::Brush;

use crate::{
    chart::{ChartHelper, Theme},
    component::SingleCartesianAxis,
    primitives::Primitives,
};

use super::data::PlotData;

#[derive(Debug, Builder, Clone, PartialEq)]
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

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct ScatterData {
    #[builder(setters(vis = ""), default = 0)]
    pub primary_data_index: usize,
    #[builder(setters(vis = ""), default = 1)]
    pub secondary_data_index: usize,
    #[builder(setters(option_fn(vis = "")))]
    pub symbol_size_index: Option<usize>,
    #[builder(setters(vis = ""))]
    pub plot_data: Vec<PlotData>,
}

impl<S: scatter_data_builder::State> ScatterDataBuilder<S> {
    pub fn y_data_index(
        self,
        index: usize,
    ) -> ScatterDataBuilder<scatter_data_builder::SetPrimaryDataIndex<S>>
    where
        S::PrimaryDataIndex: scatter_data_builder::IsUnset,
    {
        self.primary_data_index(index)
    }

    pub fn radius_data_index(
        self,
        index: usize,
    ) -> ScatterDataBuilder<scatter_data_builder::SetPrimaryDataIndex<S>>
    where
        S::PrimaryDataIndex: scatter_data_builder::IsUnset,
    {
        self.primary_data_index(index)
    }

    pub fn data<D: Into<PlotData>>(
        self,
        data: Vec<D>,
    ) -> ScatterDataBuilder<scatter_data_builder::SetPlotData<S>>
    where
        S::PlotData: scatter_data_builder::IsUnset,
    {
        self.plot_data(data.into_iter().map(|d| d.into()).collect())
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
        // TODO readd later
        // let allocation_size = self.data.plot_data[primary_data_index].len();
        // primitives.reserve(allocation_size);

        for (index, (primary_value, secondary_value)) in self.data.plot_data[primary_data_index]
            .iter()
            .zip(self.data.plot_data[secondary_data_index].iter())
            .enumerate()
        {
            let radius = match self.data.symbol_size_index {
                Some(i) => self.data.plot_data[i][index],
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
