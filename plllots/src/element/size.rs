use bon::Builder;

/// Represents the size of a plot area.
#[derive(Debug, Clone, Copy, Builder)]
pub struct PlotSize {
    /// Width of the plot in pixels
    pub width: f64,
    /// Height of the plot in pixels  
    pub height: f64,
}

impl Default for PlotSize {
    fn default() -> Self {
        Self {
            width: 1000.0,
            height: 1000.0,
        }
    }
}
