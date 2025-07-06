pub fn lttb_optimized_memory(
    x_data: &Vec<f64>,
    y_data: &Vec<f64>,
    threshold: usize,
) -> (Vec<f64>, Vec<f64>) {
    let data_len = x_data.len();

    if threshold >= data_len || threshold == 0 {
        return (x_data.to_vec(), y_data.to_vec());
    }

    let mut sampled_x = Vec::with_capacity(threshold);
    let mut sampled_y = Vec::with_capacity(threshold);
    let every = (data_len - 2) as f64 / (threshold - 2) as f64;
    let mut a = 0;

    sampled_x.push(x_data[a]);
    sampled_y.push(y_data[a]);

    // Pre-calculate all bucket boundaries to avoid repeated calculations
    let mut bucket_starts = Vec::with_capacity(threshold);
    let mut bucket_ends = Vec::with_capacity(threshold);
    let mut avg_starts = Vec::with_capacity(threshold);
    let mut avg_ends = Vec::with_capacity(threshold);

    for i in 0..threshold - 2 {
        let range_start = (i as f64 * every) as usize + 1;
        let range_end = ((i + 1) as f64 * every) as usize + 1;
        let avg_range_start = ((i + 1) as f64 * every) as usize + 1;
        let avg_range_end = ((i + 2) as f64 * every).min(data_len as f64 - 1.0) as usize + 1;

        bucket_starts.push(range_start);
        bucket_ends.push(range_end);
        avg_starts.push(avg_range_start);
        avg_ends.push(avg_range_end);
    }

    for i in 0..threshold - 2 {
        let avg_range_start = avg_starts[i];
        let avg_range_end = avg_ends[i];

        // Optimized average calculation
        let (avg_x, avg_y) = if avg_range_end - avg_range_start == 1 {
            (x_data[avg_range_start], y_data[avg_range_start])
        } else {
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            // Manual unrolling for small slices
            let len = avg_range_end - avg_range_start;
            let mut idx = 0;
            // Process in pairs
            while idx + 1 < len {
                let i1 = avg_range_start + idx;
                let i2 = avg_range_start + idx + 1;
                sum_x += x_data[i1] + x_data[i2];
                sum_y += y_data[i1] + y_data[i2];
                idx += 2;
            }
            // Handle remainder
            if idx < len {
                let i = avg_range_start + idx;
                sum_x += x_data[i];
                sum_y += y_data[i];
            }
            (sum_x / len as f64, sum_y / len as f64)
        };

        let range_start = bucket_starts[i];
        let range_end = bucket_ends[i];
        let point_a_x = x_data[a];
        let point_a_y = y_data[a];

        // Optimized area calculation with early termination
        let mut max_area = -1.0;
        let mut max_idx = 0;

        // Pre-calculate constants
        let ax_minus_avgx = point_a_x - avg_x;
        let avgy_minus_ay = avg_y - point_a_y;

        // Unrolled loop for small buckets
        let len = range_end - range_start;
        let mut idx = 0;
        while idx + 1 < len {
            let i1 = range_start + idx;
            let i2 = range_start + idx + 1;
            let p1_x = x_data[i1];
            let p1_y = y_data[i1];
            let p2_x = x_data[i2];
            let p2_y = y_data[i2];

            let area1 = (ax_minus_avgx * (p1_y - point_a_y) - (point_a_x - p1_x) * avgy_minus_ay)
                .abs()
                * 0.5;
            let area2 = (ax_minus_avgx * (p2_y - point_a_y) - (point_a_x - p2_x) * avgy_minus_ay)
                .abs()
                * 0.5;

            if area1 > max_area {
                max_area = area1;
                max_idx = idx;
            }
            if area2 > max_area {
                max_area = area2;
                max_idx = idx + 1;
            }
            idx += 2;
        }

        // Handle remainder
        if idx < len {
            let i = range_start + idx;
            let p_x = x_data[i];
            let p_y = y_data[i];
            let area =
                (ax_minus_avgx * (p_y - point_a_y) - (point_a_x - p_x) * avgy_minus_ay).abs() * 0.5;
            if area > max_area {
                max_idx = idx;
            }
        }

        let next_a = range_start + max_idx;
        sampled_x.push(x_data[next_a]);
        sampled_y.push(y_data[next_a]);
        a = next_a;
    }

    sampled_x.push(x_data[data_len - 1]);
    sampled_y.push(y_data[data_len - 1]);

    (sampled_x, sampled_y)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_version() {
        let x_values: Vec<f64> = (0..100000)
            .into_iter()
            .map(|i| (i as f64 / 100000.0) * 6.28)
            .collect();
        let y_values_sin: Vec<f64> = (0..100000)
            .into_iter()
            .map(|i| ((i as f64 / 100000.0) * 6.28).sin())
            .collect();

        let instant = std::time::Instant::now();
        let (result, _result2) = lttb_optimized_memory(&x_values, &y_values_sin, 1000);

        println!("{:?}", instant.elapsed());
        assert_eq!(result.len(), 1000);
        assert_eq!(result[0], 0.0);
        assert_eq!(result[result.len() - 1], 999999.0);
        panic!()
    }
}
