fn mandelbrot_area_estimate(grid_size: usize, max_iter: usize) -> f64 {
    let lower_x = -2.0;
    let upper_x = 1.0;
    let lower_y = -1.5;
    let upper_y = 1.5;

    let dx = (upper_x - lower_x) / grid_size as f64;
    let dy = (upper_y - lower_y) / grid_size as f64;

    // Test if a complex point c is in Mandelbrot set by iterating z = z^2 + c up to max_iter
    fn is_in_mandelbrot(cx: f64, cy: f64, max_iter: usize) -> bool {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut iteration = 0;
        while x*x + y*y <= 4.0 && iteration < max_iter {
            let x_new = x*x - y*y + cx;
            y = 2.0*x*y + cy;
            x = x_new;
            iteration += 1;
        }
        iteration == max_iter
    }

    let total_points = grid_size * grid_size;

    let inside_count = (0..grid_size).flat_map(|i| {
        (0..grid_size).map(move |j| {
            let cx = lower_x + i as f64 * dx;
            let cy = lower_y + j as f64 * dy;
            is_in_mandelbrot(cx, cy, max_iter)
        })
    }).filter(|&in_set| in_set).count();

    let sample_area = (upper_x - lower_x) * (upper_y - lower_y);
    (inside_count as f64 / total_points as f64) * sample_area
}
