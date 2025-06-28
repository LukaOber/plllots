use std::fmt::Display;

use kurbo::{Point, Stroke};
use parley::Alignment;
use peniko::Brush;
use svg::{
    Node,
    node::element::{Path, path::Data},
};

use crate::renderer::AppendSvg;

#[derive(Debug, Clone)]
pub enum Primitives<'a> {
    Line(Line<'a>),
    Text(Text<'a>),
}

impl<'a> AppendSvg for Primitives<'a> {
    fn append_svg(&self, doc: &mut svg::Document) {
        match self {
            Primitives::Line(line) => line.append_svg(doc),
            Primitives::Text(text) => text.append_svg(doc),
        }
    }
}

pub trait AppendPrimitives<'a> {
    fn append_primitives(
        &'a self,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &mut crate::chart::ChartPlotHelper,
    );
}

#[derive(Debug, Clone)]
pub struct Line<'a> {
    pub stroke: &'a Stroke,
    pub stroke_color: &'a Brush,
    pub coords: (Point, Point),
}

#[derive(Debug, Clone)]
pub struct Text<'a> {
    pub text: String,
    pub fill_color: &'a Brush,
    pub font_size: f64,
    pub text_anchor: Alignment,
    pub translation: Point,
}

impl<'a> AppendSvg for Line<'a> {
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

impl<'a> AppendSvg for Text<'a> {
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
                    format!("translate({} {})", self.translation.x, self.translation.y),
                ),
        );
    }
}
