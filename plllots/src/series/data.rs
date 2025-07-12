#[derive(Debug, Clone, PartialEq)]
pub enum PlotData {
    Float(Vec<f64>),
    String(Vec<String>),
}

impl From<Vec<f64>> for PlotData {
    fn from(value: Vec<f64>) -> Self {
        Self::Float(value)
    }
}

impl From<Vec<String>> for PlotData {
    fn from(value: Vec<String>) -> Self {
        Self::String(value)
    }
}
