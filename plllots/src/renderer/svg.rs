use parley::Alignment;
use peniko::Brush;
use svg::node::element::path::Data;
use svg::node::element::{Path, Rectangle};
use svg::{Document, Node};

use crate::chart::Chart;
use crate::component::AxisHelper;
use crate::primitives::AppendPrimitives;
use crate::{CartesianAxis, LineSeries, RenderSeries};
use std::io::Result;

pub struct SvgRenderer;

impl SvgRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, chart: &Chart) -> Document {
        let mut doc = Document::new()
            .set("width", chart.size.width)
            .set("height", chart.size.height)
            .set("viewBox", (0, 0, chart.size.width, chart.size.height))
            .add(
                Rectangle::new()
                    .set("width", chart.size.width)
                    .set("height", chart.size.height)
                    .set("x", 0)
                    .set("y", 0)
                    .set("fill", "none"),
            );

        // Render axes
        let primitives = chart.generate_primitives();
        let mut helper = chart.create_plot_helper();
        for primitive in primitives {
            // println!("{:#?}", primitive);
            primitive.append_svg(&mut doc);
        }

        // // Render series data
        // let line_series = LineSeries;
        // line_series.render_to_svg(&mut doc, &helper, &chart.x_axis.data, &chart.y_axis.data);
        doc
    }

    pub fn save<P: AsRef<std::path::Path>>(&self, chart: &Chart, path: P) -> Result<()> {
        let doc = self.render(chart);
        svg::save(path, &doc)
    }
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self::new()
    }
}

pub trait AppendSvg {
    fn append_svg(&self, doc: &mut Document);
}

impl<'a> AppendSvg for crate::primitives::Line<'a> {
    fn append_svg(&self, doc: &mut svg::Document) {
        let stroke_color = match &self.stroke_color {
            Brush::Solid(alpha_color) => {
                let colors = alpha_color.to_rgba8().to_u8_array();
                format!(
                    "#{:X}{:X}{:X}{:X}",
                    colors[0], colors[1], colors[2], colors[3]
                )
            }
            Brush::Gradient(gradient) => todo!(),
            Brush::Image(image) => todo!(),
        };
        doc.append(
            Path::new()
                .set("stroke", stroke_color)
                .set("stroke-width", self.stroke.width)
                .set(
                    "d",
                    Data::new()
                        .move_to((self.coords.0.x, self.coords.0.y))
                        .line_to((self.coords.1.x, self.coords.1.y)),
                ),
        );
    }
}

impl<'a> AppendSvg for crate::primitives::Text<'a> {
    fn append_svg(&self, doc: &mut svg::Document) {
        let fill_color = match &self.fill_color {
            Brush::Solid(alpha_color) => {
                let colors = alpha_color.to_rgba8().to_u8_array();
                format!(
                    "#{:X}{:X}{:X}{:X}",
                    colors[0], colors[1], colors[2], colors[3]
                )
            }
            Brush::Gradient(gradient) => todo!(),
            Brush::Image(image) => todo!(),
        };

        let text_anchor = match self.text_anchor {
            Alignment::Start => "start",
            Alignment::End => "end",
            Alignment::Left => "left",
            Alignment::Middle => "middle",
            Alignment::Right => "right",
            Alignment::Justified => "justified",
        };

        let style = format!("font-size:{}px;font-family:sans-serif", self.font_size);

        doc.append(
            svg::node::element::Text::new(&self.text)
                .set("dominant-baseline", "central")
                .set("text-anchor", text_anchor)
                .set("style", style)
                .set("fill", fill_color)
                .set(
                    "transform",
                    format!("translate({} {})", self.coord.x, self.coord.y),
                ),
        );
    }
}
