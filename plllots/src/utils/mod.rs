pub(crate) mod lttb;

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

// from https://github.com/jeromefroe/lttb-rs
// pub fn lttb(data: Vec<Point>, threshold: usize) -> Vec<Point> {
//     if threshold >= data.len() || threshold == 0 {
//         // Nothing to do.

//         return data;
//     }
//     let mut sampled = Vec::with_capacity(threshold);
//     // Bucket size. Leave room for start and end data points.
//     let every = ((data.len() - 2) as f64) / ((threshold - 2) as f64);
//     // Initially a is the first point in the triangle.
//     let mut a = 0;
//     // Always add the first point.
//     sampled.push(data[a]);

//     for i in 0..threshold - 2 {
//         // Calculate point average for next bucket (containing c).
//         let mut avg_x = 0f64;
//         let mut avg_y = 0f64;
//         let avg_range_start = (((i + 1) as f64) * every) as usize + 1;
//         let mut end = (((i + 2) as f64) * every) as usize + 1;
//         if end >= data.len() {
//             end = data.len();
//         }
//         let avg_range_end = end;
//         let avg_range_length = (avg_range_end - avg_range_start) as f64;
//         for i in 0..(avg_range_end - avg_range_start) {
//             let idx = (avg_range_start + i) as usize;
//             avg_x += data[idx].x;
//             avg_y += data[idx].y;
//         }
//         avg_x /= avg_range_length;
//         avg_y /= avg_range_length;
//         // Get the range for this bucket.
//         let range_offs = ((i as f64) * every) as usize + 1;
//         let range_to = (((i + 1) as f64) * every) as usize + 1;
//         // Point a.
//         let point_a_x = data[a].x;
//         let point_a_y = data[a].y;
//         let mut max_area = -1f64;
//         let mut next_a = range_offs;
//         for i in 0..(range_to - range_offs) {
//             let idx = (range_offs + i) as usize;
//             // Calculate triangle area over three buckets.
//             let area = ((point_a_x - avg_x) * (data[idx].y - point_a_y)
//                 - (point_a_x - data[idx].x) * (avg_y - point_a_y))
//                 .abs()
//                 * 0.5;
//             if area > max_area {
//                 max_area = area;
//                 next_a = idx; // Next a is this b.
//             }
//         }
//         sampled.push(data[next_a]); // Pick this point from the bucket.

//         a = next_a; // This a is the next a (chosen b).
//     }
//     // Always add the last point.
//     sampled.push(data[data.len() - 1]);

//     sampled
// }

#[cfg(test)]
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
