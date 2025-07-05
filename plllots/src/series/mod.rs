pub mod line;

pub use line::*;

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

    pub(crate) fn primary_data_index(&self) -> Option<usize> {
        match self {
            Series::Line(line) => line.data.primary_data_index,
        }
    }

    pub(crate) fn secondary_data_index(&self) -> Option<usize> {
        match self {
            Series::Line(line) => line.data.secondary_data_index,
        }
    }
}
