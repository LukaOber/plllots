pub mod line;
pub mod scatter;

pub use line::*;
pub use scatter::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Series {
    Line(Line),
    Scatter(Scatter),
}

impl From<Line> for Series {
    fn from(value: Line) -> Self {
        Self::Line(value)
    }
}

impl From<Scatter> for Series {
    fn from(value: Scatter) -> Self {
        Self::Scatter(value)
    }
}

impl Series {
    pub(crate) fn x_axis_index(&self) -> usize {
        match self {
            Series::Line(line) => line.x_axis_index,
            Series::Scatter(scatter) => scatter.x_axis_index,
        }
    }

    pub(crate) fn y_axis_index(&self) -> usize {
        match self {
            Series::Line(line) => line.y_axis_index,
            Series::Scatter(scatter) => scatter.y_axis_index,
        }
    }
}
