use bon::{Builder, builder};
use svg::{
    Document,
    node::element::{Path, Rectangle, SVG, path::Data},
};

#[derive(Debug, Clone, Builder)]
struct Chart {
    size: PlotSize,
}

#[derive(Debug, Clone, Copy)]
struct PlotSize {
    width: f32,
    height: f32,
}

impl Default for PlotSize {
    fn default() -> Self {
        Self {
            width: 1000.0,
            height: 1000.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
