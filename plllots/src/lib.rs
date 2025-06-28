//! A plotting library for creating SVG charts in Rust.
//!
//! This library provides a flexible and extensible way to create various types of charts
//! and render them as SVG. It takes inspiration from modern charting libraries with a
//! clean, builder-pattern API.
//!
//! # Quick Start
//!
//! ```rust
//! use plllots::{Chart, SvgRenderer};
//! use plllots::element::PlotSize;
//! use plllots::component::{XAxis, YAxis, AxisData};
//!
//! let chart = Chart::builder()
//!    .size(PlotSize {
//!        width: 1000.0,
//!        height: 1000.0,
//!    })
//!    .x_axis(
//!        XAxis::builder()
//!            .data(AxisData::Category(bon::vec![
//!                "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun",
//!            ]))
//!            .build(),
//!    )
//!    .y_axis(
//!        YAxis::builder()
//!            .data(AxisData::Values(vec![
//!                150.0, 230.0, 224.0, 218.0, 135.0, 147.0, 260.0,
//!            ]))
//!            .build(),
//!    )
//!    .build();
//!
//! let renderer = SvgRenderer::new();
//! renderer.save(&chart, "line.svg").unwrap();
//! ```

pub mod chart;
pub mod component;
pub mod coordinate_system;
pub mod element;
mod primitives;
pub mod renderer;
pub mod series;
pub mod utils;

// Re-export commonly used items for convenience
pub use bon;
pub use chart::{Chart, ChartPlotHelper};
pub use component::{CartesianAxis, XAxis, YAxis};
pub use element::{MarginType, Margins, Offsets, PlotSize};
pub use renderer::SvgRenderer;
pub use series::{LineSeries, RenderSeries};

#[cfg(test)]
mod tests {
    use crate::coordinate_system::Cartesian;

    use super::*;

    #[test]
    fn it_works() {
        let chart = Chart::builder()
            .size(PlotSize {
                width: 1000.0,
                height: 1000.0,
            })
            .coordinate_system(coordinate_system::CoordinateSystem::Cartesian(
                Cartesian::builder()
                    .x_axis(
                        XAxis::builder()
                            .data(CartesianAxis::Category(bon::vec![
                                "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun",
                            ]))
                            .build(),
                    )
                    .y_axis(
                        YAxis::builder()
                            .data(CartesianAxis::Values(vec![
                                150.0, 230.0, 224.0, 218.0, 135.0, 147.0, 260.0,
                            ]))
                            .build(),
                    )
                    .build(),
            ))
            .build();

        let renderer = SvgRenderer::new();
        renderer.save(&chart, "line.svg").unwrap();
        assert!(false)
    }
}
