use svg::Document;
use svg::node::element::Rectangle;

use crate::chart::Chart;
use crate::primitives::AppendPrimitives;
use crate::{LineSeries, RenderSeries};
use std::io::Result;
use std::path::Path;

pub struct SvgRenderer;

impl SvgRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, chart: &Chart) -> Document {
        let mut doc = Document::new()
            .set("width", chart.size.width)
            .set("height", chart.size.height)
            .set("viewBox", (0, 0, chart.size.width, chart.size.height))
            .add(
                Rectangle::new()
                    .set("width", chart.size.width)
                    .set("height", chart.size.height)
                    .set("x", 0)
                    .set("y", 0)
                    .set("fill", "none"),
            );

        // Render axes
        let primitives = chart.generate_primitives();
        let helper = chart.create_plot_helper();

        for primitive in primitives {
            primitive.append_svg(&mut doc);
        }
        // Render series data
        let line_series = LineSeries;
        line_series.render_to_svg(&mut doc, &helper, &chart.x_axis.data, &chart.y_axis.data);
        doc
    }

    pub fn save<P: AsRef<Path>>(&self, chart: &Chart, path: P) -> Result<()> {
        let doc = self.render(chart);
        svg::save(path, &doc)
    }
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self::new()
    }
}

pub trait AppendSvg {
    fn append_svg(&self, doc: &mut Document);
}
