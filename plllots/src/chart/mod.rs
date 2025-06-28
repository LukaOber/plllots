pub mod builder;

use crate::element::{Margins, Offsets, PlotSize};
pub use builder::*;

#[derive(Debug, Clone)]
pub struct ChartHelper {
    pub plot_size: PlotSize,
    pub margins: Margins,
    pub offsets: Offsets,
}
