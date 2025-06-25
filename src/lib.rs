pub(crate) mod utils;

use bon::{Builder, builder};
use svg::{
    Document,
    node::element::{Path, Rectangle, SVG, path::Data},
};

#[derive(Debug, Clone, Builder)]
struct Chart {
    size: PlotSize,
    #[builder(default)]
    margins: Margins,
    x_axis: Axis,
    y_axis: Axis,
}

impl Chart {
    fn to_svg(&self) -> svg::Document {
        Document::new()
            .set("width", self.size.width)
            .set("height", self.size.height)
            .set("viewBox", (0, 0, self.size.width, self.size.height))
            .add(
                Rectangle::new()
                    .set("width", self.size.width)
                    .set("height", self.size.height)
                    .set("x", 0)
                    .set("y", 0)
                    .set("fill", "none"),
            )
    }
}

#[derive(Debug, Clone)]
struct ChartPlotHelper {
    plot_size: PlotSize,
    margins: Margins,
    offsets: Offsets,
}

#[derive(Debug, Clone, Copy)]
struct PlotSize {
    width: f64,
    height: f64,
}

impl Default for PlotSize {
    fn default() -> Self {
        Self {
            width: 1000.0,
            height: 1000.0,
        }
    }
}

#[derive(Debug, Builder, Clone)]
struct Margins {
    left: MarginType,
    top: MarginType,
    right: MarginType,
    bottom: MarginType,
}

impl Default for Margins {
    fn default() -> Self {
        Self {
            left: MarginType::Percentage(10.0),
            top: MarginType::Pixel(60.0),
            right: MarginType::Percentage(10.0),
            bottom: MarginType::Pixel(60.0),
        }
    }
}

#[derive(Debug, Clone)]
enum MarginType {
    Pixel(f64),
    Percentage(f64),
}

#[derive(Debug, Clone)]
struct Offsets {
    x_axis_start: f64,
    x_axis_end: f64,
    x_span: f64,
    y_axis_start: f64,
    y_axis_end: f64,
    y_span: f64,
}

impl Offsets {
    fn from_margin(plot_size: &PlotSize, margins: &Margins) -> Offsets {
        let x_axis_start = match margins.left {
            MarginType::Pixel(pixel) => pixel,
            MarginType::Percentage(perc) => plot_size.width * (perc / 100.0),
        };
        let x_axis_end = match margins.right {
            MarginType::Pixel(pixel) => plot_size.width - pixel,
            MarginType::Percentage(perc) => plot_size.width - plot_size.width * (perc / 100.0),
        };
        let x_span = x_axis_end - x_axis_start;

        let y_axis_start = match margins.top {
            MarginType::Pixel(pixel) => pixel,
            MarginType::Percentage(perc) => plot_size.height * (perc / 100.0),
        };
        let y_axis_end = match margins.bottom {
            MarginType::Pixel(pixel) => plot_size.height - pixel,
            MarginType::Percentage(perc) => plot_size.height - plot_size.height * (perc / 100.0),
        };
        let y_span = y_axis_end - y_axis_start;

        Offsets {
            x_axis_start,
            x_axis_end,
            x_span,
            y_axis_start,
            y_axis_end,
            y_span,
        }
    }
}

#[derive(Debug, Builder, Clone)]
struct Axis {
    data: AxisData,
}

#[derive(Debug, Clone)]
enum AxisData {
    Category(Vec<String>),
    Values(Vec<f64>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let chart = Chart::builder()
            .size(PlotSize {
                width: 1000.0,
                height: 1000.0,
            })
            .x_axis(
                Axis::builder()
                    .data(AxisData::Category(bon::vec![
                        "Mon", "Tue", "Wed", "Thu", "Fri", "Sut", "Sun"
                    ]))
                    .build(),
            )
            .y_axis(
                Axis::builder()
                    .data(AxisData::Values(vec![
                        150.0, 230.0, 224.0, 218.0, 135.0, 147.0, 260.0,
                    ]))
                    .build(),
            )
            .build();

        let document = chart.to_svg();
        println!("{:#?}", document);
        println!("{:#?}", chart);
        assert!(false);
    }
}
