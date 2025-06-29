#![cfg_attr(docsrs, feature(doc_cfg))]
/*!

A plotting library for creating wgpu and svg charts in Rust.
This library provides a flexible and extensible way to create various types of charts
and render them as svg or with wgpu. It takes inspiration from modern charting libraries with a
clean, builder-pattern API and aims to be highly customizable while being beautiful by default..
# Quick Start
```rust
use plllots::{
    chart::Chart,
    component::{CartesianAxis, XAxis, YAxis},
    coordinate_system::{Cartesian, CoordinateSystem},
    element::PlotSize,
    renderer::SvgRenderer,
};
let chart = Chart::builder()
    .size(PlotSize {
        width: 1000.0,
        height: 1000.0,
    })
    .coordinate_system(CoordinateSystem::Cartesian(
        Cartesian::builder()
            .x_axis(
                XAxis::builder()
                    .cartesian_axis(CartesianAxis::Category(bon::vec![
                        "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun",
                    ]))
                    .build(),
            )
            .y_axis(
                YAxis::builder()
                    .cartesian_axis(CartesianAxis::Values)
                    .build(),
            )
            .data(vec![150.0, 230.0, 224.0, 218.0, 135.0, 147.0, 260.0])
            .build(),
    ))
    .build();

let renderer = SvgRenderer::new();
z<renderer.save(&chart, "line.svg").unwrap();
``` */
pub mod chart;
pub mod component;
pub mod coordinate_system;
pub mod element;
mod primitives;
pub mod renderer;
pub mod series;
pub mod utils;

pub use bon;

#[cfg(test)]
mod tests {
    use crate::{
        chart::Chart,
        component::{CartesianAxis, CartesianAxisLine, XAxis, YAxis},
        coordinate_system::{Cartesian, CoordinateSystem},
        element::PlotSize,
        renderer::SvgRenderer,
    };

    #[test]
    fn it_works() {
        let instant = std::time::Instant::now();
        let chart = Chart::builder()
            .size(PlotSize {
                width: 1000.0,
                height: 1000.0,
            })
            .coordinate_system(CoordinateSystem::Cartesian(
                Cartesian::builder()
                    .x_axis(
                        XAxis::builder()
                            .axis_line(CartesianAxisLine::builder().show(false).build())
                            .axis_type(CartesianAxis::Category(bon::vec![
                                "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun",
                            ]))
                            .build(),
                    )
                    .y_axis(YAxis::builder().axis_type(CartesianAxis::Values).build())
                    .data(vec![150.0, 230.0, 224.0, 218.0, 135.0, 147.0, 260.0])
                    .build(),
            ))
            .build();

        println!("{:?}", instant.elapsed());
        let renderer = SvgRenderer::new();
        renderer.save(&chart, "line.svg").unwrap();
        assert!(false)
    }
}
