use image::GrayImage;
use ndarray::{Array1, Array2};
use num_complex::Complex64;

fn mandelbrot(z: Complex64, c: Complex64) -> Complex64 {
    return z * z + c;
}

fn fatou_grid<F>(grid: &Array2<Complex64>, f: F, max_iter: u8) -> Array2<u8>
where
    F: Fn(Complex64, Complex64) -> Complex64,
{
    let mut iter_grid = Array2::<u8>::zeros(grid.raw_dim());
    for i in 0..grid.nrows() {
        for j in 0..grid.ncols() {
            let mut z = Complex64::new(0., 0.);
            for iter in 0..max_iter {
                let c = grid[(i, j)];
                z = f(z, c);
                if z.norm() > 2. {
                    iter_grid[[i, j]] = iter;
                    break;
                }
            }
        }
    }

    iter_grid
}

fn meshgrid(xrange: Array1<f64>, yrange: Array1<f64>) -> Array2<Complex64> {
    let mut grid = Array2::<Complex64>::zeros((xrange.len(), yrange.len()));

    for (i, x) in xrange.into_iter().enumerate() {
        for (j, y) in yrange.clone().into_iter().enumerate() {
            grid[(i, j)] = Complex64::new(x, y);
        }
    }

    grid
}

fn array_to_grayscale(arr: Array2<u8>) -> GrayImage {
    assert!(arr.is_standard_layout());

    let (height, width) = arr.dim();
    let raw = arr.into_raw_vec();

    GrayImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

fn main() {
    let n_grid = 20_000;
    let xrange = Array1::<f64>::linspace(-2., 2., n_grid);
    let yrange = Array1::<f64>::linspace(-1., 1., n_grid / 2);
    let grid = meshgrid(xrange, yrange);

    let iter_grid = fatou_grid(&grid, mandelbrot, 255);
    let iter_grid = array_to_grayscale(iter_grid);
    iter_grid.save("output/out.png").expect("couldn't save");
}
