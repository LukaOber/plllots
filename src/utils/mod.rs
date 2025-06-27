//! Utility functions for chart calculations and scaling.

fn get_scale_details(min_val: f64, max_val: f64) -> (f64, f64, f64) {
    // Minimal increment to avoid round extreme values to be on the edge of the chart
    let epsilon = (max_val - min_val) / 1e6;
    let max_val = if max_val < 0.0 {
        0.0
    } else {
        max_val + epsilon
    };
    let min_val = if min_val > 0.0 {
        0.0
    } else {
        min_val - epsilon
    };

    let range_val = max_val - min_val;

    // Target number of values to be displayed on the Y axis (it may be less)
    let step_count = 7;
    // First approximation
    let rough_step = range_val / (step_count as f64 - 1.0);

    // Set best step for the range
    let good_normalized_steps = vec![1.0, 1.5, 2.0, 3.0, 5.0, 7.5, 10.0]; // keep the 10 at the end

    // Normalize rough step to find the normalized one that fits best
    let step_power = 10f64.powf(-rough_step.abs().log10().floor());
    let normalized_step = rough_step * step_power;
    let good_normalized_step = good_normalized_steps
        .iter()
        .find(|&&n| n >= normalized_step)
        .unwrap();
    let step = good_normalized_step / step_power;

    // Determine the scale limits based on the chosen step.
    let scale_max = (max_val / step).ceil() * step;
    let scale_min = (min_val / step).floor() * step;

    (scale_min, scale_max, step)
}

pub fn calculate_axis_ticks(data: &[f64]) -> (f64, f64, f64) {
    // Dummy code to show a usage example.
    let minimum_value = data
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let maximum_value = data
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let results = get_scale_details(*minimum_value, *maximum_value);
    results
    // chart.YAxis.MinValue = results.0;
    // chart.YAxis.MaxValue = results.1;
    // chart.YAxis.Step = results.2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let data = [150.0, 230.0, 224.0, 218.0, 135.0, 147.0, 260.0];
        let (min, max, step) = calculate_axis_ticks(&data);
        assert_eq!(min, 0.0);
        assert_eq!(max, 300.0);
        assert_eq!(step, 50.0);
    }

    #[test]
    fn basic2() {
        let data = [820.0, 932.0, 901.0, 934.0, 1290.0, 1330.0, 1320.0];
        let (min, max, step) = calculate_axis_ticks(&data);
        assert_eq!(min, 0.0);
        assert_eq!(max, 1500.0);
        assert_eq!(step, 300.0);
    }

    #[test]
    fn basic3() {
        let data = [
            200.0, 560.0, 750.0, 580.0, 250.0, 300.0, 450.0, 300.0, 100.0,
        ];
        let (min, max, step) = calculate_axis_ticks(&data);
        assert_eq!(min, 0.0);
        assert_eq!(max, 900.0);
        assert_eq!(step, 150.0);
    }

    #[test]
    fn basic4() {
        let data = [0.0150, 0.0230, 0.0224, 0.0218, 0.0135, 0.0147, 0.0260];
        let (min, max, step) = calculate_axis_ticks(&data);
        assert_eq!(min, 0.0);
        assert_eq!(max, 0.03);
        assert_eq!(step, 0.005);
    }

    #[test]
    fn neg_basic() {
        let data = [-150.0, -230.0, -224.0, -218.0, -135.0, -147.0, -260.0];
        let (min, max, step) = calculate_axis_ticks(&data);
        assert_eq!(min, -300.0);
        assert_eq!(max, 0.0);
        assert_eq!(step, 50.0);
    }
}
