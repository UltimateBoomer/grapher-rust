pub mod graph;

use ndarray::{Array2, ArrayBase, NdFloat};
use num::{Complex, Zero};
use std::{
    error::Error,
};

/// This trait allows one to compute values for n-dimensional targets.\
/// By implementing this trait, this takes advantage of Rust's static optimization to achieve zero-cost abstraction.\
/// The `apply` function can be inlined to improve efficiency.
pub trait Grapher<T: NdFloat> {
    /// Applies the graph function to coordinate `n`.
    /// Inline of possible.
    fn apply(&self, n: &[T]) -> T;

    /// Produces a 2D array of values received from graphing.
    fn draw_2d(
        &self,
        size: (usize, usize),
        bounds: (T, T, T, T),
    ) -> Result<Array2<T>, Box<dyn Error>> {
        let (xa, ya, xb, yb) = bounds;
        let xr = xb - xa;
        let yr = yb - ya;
        let (size_x, size_y) = size;

        let mut result = ArrayBase::zeros(size);
        result.indexed_iter_mut().for_each(|((x, y), n)| {
            let x = T::from(x).unwrap() / T::from(size_x).unwrap() * xr + xa;
            let y = T::from(y).unwrap() / T::from(size_y).unwrap() * yr + ya;
            *n = self.apply(&[x, y]);
        });

        Ok(result)
    }
}

/// This grapher produces zero for all inputs.
pub struct ZeroGrapher;

/// This grapher produces the distance to origin of the vector formed by the input.
pub struct DistToGrapher;

/// This grapher produces the sum of inputs.
pub struct AddGrapher;

/// This grapher produces the value of the corresponding point in the mandelbrot set.
pub struct MandelbrotGrapher {
    pub iterations: usize,
}

impl<T: NdFloat> Grapher<T> for ZeroGrapher {
    #[inline]
    fn apply(&self, _n: &[T]) -> T {
        T::zero()
    }
}

impl Grapher<f32> for DistToGrapher {
    #[inline]
    fn apply(&self, n: &[f32]) -> f32 {
        n.iter().map(|x| x.powi(2)).sum::<f32>().sqrt()
    }
}

impl Grapher<f32> for AddGrapher {
    #[inline]
    fn apply(&self, n: &[f32]) -> f32 {
        n.iter().sum()
    }
}

impl<T: NdFloat> Grapher<T> for MandelbrotGrapher {
    #[inline]
    fn apply(&self, n: &[T]) -> T {
        let mut z = Complex::<T>::zero();
        let c = Complex::new(n[0], n[1]);
        for i in 0..self.iterations {
            if z.norm_sqr() > T::from(4).unwrap() {
                return T::from(i).unwrap();
            }
            z = z * z + c;
        }
        T::zero()
    }
}
