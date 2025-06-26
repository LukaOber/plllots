pub(crate) mod utils;

use bon::{Builder, builder};
use svg::{
    Document, Node,
    node::element::{Path, Rectangle, SVG, Text, path::Data},
};
use utils::calculate_axis_ticks;

trait AppendSvg {
    fn append_svg(&self, doc: &mut svg::Document, helper: &mut ChartPlotHelper);
}

#[derive(Debug, Clone, Builder)]
struct Chart {
    size: PlotSize,
    #[builder(default)]
    margins: Margins,
    x_axis: XAxis,
    y_axis: YAxis,
}

impl Chart {
    fn to_svg(&self) -> svg::Document {
        let mut helper = ChartPlotHelper {
            plot_size: self.size,
            margins: self.margins,
            offsets: Offsets::from_margin(&self.size, &self.margins),
            y_axis: None,
            x_axis: None,
        };

        let mut doc = Document::new()
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
            );

        self.y_axis.append_svg(&mut doc, &mut helper);

        doc
    }
}

#[derive(Debug, Clone)]
struct ChartPlotHelper {
    plot_size: PlotSize,
    margins: Margins,
    offsets: Offsets,
    y_axis: Option<AxisHelper>,
    x_axis: Option<AxisHelper>,
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

#[derive(Debug, Builder, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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
struct XAxis {
    data: AxisData,
}

#[derive(Debug, Builder, Clone)]
struct YAxis {
    data: AxisData,
}

#[derive(Debug, Clone)]
enum AxisHelper {
    Category(AxisCategoryHelper),
    Values(AxisValuesHelper),
}

#[derive(Debug, Clone)]
struct AxisCategoryHelper {
    amount: usize,
}

#[derive(Debug, Clone)]
struct AxisValuesHelper {
    min: f64,
    max: f64,
    step: f64,
}

impl AppendSvg for YAxis {
    fn append_svg(&self, doc: &mut svg::Document, helper: &mut ChartPlotHelper) {
        doc.append(svg::node::Comment::new("YAxis"));
        match &self.data {
            AxisData::Category(items) => todo!(),
            AxisData::Values(items) => {
                let (min, max, step_size) = calculate_axis_ticks(&items);
                let sub_tick_spacing = helper.offsets.y_span / (max / step_size);
                for sub_tick_index in 0..((max / step_size) as i32 + 1) {
                    let sub_tick_height =
                        helper.offsets.y_axis_end - (sub_tick_index as f64 * sub_tick_spacing);
                    println!("{}", sub_tick_index as f64 * sub_tick_spacing);
                    println!("{:?}", helper);
                    doc.append(
                        Path::new().set("stroke", "#E0E6F1").set(
                            "d",
                            Data::new()
                                .move_to((helper.offsets.x_axis_start, sub_tick_height))
                                .line_to((helper.offsets.x_axis_end, sub_tick_height)),
                        ),
                    );
                    doc.append(
                        Text::new(format!("{}", min + step_size * sub_tick_index as f64))
                            .set("dominant-baseline", "central")
                            .set("text-anchor", "end")
                            .set("style", "font-size:12px;font-family:sans-serif")
                            .set("fill", "#6E7079")
                            .set(
                                "transform",
                                format!(
                                    "translate({} {})",
                                    helper.offsets.x_axis_start - 8.0,
                                    sub_tick_height
                                ),
                            ),
                    );
                }
            }
        }
    }
}

impl AppendSvg for XAxis {
    fn append_svg(&self, doc: &mut svg::Document, helper: &mut ChartPlotHelper) {
        doc.append(svg::node::Comment::new("XAxis"));
        match &self.data {
            AxisData::Category(items) => todo!(),
            AxisData::Values(items) => todo!(),
        }
    }
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
                XAxis::builder()
                    .data(AxisData::Category(bon::vec![
                        "Mon", "Tue", "Wed", "Thu", "Fri", "Sut", "Sun"
                    ]))
                    .build(),
            )
            .y_axis(
                YAxis::builder()
                    .data(AxisData::Values(vec![
                        150.0, 230.0, 224.0, 218.0, 135.0, 147.0, 260.0,
                    ]))
                    .build(),
            )
            .build();

        let document = chart.to_svg();
        svg::save("line.svg", &document).unwrap();
        println!("{:#?}", document);
        println!("{:#?}", chart);
        assert!(false);
    }
}
