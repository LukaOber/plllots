use super::ChartHelper;
use crate::coordinate_system::CoordinateSystem;
use crate::element::{Margins, Offsets, PlotSize};
use crate::primitives::{AppendPrimitives, Primitives};
use bon::Builder;
use kurbo::{Cap, Stroke};
use peniko::{Brush, Color};

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct Chart {
    #[builder(with = |width: f64, height: f64| PlotSize { width, height })]
    pub size: PlotSize,
    #[builder(default, setters(option_fn(vis = "")))]
    pub margins: Margins,
    pub coordinate_system: CoordinateSystem,
    #[builder(default = Theme::white(), setters(option_fn(vis = "")))]
    pub theme: Theme,
}

impl Chart {
    pub(crate) fn create_plot_helper(&self) -> ChartHelper {
        ChartHelper {
            plot_size: self.size.clone(),
            margins: self.margins.clone(),
            offsets: Offsets::from_margin(&self.size, &self.margins),
        }
    }

    pub(crate) fn generate_primitives(&self) -> Vec<Primitives> {
        let mut helper = self.create_plot_helper();
        let mut primitives = Vec::new();
        self.coordinate_system
            .append_primitives(&mut primitives, &mut helper, &self.theme);
        primitives
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub background: Color,
    pub cartesian_category_axis: CartesianAxisTheme,
    pub cartesian_value_axis: CartesianAxisTheme,
    pub line: LineTheme,
    pub scatter: ScatterTheme,
    pub series_colors: Vec<Brush>,
}

impl Theme {
    fn white() -> Self {
        Self {
            background: Color::from_rgba8(0xff, 0xff, 0xff, 0xff),
            cartesian_category_axis: CartesianAxisTheme {
                axis_show: true,
                axis_stroke: Stroke::new(1.0)
                    .with_start_cap(Cap::Square)
                    .with_end_cap(Cap::Square),
                axis_color: Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)),
                axis_auto_offset: 20.0,
                ticks_show: true,
                ticks_length: 5.0,
                ticks_stroke: Stroke::new(1.0)
                    .with_start_cap(Cap::Square)
                    .with_end_cap(Cap::Square),
                ticks_color: Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)),
                split_lines_show: false,
                split_lines_color: Brush::Solid(Color::from_rgba8(0xe0, 0xe6, 0xe1, 0xff)),
                split_lines_stroke: Stroke::new(1.0)
                    .with_start_cap(Cap::Square)
                    .with_end_cap(Cap::Square),
                labels_show: true,
                labels_margin: 8.0,
                labels_color: Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)),
                labels_font_size: 12.0,
            },
            cartesian_value_axis: CartesianAxisTheme {
                axis_show: false,
                axis_stroke: Stroke::new(1.0)
                    .with_start_cap(Cap::Square)
                    .with_end_cap(Cap::Square),
                axis_color: Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)),
                axis_auto_offset: 20.0,
                ticks_show: true,
                ticks_length: 5.0,
                ticks_stroke: Stroke::new(1.0)
                    .with_start_cap(Cap::Square)
                    .with_end_cap(Cap::Square),
                ticks_color: Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)),
                split_lines_show: true,
                split_lines_color: Brush::Solid(Color::from_rgba8(0xe0, 0xe6, 0xe1, 0xff)),
                split_lines_stroke: Stroke::new(1.0)
                    .with_start_cap(Cap::Square)
                    .with_end_cap(Cap::Square),
                labels_show: true,
                labels_margin: 8.0,
                labels_color: Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)),
                labels_font_size: 12.0,
            },
            line: LineTheme {
                stroke: Stroke::new(2.0),
                symbol_show: true,
                symbol_stroke: Stroke::new(2.0),
                symbol_fill_color: Brush::Solid(Color::from_rgba8(0xff, 0xff, 0xff, 0xff)),
                symbol_size: 2.0,
            },
            scatter: ScatterTheme {
                stroke: Stroke::new(0.0),
                stroke_color: Brush::Solid(Color::from_rgba8(0xee, 0xee, 0xee, 0xff)),
                symbol_size: 10.0,
            },
            series_colors: vec![
                Brush::Solid(Color::from_rgba8(0x54, 0x70, 0xc6, 0xff)),
                Brush::Solid(Color::from_rgba8(0x91, 0xcc, 0x75, 0xff)),
                Brush::Solid(Color::from_rgba8(0xfa, 0xc8, 0x58, 0xff)),
                Brush::Solid(Color::from_rgba8(0xee, 0x66, 0x66, 0xff)),
                Brush::Solid(Color::from_rgba8(0x73, 0xc0, 0xde, 0xff)),
                Brush::Solid(Color::from_rgba8(0x3b, 0xa2, 0x72, 0xff)),
                Brush::Solid(Color::from_rgba8(0xfc, 0x84, 0x52, 0xff)),
                Brush::Solid(Color::from_rgba8(0x9a, 0x60, 0xb4, 0xff)),
                Brush::Solid(Color::from_rgba8(0xea, 0x7c, 0xcc, 0xff)),
                Brush::Solid(Color::from_rgba8(0x17, 0xa2, 0xb8, 0xff)),
                Brush::Solid(Color::from_rgba8(0xf3, 0x9c, 0x12, 0xff)),
                Brush::Solid(Color::from_rgba8(0x26, 0xde, 0x81, 0xff)),
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CartesianAxisTheme {
    pub axis_show: bool,
    pub axis_stroke: Stroke,
    pub axis_color: Brush,
    pub axis_auto_offset: f64,
    pub ticks_show: bool,
    pub ticks_length: f64,
    pub ticks_stroke: Stroke,
    pub ticks_color: Brush,
    pub split_lines_show: bool,
    pub split_lines_color: Brush,
    pub split_lines_stroke: Stroke,
    pub labels_show: bool,
    pub labels_margin: f64,
    pub labels_color: Brush,
    pub labels_font_size: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineTheme {
    pub stroke: Stroke,
    pub symbol_show: bool,
    pub symbol_stroke: Stroke,
    pub symbol_fill_color: Brush,
    pub symbol_size: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScatterTheme {
    pub stroke: Stroke,
    pub stroke_color: Brush,
    pub symbol_size: f64,
}
