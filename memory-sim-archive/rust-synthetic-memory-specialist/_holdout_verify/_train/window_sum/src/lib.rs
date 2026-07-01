#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowReport {
    pub node: String,
    pub sum: i64,
    pub used: Vec<i64>,
}

struct Series {
    node: String,
    values: Vec<i64>,
}

pub struct WindowSum {
    window: usize,
    series: Vec<Series>,
}

impl WindowSum {
    pub fn new() -> Self {
        WindowSum { window: 0, series: Vec::new() }
    }

    pub fn set_window(&mut self, window: usize) {
        self.window = window;
    }

    pub fn push(&mut self, node: &str, value: i64) {
        match self.series.iter_mut().find(|s| s.node == node) {
            Some(series) => series.values.push(value),
            None => self.series.push(Series { node: node.to_string(), values: vec![value] }),
        }
    }

    pub fn report(&self, node: &str) -> WindowReport {
        let values = match self.series.iter().find(|s| s.node == node) {
            Some(series) => series.values.as_slice(),
            None => &[],
        };
        let start = if self.window >= values.len() {
            0
        } else {
            values.len() - self.window
        };
        let used: Vec<i64> = values[start..].to_vec();
        let sum: i64 = used.iter().sum();
        WindowReport { node: node.to_string(), sum, used }
    }
}
