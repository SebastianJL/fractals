use ndarray::{Array1, Array2};
use num_complex::Complex64;

fn mandelbrot(z: Complex64, c: Complex64) -> Complex64 {
    return z * z + c;
}

fn fatou_grid<F>(grid: &Array2<Complex64>, f: F, max_iter: i32) -> Array2<i32>
where
    F: Fn(Complex64, Complex64) -> Complex64,
{
    let mut iter_grid = Array2::<i32>::zeros(grid.raw_dim());
    for i in 0..grid.nrows() {
        for j in 0..grid.ncols() {
            let mut z = Complex64::new(0., 0.);
            for iter in 0..max_iter {
                let c = grid[(i, j)];
                z = f(z, c);
                if z.norm() > 2. {
                    if iter > 3 {
                        println!("{}", iter);
                    }

                    //println!("{}: broken", z);
                    iter_grid[[i, j]] = iter;
                    break;
                }
            }
        }
    }

    println!("{}", iter_grid);
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

fn main() {
    let n_grid = 100;
    let xrange = Array1::<f64>::linspace(-2., 2., n_grid);
    let yrange = Array1::<f64>::linspace(-2., 2., n_grid);
    let grid = meshgrid(xrange, yrange);

    let iter_grid = fatou_grid(&grid, mandelbrot, 100);

    println!("{}", grid);
    println!("{}", iter_grid);
}
