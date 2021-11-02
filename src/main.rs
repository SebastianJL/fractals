use image::GrayImage;
use ndarray::{Array1, Array2};
use num_complex::Complex64;

fn mandelbrot(z: Complex64, c: Complex64) -> Complex64 {
    return z * z + c;
}

fn burning_ship(z: Complex64, c: Complex64) -> Complex64 {
    let i = Complex64::new(0., 1.);
    return (z.re.abs() + i * z.im.abs()).powu(2) + c;
}

fn fatou_grid<F>(grid: &Array2<Complex64>, f: F, div_radius: f64, max_iter: u32) -> Array2<u32>
where
    F: Fn(Complex64, Complex64) -> Complex64,
{
    let mut iter_grid = Array2::<u32>::zeros(grid.raw_dim());
    for i in 0..grid.nrows() {
        for j in 0..grid.ncols() {
            let mut z = Complex64::new(0., 0.);
            for iter in 0..max_iter {
                let c = grid[(i, j)];
                z = f(z, c);
                if z.norm() > div_radius {
                    iter_grid[[i, j]] = iter;
                    break;
                }
            }
        }
    }

    iter_grid
}

/// Return a 2d grid of complex numbers.
///
/// Creates a 2d grid of complex numbers with `xrange` and `yrange` coordinates.
/// Functions similar to numpy.meshgrid but instead of computing two separate 2d arrays
/// for x and y coordinates it puts them in a 2d grid of complex numbers.
fn complex_grid(xrange: Array1<f64>, yrange: Array1<f64>) -> Array2<Complex64> {
    let mut grid = Array2::<Complex64>::zeros((xrange.len(), yrange.len()));

    for (i, x) in xrange.into_iter().enumerate() {
        for (j, y) in yrange.clone().into_iter().enumerate() {
            grid[(i, j)] = Complex64::new(x, y);
        }
    }

    grid
}

fn array_to_grayscale(arr: Array2<u32>) -> GrayImage {
    assert!(arr.is_standard_layout());
    assert!(arr.len() > 0);

    let arr_u8 = Array2::<u8>::zeros(arr.raw_dim());

    let max: u32 = *arr.iter().max().expect("empty array");
    arr_u8 = (arr * 255 / max) as Array2<u8>;

    let (height, width) = arr_u8.dim();
    let raw = arr_u8.into_raw_vec();

    GrayImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

fn main() {
    // Draw mandelbrot
    if 1 == 0 {
        let (xmin, xmax) = (-2.5, 1.);
        let (ymin, ymax) = (-1., 1.);
        let n_grid_x = 20_000.;
        let n_grid_y = n_grid_x * (ymax - ymin) / (xmax - xmin);
        let n_grid_x = n_grid_x as usize;
        let n_grid_y = n_grid_y as usize;
        let x = Array1::<f64>::linspace(xmin, xmax, n_grid_x);
        let y = Array1::<f64>::linspace(ymin, ymax, n_grid_y);
        let grid = complex_grid(x, y);

        let max_iter = 255;
        let div_radius = 2.;
        let iter_grid = fatou_grid(&grid, mandelbrot, div_radius, max_iter);
        let iter_grid = array_to_grayscale(iter_grid);
        iter_grid
            .save("output/mandelbrot_out.png")
            .expect("couldn't save");
    }

    // Draw burning ship
    if 1 == 1 {
        let (xmin, xmax) = (-4., 4.);
        let (ymin, ymax) = (-4., 4.);
        let n_grid_x = 50_000.;
        let n_grid_y = n_grid_x * (ymax - ymin) / (xmax - xmin);
        let n_grid_x = n_grid_x as usize;
        let n_grid_y = n_grid_y as usize;
        let x = Array1::<f64>::linspace(xmin, xmax, n_grid_x);
        let y = Array1::<f64>::linspace(ymin, ymax, n_grid_y);
        let grid = complex_grid(x, y);

        let max_iter = 255;
        let div_radius = 4.;
        let iter_grid = fatou_grid(&grid, burning_ship, div_radius, max_iter);
        let iter_grid = array_to_grayscale(iter_grid);
        iter_grid
            .save("output/burning_ship_out.png")
            .expect("couldn't save");
    }
}
