pub mod line;

pub use line::*;

use crate::utils::get_raw_range;

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
    pub(crate) fn get_raw_range(&self) -> (f64, f64) {
        match self {
            Series::Line(line) => get_raw_range(&line.data),
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
