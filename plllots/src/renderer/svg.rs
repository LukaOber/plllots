use kurbo::Cap;
use parley::Alignment;
use peniko::Brush;
use svg::node::element::path::Data;
use svg::node::element::{Circle, Path, Rectangle};
use svg::{Document, Node};

use crate::chart::Chart;
use std::io::Result;

pub struct SvgRenderer;

impl SvgRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, chart: &Chart) -> Document {
        let background = chart.theme.background.to_rgba8().to_u8_array();
        let background = format!(
            "#{:X}{:X}{:X}{:X}",
            background[0], background[1], background[2], background[3]
        );
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
                    .set("fill", background),
            );

        let primitives = chart.generate_primitives();
        for primitive in primitives {
            primitive.append_svg(&mut doc);
        }
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

impl AppendSvg for crate::primitives::Line<'_> {
    fn append_svg(&self, doc: &mut svg::Document) {
        let stroke_color = match &self.stroke_color {
            Brush::Solid(alpha_color) => {
                let colors = alpha_color.to_rgba8().to_u8_array();
                format!(
                    "#{:X}{:X}{:X}{:X}",
                    colors[0], colors[1], colors[2], colors[3]
                )
            }
            Brush::Gradient(_gradient) => todo!(),
            Brush::Image(_image) => todo!(),
        };
        let cap = match (self.stroke.start_cap, self.stroke.end_cap) {
            (Cap::Butt, Cap::Butt) => "butt",
            // (Cap::Butt, Cap::Square) => todo!(),
            // (Cap::Butt, Cap::Round) => todo!(),
            // (Cap::Square, Cap::Butt) => todo!(),
            (Cap::Square, Cap::Square) => "square",
            // (Cap::Square, Cap::Round) => todo!(),
            // (Cap::Round, Cap::Butt) => todo!(),
            // (Cap::Round, Cap::Square) => todo!(),
            (Cap::Round, Cap::Round) => "round",
            _ => todo!(),
        };

        doc.append(
            Path::new()
                .set("stroke", stroke_color)
                .set("stroke-width", self.stroke.width)
                .set("stroke-linecap", cap)
                .set(
                    "d",
                    Data::new()
                        .move_to((self.coords.0.x, self.coords.0.y))
                        .line_to((self.coords.1.x, self.coords.1.y)),
                ),
        );
    }
}

impl AppendSvg for crate::primitives::Text<'_> {
    fn append_svg(&self, doc: &mut svg::Document) {
        let fill_color = match &self.fill_color {
            Brush::Solid(alpha_color) => {
                let colors = alpha_color.to_rgba8().to_u8_array();
                format!(
                    "#{:X}{:X}{:X}{:X}",
                    colors[0], colors[1], colors[2], colors[3]
                )
            }
            Brush::Gradient(_gradient) => todo!(),
            Brush::Image(_image) => todo!(),
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
        let transform = match self.rotation {
            Some(r) => format!(
                "matrix({}, {}, {}, {}, {}, {})",
                r.cos(),
                r.sin(),
                -r.sin(),
                r.cos(),
                self.coord.x,
                self.coord.y
            ),
            None => format!("translate({} {})", self.coord.x, self.coord.y),
        };

        doc.append(
            svg::node::element::Text::new(&self.text)
                .set("dominant-baseline", "central")
                .set("text-anchor", text_anchor)
                .set("style", style)
                .set("fill", fill_color)
                .set("transform", transform),
        );
    }
}

impl AppendSvg for crate::primitives::Path<'_> {
    fn append_svg(&self, doc: &mut svg::Document) {
        let stroke_color = match &self.stroke_color {
            Brush::Solid(alpha_color) => {
                let colors = alpha_color.to_rgba8().to_u8_array();
                format!(
                    "#{:X}{:X}{:X}{:X}",
                    colors[0], colors[1], colors[2], colors[3]
                )
            }
            Brush::Gradient(_gradient) => todo!(),
            Brush::Image(_image) => todo!(),
        };

        let mut path = Data::new();
        for (index, point) in self.coords.iter().enumerate() {
            if index == 0 {
                path = path.move_to((point.x, point.y));
            } else {
                path = path.line_to((point.x, point.y));
            }
        }

        doc.append(
            Path::new()
                .set("stroke", stroke_color)
                .set("stroke-width", self.stroke.width)
                .set("fill", "transparent")
                .set("d", path),
        );
    }
}

impl AppendSvg for crate::primitives::Circle<'_> {
    fn append_svg(&self, doc: &mut Document) {
        let stroke_color = match &self.stroke_color {
            Brush::Solid(alpha_color) => {
                let colors = alpha_color.to_rgba8().to_u8_array();
                format!(
                    "#{:X}{:X}{:X}{:X}",
                    colors[0], colors[1], colors[2], colors[3]
                )
            }
            Brush::Gradient(_gradient) => todo!(),
            Brush::Image(_image) => todo!(),
        };

        let fill_color = match &self.fill_color {
            Brush::Solid(alpha_color) => {
                let colors = alpha_color.to_rgba8().to_u8_array();
                format!(
                    "#{:X}{:X}{:X}{:X}",
                    colors[0], colors[1], colors[2], colors[3]
                )
            }
            Brush::Gradient(_gradient) => todo!(),
            Brush::Image(_image) => todo!(),
        };

        doc.append(
            Circle::new()
                .set("r", self.radius)
                .set("cx", self.coord.x)
                .set("cy", self.coord.y)
                .set("fill", fill_color)
                .set("stroke", stroke_color)
                .set("stroke-width", self.stroke.width),
        )
    }
}

impl AppendSvg for crate::primitives::MultiCircle<'_> {
    fn append_svg(&self, doc: &mut Document) {
        let stroke_color = match &self.stroke_color {
            Brush::Solid(alpha_color) => {
                let colors = alpha_color.to_rgba8().to_u8_array();
                format!(
                    "#{:X}{:X}{:X}{:X}",
                    colors[0], colors[1], colors[2], colors[3]
                )
            }
            Brush::Gradient(_gradient) => todo!(),
            Brush::Image(_image) => todo!(),
        };

        let fill_color = match &self.fill_color {
            Brush::Solid(alpha_color) => {
                let colors = alpha_color.to_rgba8().to_u8_array();
                format!(
                    "#{:X}{:X}{:X}{:X}",
                    colors[0], colors[1], colors[2], colors[3]
                )
            }
            Brush::Gradient(_gradient) => todo!(),
            Brush::Image(_image) => todo!(),
        };

        for coord in &self.coords {
            doc.append(
                Circle::new()
                    .set("r", self.radius)
                    .set("cx", coord.x)
                    .set("cy", coord.y)
                    .set("fill", fill_color.clone())
                    .set("stroke", stroke_color.clone())
                    .set("stroke-width", self.stroke.width),
            );
        }
    }
}
