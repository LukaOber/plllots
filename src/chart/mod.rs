//! Main chart structure and building functionality.

pub mod builder;

use crate::component::AxisHelper;
use crate::element::{Margins, Offsets, PlotSize};
pub use builder::*;

/// Helper structure for chart plotting operations.
#[derive(Debug, Clone)]
pub struct ChartPlotHelper {
    pub plot_size: PlotSize,
    pub margins: Margins,
    pub offsets: Offsets,
    pub y_axis: Option<AxisHelper>,
    pub x_axis: Option<AxisHelper>,
}
