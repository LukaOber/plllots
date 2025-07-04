pub mod line;

pub use line::*;

use crate::utils::calculate_axis_ticks;

#[derive(Debug, Clone)]
pub enum Series {
    Line(Line),
}

impl From<Line> for Series {
    fn from(value: Line) -> Self {
        Self::Line(value)
    }
}

impl Series {
    pub(crate) fn calculate_axis_ticks(&self) -> (f64, f64, f64) {
        match self {
            Series::Line(line) => calculate_axis_ticks(&line.data),
        }
    }

    pub(crate) fn x_axis_index(&self) -> usize {
        match self {
            Series::Line(line) => line.x_axis_index,
        }
    }

    pub(crate) fn y_axis_index(&self) -> usize {
        match self {
            Series::Line(line) => line.y_axis_index,
        }
    }
}
