use super::size::PlotSize;
use bon::Builder;

#[derive(Debug, Clone, Copy)]
pub enum MarginType {
    Pixel(f64),
    Percentage(f64),
}

#[derive(Debug, Builder, Clone, Copy)]
pub struct Margins {
    pub left: MarginType,
    pub top: MarginType,
    pub right: MarginType,
    pub bottom: MarginType,
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

#[derive(Debug, Clone)]
pub struct Offsets {
    pub x_axis_start: f64,
    pub x_axis_end: f64,
    pub x_span: f64,
    pub y_axis_start: f64,
    pub y_axis_end: f64,
    pub y_span: f64,
}

impl Offsets {
    pub fn from_margin(plot_size: &PlotSize, margins: &Margins) -> Offsets {
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
