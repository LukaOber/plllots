use bon::Builder;

#[derive(Debug, Clone, Copy, Builder)]
pub struct PlotSize {
    pub width: f64,
    pub height: f64,
}

impl Default for PlotSize {
    fn default() -> Self {
        Self {
            width: 1000.0,
            height: 1000.0,
        }
    }
}
