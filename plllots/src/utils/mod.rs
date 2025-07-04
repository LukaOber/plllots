pub(crate) fn get_scale_details(min: f64, max: f64) -> (f64, f64, f64) {
    let epsilon = (max - min) / 1e6;
    let max = if max < 0.0 { 0.0 } else { max + epsilon };
    let min = if min > 0.0 { 0.0 } else { min - epsilon };

    let range_val = max - min;

    let step_count = 6;
    let rough_step = range_val / (step_count as f64);

    let good_normalized_steps = [1.0, 1.5, 2.0, 3.0, 5.0, 7.5, 10.0]; // keep the 10 at the end

    let step_power = 10f64.powf(-rough_step.abs().log10().floor());
    let normalized_step = rough_step * step_power;
    let good_normalized_step = good_normalized_steps
        .iter()
        .find(|&&n| n >= normalized_step)
        .unwrap();
    let step = good_normalized_step / step_power;

    let scale_max = (max / step).ceil() * step;
    let scale_min = (min / step).floor() * step;

    (scale_min, scale_max, step)
}

pub(crate) fn get_raw_range(data: &[f64]) -> (f64, f64) {
    let minimum_value = data
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let maximum_value = data
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    (*minimum_value, *maximum_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let data = [150.0, 230.0, 224.0, 218.0, 135.0, 147.0, 260.0];
        let (min, max) = get_raw_range(&data);
        let (min, max, step) = get_scale_details(min, max);
        assert_eq!(min, 0.0);
        assert_eq!(max, 300.0);
        assert_eq!(step, 50.0);
    }

    #[test]
    fn basic2() {
        let data = [820.0, 932.0, 901.0, 934.0, 1290.0, 1330.0, 1320.0];
        let (min, max) = get_raw_range(&data);
        let (min, max, step) = get_scale_details(min, max);
        assert_eq!(min, 0.0);
        assert_eq!(max, 1500.0);
        assert_eq!(step, 300.0);
    }

    #[test]
    fn basic3() {
        let data = [200.0, 560.0, 750.0, 580.0, 300.0, -250.0, 450.0];
        let (min, max) = get_raw_range(&data);
        let (min, max, step) = get_scale_details(min, max);
        assert_eq!(min, -400.0);
        assert_eq!(max, 800.0);
        assert_eq!(step, 200.0);
    }

    #[test]
    fn basic4() {
        let data = [0.0150, 0.0230, 0.0224, 0.0218, 0.0135, 0.0147, 0.0260];
        let (min, max) = get_raw_range(&data);
        let (min, max, step) = get_scale_details(min, max);
        assert_eq!(min, 0.0);
        assert_eq!(max, 0.03);
        assert_eq!(step, 0.005);
    }

    #[test]
    fn neg_basic() {
        let data = [-150.0, -230.0, -224.0, -218.0, -135.0, -147.0, -260.0];
        let (min, max) = get_raw_range(&data);
        let (min, max, step) = get_scale_details(min, max);
        assert_eq!(min, -300.0);
        assert_eq!(max, 0.0);
        assert_eq!(step, 50.0);
    }
}
