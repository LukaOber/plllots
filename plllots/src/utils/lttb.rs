// Add these dependencies to your Cargo.toml:
// [dependencies]
// wide = "0.7"
// faster = "0.5"

use kurbo::Point;
use wide::f64x4;

pub fn lttb(data: &[Point], threshold: usize) -> Vec<Point> {
    if threshold >= data.len() || threshold == 0 {
        // Nothing to do.
        return data.to_vec();
    }
    let mut sampled = Vec::with_capacity(threshold);
    // Bucket size. Leave room for start and end data points.
    let every = ((data.len() - 2) as f64) / ((threshold - 2) as f64);
    // Initially a is the first point in the triangle.
    let mut a = 0;
    // Always add the first point.
    sampled.push(data[a]);
    for i in 0..threshold - 2 {
        // Calculate point average for next bucket (containing c).
        let mut avg_x = 0f64;
        let mut avg_y = 0f64;
        let avg_range_start = (((i + 1) as f64) * every) as usize + 1;
        let mut end = (((i + 2) as f64) * every) as usize + 1;
        if end >= data.len() {
            end = data.len();
        }
        let avg_range_end = end;
        let avg_range_length = (avg_range_end - avg_range_start) as f64;
        for i in 0..(avg_range_end - avg_range_start) {
            let idx = (avg_range_start + i) as usize;
            avg_x += data[idx].x;
            avg_y += data[idx].y;
        }
        avg_x /= avg_range_length;
        avg_y /= avg_range_length;
        // Get the range for this bucket.
        let range_offs = ((i as f64) * every) as usize + 1;
        let range_to = (((i + 1) as f64) * every) as usize + 1;
        // Point a.
        let point_a_x = data[a].x;
        let point_a_y = data[a].y;
        let mut max_area = -1f64;
        let mut next_a = range_offs;
        for i in 0..(range_to - range_offs) {
            let idx = (range_offs + i) as usize;
            // Calculate triangle area over three buckets.
            let area = ((point_a_x - avg_x) * (data[idx].y - point_a_y)
                - (point_a_x - data[idx].x) * (avg_y - point_a_y))
                .abs()
                * 0.5;
            if area > max_area {
                max_area = area;
                next_a = idx; // Next a is this b.
            }
        }
        sampled.push(data[next_a]); // Pick this point from the bucket.
        a = next_a; // This a is the next a (chosen b).
    }
    // Always add the last point.
    sampled.push(data[data.len() - 1]);
    sampled
}
pub fn lttb_simd_wide(data: &[Point], threshold: usize) -> Vec<Point> {
    if threshold >= data.len() || threshold == 0 {
        return data.to_vec();
    }

    let mut sampled = Vec::with_capacity(threshold);
    let every = (data.len() - 2) as f64 / (threshold - 2) as f64;
    let mut a = 0;

    sampled.push(data[a]);

    for i in 0..threshold - 2 {
        let avg_range_start = ((i + 1) as f64 * every) as usize + 1;
        let mut avg_range_end = ((i + 2) as f64 * every) as usize + 1;
        if avg_range_end >= data.len() {
            avg_range_end = data.len();
        }

        let (avg_x, avg_y) = calculate_average_simd_wide(&data[avg_range_start..avg_range_end]);

        let range_start = (i as f64 * every) as usize + 1;
        let range_end = ((i + 1) as f64 * every) as usize + 1;

        let point_a_x = data[a].x;
        let point_a_y = data[a].y;

        let next_a = find_max_area_simd_wide(
            &data[range_start..range_end],
            range_start,
            point_a_x,
            point_a_y,
            avg_x,
            avg_y,
        );

        sampled.push(data[next_a]);
        a = next_a;
    }

    sampled.push(data[data.len() - 1]);
    sampled
}

#[inline]
fn calculate_average_simd_wide(data: &[Point]) -> (f64, f64) {
    if data.is_empty() {
        return (0.0, 0.0);
    }

    let len = data.len();
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;

    // Process 4 points at a time with SIMD
    let chunks = len / 4;

    if chunks > 0 {
        let mut x_acc = f64x4::ZERO;
        let mut y_acc = f64x4::ZERO;

        for chunk in data.chunks_exact(4) {
            let x_vals = f64x4::new([chunk[0].x, chunk[1].x, chunk[2].x, chunk[3].x]);
            let y_vals = f64x4::new([chunk[0].y, chunk[1].y, chunk[2].y, chunk[3].y]);

            x_acc = x_acc + x_vals;
            y_acc = y_acc + y_vals;
        }

        // Sum all elements in the SIMD vectors
        sum_x = x_acc.to_array().iter().sum::<f64>();
        sum_y = y_acc.to_array().iter().sum::<f64>();
    }

    // Handle remainder
    for point in data.iter().skip(chunks * 4) {
        sum_x += point.x;
        sum_y += point.y;
    }

    (sum_x / len as f64, sum_y / len as f64)
}

#[inline]
fn find_max_area_simd_wide(
    data: &[Point],
    offset: usize,
    point_a_x: f64,
    point_a_y: f64,
    avg_x: f64,
    avg_y: f64,
) -> usize {
    let len = data.len();
    if len == 0 {
        return offset;
    }

    let mut max_area = -1.0;
    let mut max_idx = 0;

    // Pre-calculate constants for area formula
    let ax_minus_avgx = point_a_x - avg_x;
    let avgy_minus_ay = avg_y - point_a_y;

    // Process 4 points at a time with SIMD
    let chunks = len / 4;

    if chunks > 0 {
        let ax_minus_avgx_vec = f64x4::splat(ax_minus_avgx);
        let avgy_minus_ay_vec = f64x4::splat(avgy_minus_ay);
        let point_a_x_vec = f64x4::splat(point_a_x);
        let point_a_y_vec = f64x4::splat(point_a_y);
        let half = f64x4::splat(0.5);

        for (chunk_idx, chunk) in data.chunks_exact(4).enumerate() {
            let x_vals = f64x4::new([chunk[0].x, chunk[1].x, chunk[2].x, chunk[3].x]);
            let y_vals = f64x4::new([chunk[0].y, chunk[1].y, chunk[2].y, chunk[3].y]);

            // Calculate area = abs((point_a_x - avg_x) * (y - point_a_y) - (point_a_x - x) * (avg_y - point_a_y)) * 0.5
            let term1 = ax_minus_avgx_vec * (y_vals - point_a_y_vec);
            let term2 = (point_a_x_vec - x_vals) * avgy_minus_ay_vec;
            let areas = (term1 - term2).abs() * half;

            // Check each area in the SIMD vector
            let areas_array = areas.to_array();
            for (i, &area) in areas_array.iter().enumerate() {
                if area > max_area {
                    max_area = area;
                    max_idx = chunk_idx * 4 + i;
                }
            }
        }
    }

    // Handle remainder
    for (i, point) in data.iter().enumerate().skip(chunks * 4) {
        let area = ((point_a_x - avg_x) * (point.y - point_a_y)
            - (point_a_x - point.x) * (avg_y - point_a_y))
            .abs()
            * 0.5;

        if area > max_area {
            max_area = area;
            max_idx = i;
        }
    }

    offset + max_idx
}
pub fn lttb_optimized_memory(data: &[Point], threshold: usize) -> Vec<Point> {
    if threshold >= data.len() || threshold == 0 {
        return data.to_vec();
    }

    let mut sampled = Vec::with_capacity(threshold);
    let every = (data.len() - 2) as f64 / (threshold - 2) as f64;
    let mut a = 0;

    sampled.push(data[a]);

    // Pre-calculate all bucket boundaries to avoid repeated calculations
    let mut bucket_starts = Vec::with_capacity(threshold);
    let mut bucket_ends = Vec::with_capacity(threshold);
    let mut avg_starts = Vec::with_capacity(threshold);
    let mut avg_ends = Vec::with_capacity(threshold);

    for i in 0..threshold - 2 {
        let range_start = (i as f64 * every) as usize + 1;
        let range_end = ((i + 1) as f64 * every) as usize + 1;
        let avg_range_start = ((i + 1) as f64 * every) as usize + 1;
        let avg_range_end = ((i + 2) as f64 * every).min(data.len() as f64 - 1.0) as usize + 1;

        bucket_starts.push(range_start);
        bucket_ends.push(range_end);
        avg_starts.push(avg_range_start);
        avg_ends.push(avg_range_end);
    }

    for i in 0..threshold - 2 {
        let avg_range_start = avg_starts[i];
        let avg_range_end = avg_ends[i];

        // Optimized average calculation
        let avg_slice = &data[avg_range_start..avg_range_end];
        let (avg_x, avg_y) = if avg_slice.len() == 1 {
            (avg_slice[0].x, avg_slice[0].y)
        } else {
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;

            // Manual unrolling for small slices
            let len = avg_slice.len();
            let mut idx = 0;

            // Process in pairs
            while idx + 1 < len {
                sum_x += avg_slice[idx].x + avg_slice[idx + 1].x;
                sum_y += avg_slice[idx].y + avg_slice[idx + 1].y;
                idx += 2;
            }

            // Handle remainder
            if idx < len {
                sum_x += avg_slice[idx].x;
                sum_y += avg_slice[idx].y;
            }

            (sum_x / len as f64, sum_y / len as f64)
        };

        let range_start = bucket_starts[i];
        let range_end = bucket_ends[i];

        let point_a_x = data[a].x;
        let point_a_y = data[a].y;

        // Optimized area calculation with early termination
        let bucket_slice = &data[range_start..range_end];
        let mut max_area = -1.0;
        let mut max_idx = 0;

        // Pre-calculate constants
        let ax_minus_avgx = point_a_x - avg_x;
        let avgy_minus_ay = avg_y - point_a_y;

        // Unrolled loop for small buckets
        let len = bucket_slice.len();
        let mut idx = 0;

        while idx + 1 < len {
            let p1 = bucket_slice[idx];
            let p2 = bucket_slice[idx + 1];

            let area1 = (ax_minus_avgx * (p1.y - point_a_y) - (point_a_x - p1.x) * avgy_minus_ay)
                .abs()
                * 0.5;
            let area2 = (ax_minus_avgx * (p2.y - point_a_y) - (point_a_x - p2.x) * avgy_minus_ay)
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
            let p = bucket_slice[idx];
            let area =
                (ax_minus_avgx * (p.y - point_a_y) - (point_a_x - p.x) * avgy_minus_ay).abs() * 0.5;
            if area > max_area {
                max_idx = idx;
            }
        }

        let next_a = range_start + max_idx;
        sampled.push(data[next_a]);
        a = next_a;
    }

    sampled.push(data[data.len() - 1]);
    sampled
}

pub fn lttb_parallel(data: &[Point], threshold: usize) -> Vec<Point> {
    use rayon::prelude::*;

    if threshold >= data.len() || threshold == 0 {
        return data.to_vec();
    }

    if data.len() < 10000 {
        // Use sequential version for small datasets
        return lttb_optimized_memory(data, threshold);
    }

    let mut sampled = Vec::with_capacity(threshold);
    let every = (data.len() - 2) as f64 / (threshold - 2) as f64;
    let mut a = 0;

    sampled.push(data[a]);

    // Pre-calculate all bucket info
    let bucket_info: Vec<_> = (0..threshold - 2)
        .into_par_iter()
        .map(|i| {
            let range_start = (i as f64 * every) as usize + 1;
            let range_end = ((i + 1) as f64 * every) as usize + 1;
            let avg_range_start = ((i + 1) as f64 * every) as usize + 1;
            let avg_range_end = ((i + 2) as f64 * every).min(data.len() as f64 - 1.0) as usize + 1;

            // Calculate average
            let avg_slice = &data[avg_range_start..avg_range_end];
            let (avg_x, avg_y) = if avg_slice.len() == 1 {
                (avg_slice[0].x, avg_slice[0].y)
            } else {
                let sum_x: f64 = avg_slice.iter().map(|p| p.x).sum();
                let sum_y: f64 = avg_slice.iter().map(|p| p.y).sum();
                (
                    sum_x / avg_slice.len() as f64,
                    sum_y / avg_slice.len() as f64,
                )
            };

            (range_start, range_end, avg_x, avg_y)
        })
        .collect();

    // Sequential processing (dependencies prevent full parallelization)
    for (range_start, range_end, avg_x, avg_y) in bucket_info {
        let point_a_x = data[a].x;
        let point_a_y = data[a].y;

        let bucket_slice = &data[range_start..range_end];
        let ax_minus_avgx = point_a_x - avg_x;
        let avgy_minus_ay = avg_y - point_a_y;

        let (max_idx, _) = bucket_slice
            .par_iter()
            .enumerate()
            .map(|(idx, point)| {
                let area = (ax_minus_avgx * (point.y - point_a_y)
                    - (point_a_x - point.x) * avgy_minus_ay)
                    .abs();
                (idx, area)
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or((0, 0.0));

        let next_a = range_start + max_idx;
        sampled.push(data[next_a]);
        a = next_a;
    }

    sampled.push(data[data.len() - 1]);
    sampled
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_version() {
        let data: Vec<Point> = (0..1000000)
            .map(|i| Point {
                x: i as f64,
                y: (i as f64 * 0.1).sin(),
            })
            .collect();

        let instant = std::time::Instant::now();
        let result = lttb_simd_wide(&data, 1000);

        println!("{:?}", instant.elapsed());
        assert_eq!(result.len(), 1000);
        assert_eq!(result[0].x, 0.0);
        assert_eq!(result[result.len() - 1].x, 999999.0);
        panic!()
    }

    #[test]
    fn test_naive_version() {
        let data: Vec<Point> = (0..1000000)
            .map(|i| Point {
                x: i as f64,
                y: (i as f64 * 0.1).sin(),
            })
            .collect();

        let instant = std::time::Instant::now();
        let result = lttb(&data, 1000);

        println!("{:?}", instant.elapsed());
        assert_eq!(result.len(), 1000);
        assert_eq!(result[0].x, 0.0);
        assert_eq!(result[result.len() - 1].x, 999999.0);
        panic!()
    }

    #[test]
    fn test_memory_version() {
        let data: Vec<Point> = (0..1000000)
            .map(|i| Point {
                x: i as f64,
                y: (i as f64 * 0.1).sin(),
            })
            .collect();

        let instant = std::time::Instant::now();
        let result = lttb_optimized_memory(&data, 1000);

        println!("{:?}", instant.elapsed());
        assert_eq!(result.len(), 1000);
        assert_eq!(result[0].x, 0.0);
        assert_eq!(result[result.len() - 1].x, 999999.0);
        panic!()
    }

    #[test]
    fn test_parallel_version() {
        let data: Vec<Point> = (0..1000000)
            .map(|i| Point {
                x: i as f64,
                y: (i as f64 * 0.1).sin(),
            })
            .collect();

        let instant = std::time::Instant::now();
        let result = lttb_parallel(&data, 1000);

        println!("{:?}", instant.elapsed());
        assert_eq!(result.len(), 1000);
        assert_eq!(result[0].x, 0.0);
        assert_eq!(result[result.len() - 1].x, 999999.0);
        panic!()
    }
}
