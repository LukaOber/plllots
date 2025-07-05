use bon::Builder;
use std::iter::Peekable;

use crate::{
    chart::Theme,
    component::{AxisType, CartesianAxis},
    primitives::AppendPrimitives,
    series::Series,
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

impl Cartesian {
    fn filtered_series(
        &self,
        x_axis_index: Option<usize>,
        y_axis_index: Option<usize>,
        // ) -> impl Iterator<Item = &Series> {
    ) -> Peekable<impl Iterator<Item = &Series>> {
        self.series
            .iter()
            .filter(move |s| {
                x_axis_index.unwrap_or(s.x_axis_index()) == s.x_axis_index()
                    && y_axis_index.unwrap_or(s.y_axis_index()) == s.y_axis_index()
            })
            .peekable()
    }
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
                    x_axis.draw_axis(x_axis_index, &AxisType::XAxis, primitives, helper, theme);
                    for (y_axis_index, y_axis) in y_axes.iter().enumerate() {
                        let mut filtered_series =
                            self.filtered_series(Some(x_axis_index), Some(y_axis_index));

                        match filtered_series.peek() {
                            Some(_) => (),
                            None => break,
                        }
                        let y_axis_meta = y_axis.draw_axis(
                            y_axis_index,
                            &AxisType::YAxis,
                            primitives,
                            helper,
                            theme,
                            filtered_series,
                            true,
                        );
                        let filtered_series =
                            self.filtered_series(Some(x_axis_index), Some(y_axis_index));
                        filtered_series.enumerate().for_each(|(i, s)| match s {
                            Series::Line(line) => line.draw_line(
                                i,
                                &x_axis.into(),
                                &(y_axis, &y_axis_meta).into(),
                                helper,
                                primitives,
                                theme,
                            ),
                        });
                    }
                }
            }
            (CartesianAxis::Value(x_axes), CartesianAxis::Category(y_axes)) => {
                for (y_axis_index, y_axis) in y_axes.iter().enumerate() {
                    y_axis.draw_axis(y_axis_index, &AxisType::YAxis, primitives, helper, theme);
                    for (x_axis_index, x_axis) in x_axes.iter().enumerate() {
                        let mut filtered_series =
                            self.filtered_series(Some(x_axis_index), Some(y_axis_index));

                        match filtered_series.peek() {
                            Some(_) => (),
                            None => break,
                        }
                        let x_axis_meta = x_axis.draw_axis(
                            x_axis_index,
                            &AxisType::XAxis,
                            primitives,
                            helper,
                            theme,
                            filtered_series,
                            true,
                        );
                        let filtered_series =
                            self.filtered_series(Some(x_axis_index), Some(y_axis_index));
                        filtered_series.enumerate().for_each(|(i, s)| match s {
                            Series::Line(line) => line.draw_line(
                                i,
                                &(x_axis, &x_axis_meta).into(),
                                &y_axis.into(),
                                helper,
                                primitives,
                                theme,
                            ),
                        });
                    }
                }
            }
            (CartesianAxis::Value(x_axes), CartesianAxis::Value(y_axes)) => {
                for (x_axis_index, x_axis) in x_axes.iter().enumerate() {
                    let mut filtered_series = self.filtered_series(Some(x_axis_index), None);

                    match filtered_series.peek() {
                        Some(_) => (),
                        None => break,
                    }
                    let x_axis_meta = x_axis.draw_axis(
                        x_axis_index,
                        &AxisType::XAxis,
                        primitives,
                        helper,
                        theme,
                        filtered_series,
                        false,
                    );
                    for (y_axis_index, y_axis) in y_axes.iter().enumerate() {
                        let mut filtered_series = self.filtered_series(None, Some(y_axis_index));

                        match filtered_series.peek() {
                            Some(_) => (),
                            None => break,
                        }
                        let y_axis_meta = y_axis.draw_axis(
                            y_axis_index,
                            &AxisType::YAxis,
                            primitives,
                            helper,
                            theme,
                            filtered_series,
                            true,
                        );
                        let filtered_series =
                            self.filtered_series(Some(x_axis_index), Some(y_axis_index));
                        filtered_series.enumerate().for_each(|(i, s)| match s {
                            Series::Line(line) => line.draw_line(
                                i,
                                &(x_axis, &x_axis_meta).into(),
                                &(y_axis, &y_axis_meta).into(),
                                helper,
                                primitives,
                                theme,
                            ),
                        });
                    }
                }
            }
        }
    }
}
