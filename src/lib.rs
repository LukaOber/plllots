pub(crate) mod utils;

use bon::{Builder, builder};
use svg::{
    Document, Node,
    node::element::{Circle, Path, Rectangle, SVG, Text, path::Data},
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

        self.x_axis.append_svg(&mut doc, &mut helper);
        self.y_axis.append_svg(&mut doc, &mut helper);

        match (&self.x_axis.data, &self.y_axis.data) {
            (AxisData::Category(x_items), AxisData::Category(y_items)) => todo!(),
            (AxisData::Category(x_items), AxisData::Values(y_items)) => {
                let mut path = String::new();
                let mut symbols = Vec::new();
                for (index, (x_item, y_item)) in x_items.iter().zip(y_items).enumerate() {
                    let y_pos = if let AxisHelper::Values(y_axis_helper) =
                        &helper.y_axis.as_ref().unwrap()
                    {
                        let percentage_height = (y_item / y_axis_helper.max);
                        helper.offsets.y_axis_end - (percentage_height * helper.offsets.y_span)
                    } else {
                        unreachable!()
                    };

                    let x_spacing = helper.offsets.x_span / x_items.len() as f64;
                    let x_pos = helper.offsets.x_axis_start + (index as f64 + 0.5) * x_spacing;

                    symbols.push(
                        Circle::new()
                            .set("r", 2)
                            .set("fill", "#ffffff")
                            .set("stroke", "#5470c6")
                            .set("stroke-width", 2)
                            .set("cx", x_pos)
                            .set("cy", y_pos),
                    );

                    if index == 0 {
                        path.push_str(&format!("M{x_pos} {y_pos}"));
                    } else {
                        path.push_str(&format!("L{x_pos} {y_pos}"));
                    }
                }

                doc.append(svg::node::Comment::new("Data line"));
                doc.append(
                    Path::new()
                        .set("d", path)
                        .set("fill", "transparent")
                        .set("stroke", "#5470c6")
                        .set("stroke-width", 2)
                        .set("linejoin", "bevel"),
                );

                doc.append(svg::node::Comment::new("Data symbols"));
                for symbol in symbols {
                    doc.append(symbol);
                }
            }
            (AxisData::Values(x_items), AxisData::Category(y_items)) => todo!(),
            (AxisData::Values(x_items), AxisData::Values(y_items)) => todo!(),
        }

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
    step_size: f64,
}

impl AppendSvg for YAxis {
    fn append_svg(&self, doc: &mut svg::Document, helper: &mut ChartPlotHelper) {
        match &self.data {
            AxisData::Category(items) => todo!(),
            AxisData::Values(items) => {
                let (min, max, step_size) = calculate_axis_ticks(&items);
                let sub_tick_spacing = helper.offsets.y_span / (max / step_size);
                doc.append(svg::node::Comment::new("YAxis: Subticks"));
                for sub_tick_index in 1..((max / step_size) as i32 + 1) {
                    let sub_tick_height =
                        helper.offsets.y_axis_end - (sub_tick_index as f64 * sub_tick_spacing);
                    doc.append(
                        Path::new().set("stroke", "#E0E6F1").set(
                            "d",
                            Data::new()
                                .move_to((helper.offsets.x_axis_start, sub_tick_height))
                                .line_to((helper.offsets.x_axis_end, sub_tick_height)),
                        ),
                    );
                }

                doc.append(svg::node::Comment::new("YAxis: Labels"));
                for sub_tick_index in 0..((max / step_size) as i32 + 1) {
                    let sub_tick_height =
                        helper.offsets.y_axis_end - (sub_tick_index as f64 * sub_tick_spacing);
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

                helper.y_axis = Some(AxisHelper::Values(AxisValuesHelper {
                    min,
                    max,
                    step_size,
                }))
            }
        }
    }
}

impl AppendSvg for XAxis {
    fn append_svg(&self, doc: &mut svg::Document, helper: &mut ChartPlotHelper) {
        match &self.data {
            AxisData::Category(items) => {
                doc.append(svg::node::Comment::new("XAxis"));
                doc.append(
                    Path::new()
                        .set("stroke", "#6E7079")
                        .set("stroke-linecap", "square")
                        .set(
                            "d",
                            Data::new()
                                .move_to((helper.offsets.x_axis_start, helper.offsets.y_axis_end))
                                .line_to((helper.offsets.x_axis_end, helper.offsets.y_axis_end)),
                        ),
                );

                let label_spacing = helper.offsets.x_span / items.len() as f64;
                doc.append(svg::node::Comment::new("XAxis: Subticks"));
                for label_index in 0..=items.len() {
                    let x_pos = helper.offsets.x_axis_start + label_index as f64 * label_spacing;
                    let y_pos = helper.offsets.y_axis_end + 5.0;
                    doc.append(
                        Path::new().set("stroke", "#6E7079").set(
                            "d",
                            Data::new()
                                .move_to((x_pos, helper.offsets.y_axis_end))
                                .line_to((x_pos, y_pos)),
                        ),
                    );
                }

                doc.append(svg::node::Comment::new("XAxis: Labels"));
                for (label_index, label) in items.iter().enumerate() {
                    let x_pos =
                        helper.offsets.x_axis_start + (label_index as f64 + 0.5) * label_spacing;
                    let y_pos = helper.offsets.y_axis_end + 14.0;
                    doc.append(
                        Text::new(format!("{label}"))
                            .set("dominant-baseline", "central")
                            .set("text-anchor", "middle")
                            .set("style", "font-size:12px;font-family:sans-serif")
                            .set("fill", "#6E7079")
                            .set("transform", format!("translate({} {})", x_pos, y_pos)),
                    );
                }

                helper.x_axis = Some(AxisHelper::Category(AxisCategoryHelper {
                    amount: items.len(),
                }))
            }
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
    use std::time::Instant;

    use super::*;

    #[test]
    fn it_works() {
        let instant = Instant::now();
        let chart = Chart::builder()
            .size(PlotSize {
                width: 1000.0,
                height: 1000.0,
            })
            .x_axis(
                XAxis::builder()
                    .data(AxisData::Category(bon::vec![
                        "Mon", "Tue", "Wed", "Thu", "Fri", "Sut", "Sun",
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
        println!("{:?}", instant.elapsed());
        svg::save("line.svg", &document).unwrap();
        assert!(false);
    }
}
