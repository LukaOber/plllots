use crate::utils::calculate_axis_ticks;
use bon::Builder;
use svg::{
    Document, Node,
    node::element::{Path, Text, path::Data},
};

/// Data that can be displayed on an axis.
#[derive(Debug, Clone)]
pub enum AxisData {
    /// Categorical data (labels)
    Category(Vec<String>),
    /// Numerical data (values)
    Values(Vec<f64>),
}

/// X-axis configuration and data.
#[derive(Debug, Builder, Clone)]
pub struct XAxis {
    pub data: AxisData,
}

/// Y-axis configuration and data.
#[derive(Debug, Builder, Clone)]
pub struct YAxis {
    pub data: AxisData,
}

/// Helper structures for axis calculations.
#[derive(Debug, Clone)]
pub enum AxisHelper {
    Category(AxisCategoryHelper),
    Values(AxisValuesHelper),
}

/// Helper for categorical axis calculations.
#[derive(Debug, Clone)]
pub struct AxisCategoryHelper {
    pub amount: usize,
}

/// Helper for numerical axis calculations.
#[derive(Debug, Clone)]
pub struct AxisValuesHelper {
    pub min: f64,
    pub max: f64,
    pub step_size: f64,
}

/// Trait for appending SVG elements to a document.
pub trait AppendSvg {
    fn append_svg(&self, doc: &mut Document, helper: &mut crate::chart::ChartPlotHelper);
}

impl AppendSvg for YAxis {
    fn append_svg(&self, doc: &mut Document, helper: &mut crate::chart::ChartPlotHelper) {
        match &self.data {
            AxisData::Category(_items) => todo!(),
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
    fn append_svg(&self, doc: &mut Document, helper: &mut crate::chart::ChartPlotHelper) {
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
            AxisData::Values(_items) => todo!(),
        }
    }
}
