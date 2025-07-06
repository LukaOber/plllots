use kurbo::{Point, Stroke};
use parley::Alignment;
use peniko::Brush;

use crate::{
    chart::Theme,
    renderer::{AppendSvg, AppendVello},
};

#[derive(Debug, Clone)]
pub enum Primitives<'a> {
    Line(Line<'a>),
    Text(Text<'a>),
    Path(Path<'a>),
    Circle(Circle<'a>),
    MultiCircle(MultiCircle<'a>),
}

impl AppendSvg for Primitives<'_> {
    fn append_svg(&self, doc: &mut svg::Document) {
        match self {
            Primitives::Line(line) => line.append_svg(doc),
            Primitives::Text(text) => text.append_svg(doc),
            Primitives::Path(path) => path.append_svg(doc),
            Primitives::Circle(circle) => circle.append_svg(doc),
            Primitives::MultiCircle(multi_circle) => multi_circle.append_svg(doc),
        }
    }
}

impl AppendVello for Primitives<'_> {
    fn append_vello(
        &self,
        scene: &mut vello::Scene,
        vello_render: &mut crate::renderer::VelloRenderer,
    ) {
        match self {
            Primitives::Line(line) => line.append_vello(scene, vello_render),
            Primitives::Text(text) => text.append_vello(scene, vello_render),
            Primitives::Path(path) => path.append_vello(scene, vello_render),
            Primitives::Circle(circle) => circle.append_vello(scene, vello_render),
            Primitives::MultiCircle(multi_circle) => multi_circle.append_vello(scene, vello_render),
        }
    }
}

pub trait AppendPrimitives<'a> {
    fn append_primitives(
        &'a self,
        primitives: &mut Vec<Primitives<'a>>,
        helper: &mut crate::chart::ChartHelper,
        theme: &'a Theme,
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
    pub rotation: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Path<'a> {
    pub stroke: &'a Stroke,
    pub stroke_color: &'a Brush,
    pub coords: Vec<Point>,
}

#[derive(Debug, Clone)]
pub struct Circle<'a> {
    pub stroke: &'a Stroke,
    pub stroke_color: &'a Brush,
    pub fill_color: &'a Brush,
    pub coord: Point,
    pub radius: f64,
}

#[derive(Debug, Clone)]
pub struct MultiCircle<'a> {
    pub stroke: &'a Stroke,
    pub stroke_color: &'a Brush,
    pub fill_color: &'a Brush,
    pub coords: &'a Vec<Point>,
    pub radius: f64,
}
