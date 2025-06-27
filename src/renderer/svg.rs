use crate::chart::Chart;
use std::io::Result;
use std::path::Path;

/// SVG renderer for charts.
pub struct SvgRenderer;

impl SvgRenderer {
    /// Create a new SVG renderer.
    pub fn new() -> Self {
        Self
    }

    /// Render a chart to SVG string.
    pub fn render(&self, chart: &Chart) -> String {
        chart.to_svg().to_string()
    }

    /// Save a chart as SVG file.
    pub fn save<P: AsRef<Path>>(&self, chart: &Chart, path: P) -> Result<()> {
        let doc = chart.to_svg();
        svg::save(path, &doc)
    }
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self::new()
    }
}
