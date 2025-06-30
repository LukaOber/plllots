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
