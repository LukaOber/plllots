use super::ChartHelper;
use crate::coordinate_system::CoordinateSystem;
use crate::element::{Margins, Offsets, PlotSize};
use crate::primitives::{AppendPrimitives, Primitives};
use bon::Builder;

#[derive(Debug, Clone, Builder)]
pub struct Chart {
    #[builder(with = |width: f64, height: f64| PlotSize { width, height })]
    pub size: PlotSize,
    #[builder(default)]
    pub margins: Margins,
    pub coordinate_system: CoordinateSystem,
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
            .append_primitives(&mut primitives, &mut helper);
        primitives
    }
}
