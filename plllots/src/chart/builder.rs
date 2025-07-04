use super::ChartHelper;
use crate::coordinate_system::CoordinateSystem;
use crate::element::{Margins, Offsets, PlotSize};
use crate::primitives::{AppendPrimitives, Primitives};
use bon::Builder;
use kurbo::{Cap, Stroke};
use peniko::{Brush, Color};

#[derive(Debug, Clone, Builder)]
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
            plot_size: self.size,
            margins: self.margins,
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

#[derive(Debug, Clone)]
pub struct Theme {
    pub background: Color,
    pub cartesian_category_axis: CartesianAxisTheme,
    pub cartesian_value_axis: CartesianAxisTheme,
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
                labels_margin: 14.0,
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
        }
    }
}

#[derive(Debug, Clone)]
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
