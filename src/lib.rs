use std::{error::Error, ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign}};
use ndarray::{Array2, ArrayBase};
use num::{Float, Complex, Zero, FromPrimitive};

pub trait Draw<T: Float + 'static + FromPrimitive> {
    /// Applies the graph function to `n`.
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
            let x = T::from_usize(x).unwrap() / T::from_usize(size_x).unwrap() * xr + xa;
            let y = T::from_usize(y).unwrap() / T::from_usize(size_y).unwrap() * yr + ya;
            *n = self.apply(&[x, y]);
        });

        Ok(result)
    }
}

/// Draws value `0.0` for all inputs.
pub struct ZeroGrapher;

/// Draws the distance to origin
pub struct DistToGrapher;

/// Draws the sum of inputs
pub struct AddGrapher;

/// Draws the mandelbrot set in depth
pub struct MandelbrotGrapher<T: Float> {
    pub iterations: usize,
    pub cutoff: T,
}

impl<T: Float + 'static + FromPrimitive> Draw<T> for ZeroGrapher {
    #[inline]
    fn apply(&self, _n: &[T]) -> T {
        T::zero()
    }
}

impl Draw<f32> for DistToGrapher {
    #[inline]
    fn apply(&self, n: &[f32]) -> f32 {
        n.iter().map(|x| x.powi(2)).sum::<f32>().sqrt()
    }
}

impl Draw<f32> for AddGrapher {
    #[inline]
    fn apply(&self, n: &[f32]) -> f32 {
        n.iter().sum()
    }
}

impl<T: Float + 'static + FromPrimitive + AddAssign + SubAssign + MulAssign + DivAssign + RemAssign> Draw<T> for MandelbrotGrapher<T> {
    #[inline]
    fn apply(&self, n: &[T]) -> T {
        let mut z = Complex::<T>::zero();
        let c = Complex::new(n[0], n[1]);
        for i in 0..self.iterations {
            if z.re.abs() > self.cutoff || z.im.abs() > self.cutoff {
                return T::from(i).unwrap()
            }
            z += c;
        }
        T::from(self.iterations).unwrap()
    }
}