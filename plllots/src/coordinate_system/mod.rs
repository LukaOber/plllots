pub mod cartesian;

pub use cartesian::*;

use crate::{chart::Theme, primitives::AppendPrimitives};

#[derive(Debug, Clone, PartialEq)]
pub enum CoordinateSystem {
    Cartesian(Cartesian),
}

impl<'a> AppendPrimitives<'a> for CoordinateSystem {
    fn append_primitives(
        &'a self,
        primitives: &mut Vec<crate::primitives::Primitives<'a>>,
        helper: &mut crate::chart::ChartHelper,
        theme: &'a Theme,
    ) {
        match self {
            CoordinateSystem::Cartesian(cartesian) => {
                cartesian.append_primitives(primitives, helper, theme);
            }
        }
    }
}
