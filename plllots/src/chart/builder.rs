use super::ChartPlotHelper;
use crate::component::{AppendSvg, XAxis, YAxis};
use crate::element::{Margins, Offsets, PlotSize};
use crate::series::{LineSeries, RenderSeries};
use bon::Builder;
use svg::{Document, node::element::Rectangle};

/// Main chart structure with builder pattern support.
#[derive(Debug, Clone, Builder)]
pub struct Chart {
    /// Size of the chart
    pub size: PlotSize,
    /// Margins around the chart
    #[builder(default)]
    pub margins: Margins,
    /// X-axis configuration
    pub x_axis: XAxis,
    /// Y-axis configuration
    pub y_axis: YAxis,
}

impl Chart {
    /// Convert the chart to an SVG document.
    pub fn to_svg(&self) -> Document {
        let mut helper = ChartPlotHelper {
            plot_size: self.size,
            margins: self.margins,
            offsets: Offsets::from_margin(&self.size, &self.margins),
            y_axis: None,
            x_axis: None,
        };

        let mut doc = Document::new()
            .set("width", self.size.width)
            .set("height", self.size.height)
            .set("viewBox", (0, 0, self.size.width, self.size.height))
            .add(
                Rectangle::new()
                    .set("width", self.size.width)
                    .set("height", self.size.height)
                    .set("x", 0)
                    .set("y", 0)
                    .set("fill", "none"),
            );

        // Render axes
        self.x_axis.append_svg(&mut doc, &mut helper);
        self.y_axis.append_svg(&mut doc, &mut helper);

        // Render series data
        let line_series = LineSeries;
        line_series.render_to_svg(&mut doc, &helper, &self.x_axis.data, &self.y_axis.data);

        doc
    }
}
