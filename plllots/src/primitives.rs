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
    pub coord: Point,
}
