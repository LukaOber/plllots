use crate::chart::ChartPlotHelper;
use crate::component::{AxisData, AxisHelper};
use svg::{
    Document, Node,
    node::element::{Circle, Path},
};

/// Trait for rendering series data to SVG.
pub trait RenderSeries {
    fn render_to_svg(
        &self,
        doc: &mut Document,
        helper: &ChartPlotHelper,
        x_data: &AxisData,
        y_data: &AxisData,
    );
}

/// Line series renderer.
pub struct LineSeries;

impl RenderSeries for LineSeries {
    fn render_to_svg(
        &self,
        doc: &mut Document,
        helper: &ChartPlotHelper,
        x_data: &AxisData,
        y_data: &AxisData,
    ) {
        match (x_data, y_data) {
            (AxisData::Category(_x_items), AxisData::Category(_y_items)) => todo!(),
            (AxisData::Category(x_items), AxisData::Values(y_items)) => {
                let mut path = String::new();
                let mut symbols = Vec::new();

                for (index, (_x_item, y_item)) in x_items.iter().zip(y_items).enumerate() {
                    let y_pos = if let AxisHelper::Values(y_axis_helper) =
                        &helper.y_axis.as_ref().unwrap()
                    {
                        let percentage_height = y_item / y_axis_helper.max;
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
            (AxisData::Values(_x_items), AxisData::Category(_y_items)) => todo!(),
            (AxisData::Values(_x_items), AxisData::Values(_y_items)) => todo!(),
        }
    }
}
