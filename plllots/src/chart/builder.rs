use super::ChartPlotHelper;
use crate::component::{XAxis, YAxis};
use crate::coordinate_system::CoordinateSystem;
use crate::element::{Margins, Offsets, PlotSize};
use crate::primitives::{AppendPrimitives, Primitives};
use crate::renderer::AppendSvg;
use crate::series::{LineSeries, RenderSeries};
use bon::Builder;
use svg::{Document, node::element::Rectangle};

/// Main chart structure with builder pattern support.
#[derive(Debug, Clone, Builder)]
pub struct Chart {
    pub size: PlotSize,
    #[builder(default)]
    pub margins: Margins,
    pub coordinate_system: CoordinateSystem,
}

impl Chart {
    pub(crate) fn create_plot_helper(&self) -> ChartPlotHelper {
        ChartPlotHelper {
            plot_size: self.size,
            margins: self.margins,
            offsets: Offsets::from_margin(&self.size, &self.margins),
            y_axis: None,
            x_axis: None,
        }
    }

    pub(crate) fn generate_primitives(&self) -> Vec<Primitives> {
        let mut helper = self.create_plot_helper();
        let mut primitives = Vec::new();
        self.coordinate_system
            .append_primitives(&mut primitives, &mut helper);
        primitives
    }
}
