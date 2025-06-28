pub mod cartesian;

pub use cartesian::*;

use crate::primitives::AppendPrimitives;

#[derive(Debug, Clone)]
pub enum CoordinateSystem {
    Cartesian(Cartesian),
}

impl<'a> AppendPrimitives<'a> for CoordinateSystem {
    fn append_primitives(
        &'a self,
        primitives: &mut Vec<crate::primitives::Primitives<'a>>,
        helper: &mut crate::chart::ChartPlotHelper,
    ) {
        match self {
            CoordinateSystem::Cartesian(cartesian) => {
                cartesian.x_axis.append_primitives(primitives, helper);
                cartesian.y_axis.append_primitives(primitives, helper);
            }
        }
    }
}
