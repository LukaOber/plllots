use super::size::PlotSize;
use bon::Builder;

/// Represents different types of margins that can be applied to a chart.
#[derive(Debug, Clone, Copy)]
pub enum MarginType {
    /// Margin specified in pixels
    Pixel(f64),
    /// Margin specified as a percentage of the chart dimension
    Percentage(f64),
}

/// Represents the margins around a chart.
#[derive(Debug, Builder, Clone, Copy)]
pub struct Margins {
    /// Left margin
    pub left: MarginType,
    /// Top margin
    pub top: MarginType,
    /// Right margin
    pub right: MarginType,
    /// Bottom margin
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

/// Calculated offset values for positioning chart elements.
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
    /// Calculate offsets from plot size and margins.
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
