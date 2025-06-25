import numpy as np

def get_scale_details(min_val, max_val):
    # Minimal increment to avoid round extreme values to be on the edge of the chart
    epsilon = (max_val - min_val) / 1e6
    max_val += epsilon
    # min_val -= epsilon
    range_val = max_val - min_val

    # Target number of values to be displayed on the Y axis (it may be less)
    step_count = 8
    # First approximation
    rough_step = range_val / (step_count - 1)

    # Set best step for the range
    good_normalized_steps = [1, 1.5, 2, 2.5, 5, 7.5, 10]  # keep the 10 at the end

    # Normalize rough step to find the normalized one that fits best
    step_power = 10 ** -np.floor(np.log10(abs(rough_step)))
    normalized_step = rough_step * step_power
    good_normalized_step = next(n for n in good_normalized_steps if n >= normalized_step)
    step = good_normalized_step / step_power

    # Determine the scale limits based on the chosen step.
    scale_max = np.ceil(max_val / step) * step
    scale_min = np.floor(min_val / step) * step

    return scale_min, scale_max, step

def calculate(data):
    # Dummy code to show a usage example.
    minimum_value = min(data)
    maximum_value = max(data)
    results = get_scale_details(minimum_value, maximum_value)
    print(results)
    # chart.YAxis.MinValue = results[0]
    # chart.YAxis.MaxValue = results[1]
    # chart.YAxis.Step = results[2]

if __name__ == "__main__":
    calculate([-10, 1500, 2300, 2240, 2180, 1350, 1470, 2600])

