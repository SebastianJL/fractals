use image::GrayImage;
use ndarray::{Array1, Array2};
use num_complex::Complex32;

fn mandelbrot(z: Complex32, c: Complex32) -> Complex32 {
    return z * z + c;
}

fn burning_ship(z: Complex32, c: Complex32) -> Complex32 {
    let i = Complex32::new(0., 1.);
    return (z.re.abs() + i * z.im.abs()).powu(2) + c;
}

/// Returns Array2<u8> such that it can be plotted as a gray scale image.
fn fatou_grid<F>(
    xrange: Array1<f32>,
    yrange: Array1<f32>,
    f: F,
    div_radius: f32,
    max_iter: u32,
) -> Array2<u8>
where
    F: Fn(Complex32, Complex32) -> Complex32,
{
    let mut iter_grid = Array2::<u8>::zeros((yrange.len(), xrange.len()));

    let z0 = Complex32::new(0., 0.);
    for (i, y) in yrange.iter().enumerate() {
        for (j, x) in xrange.iter().enumerate() {
            let mut z = z0;
            for iter in 0..max_iter {
                let c = Complex32::new(*x, *y);
                z = f(z, c);
                if z.norm() > div_radius {
                    iter_grid[[i, j]] = (iter * u8::MAX as u32 / max_iter) as u8;
                    break;
                }
            }
        }
    }

    iter_grid
}

fn array_to_grayscale(arr: Array2<u8>) -> GrayImage {
    assert!(arr.is_standard_layout());
    assert!(arr.len() > 0);

    let (height, width) = arr.dim();
    let raw = arr.into_raw_vec();

    GrayImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

fn main() {
    // Draw mandelbrot
    if 1 == 1 {
        let (xmin, xmax) = (-2.5, 1.);
        let (ymin, ymax) = (-1., 1.);
        let n_grid_x = 20_00.;
        let n_grid_y = n_grid_x * (ymax - ymin) / (xmax - xmin);
        let n_grid_x = n_grid_x as usize;
        let n_grid_y = n_grid_y as usize;
        let x = Array1::<f32>::linspace(xmin, xmax, n_grid_x);
        let y = Array1::<f32>::linspace(ymin, ymax, n_grid_y);

        let max_iter = 100;
        let div_radius = 2.;
        let iter_grid = fatou_grid(x, y, mandelbrot, div_radius, max_iter);
        let iter_grid = array_to_grayscale(iter_grid);
        iter_grid
            .save("output/mandelbrot_out.png")
            .expect("couldn't save");
    }

    // Draw burning ship
    if 1 == 1 {
        let (xmin, xmax) = (-1.8, -1.7);
        let (ymin, ymax) = (-0.09, 0.01);
        let n_grid_x = 20_00.;
        let n_grid_y = n_grid_x * (ymax - ymin) / (xmax - xmin);
        let n_grid_x = n_grid_x as usize;
        let n_grid_y = n_grid_y as usize;
        let x = Array1::<f32>::linspace(xmin, xmax, n_grid_x);
        let y = Array1::<f32>::linspace(ymin, ymax, n_grid_y);

        let max_iter = 100;
        let div_radius = 4.;
        let iter_grid = fatou_grid(x, y, burning_ship, div_radius, max_iter);
        let iter_grid = array_to_grayscale(iter_grid);
        iter_grid
            .save("output/burning_ship_out.png")
            .expect("couldn't save");
    }
}
