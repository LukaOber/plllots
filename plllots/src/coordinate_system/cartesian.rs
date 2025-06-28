use bon::Builder;

use crate::{XAxis, YAxis, primitives::AppendPrimitives};

#[derive(Debug, Builder, Clone)]
pub struct Cartesian {
    pub x_axis: XAxis,
    pub y_axis: YAxis,
}

impl<'a> AppendPrimitives<'a> for Cartesian {
    fn append_primitives(
        &'a self,
        primitives: &mut Vec<crate::primitives::Primitives<'a>>,
        helper: &mut crate::chart::ChartPlotHelper,
    ) {
        self.x_axis.append_primitives(primitives, helper);
        self.y_axis.append_primitives(primitives, helper);
    }
}
