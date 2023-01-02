use std::error::Error;
use ndarray::{Array2, ArrayBase};
use num::{Float, traits::AsPrimitive};

pub trait Draw<T: Float + 'static> {
    /// Applies the graph function to `n`.
    /// Inline of possible.
    fn apply(&self, n: &[T]) -> T;

    /// Produces a 2D array of values received from graphing.
    fn draw_2d(
        &self,
        size: (usize, usize),
        bounds: (T, T, T, T),
    ) -> Result<Array2<T>, Box<dyn Error>> where usize: AsPrimitive<T> {
        let (xa, ya, xb, yb) = bounds;
        let xr = xb - xa;
        let yr = yb - ya;
        let (size_x, size_y) = size;

        let mut result = ArrayBase::zeros(size);
        result.indexed_iter_mut().for_each(|((x, y), n)| {
            let x = x.as_() / size_x.as_() * xr + xa;
            let y = y.as_() / size_y.as_() * yr + ya;
            *n = self.apply(&[x, y]);
        });

        Ok(result)
    }
}

/// Draws value `1.0` for all inputs.
pub struct TrueGrapher;

/// Draws the distance to origin
pub struct DistToGrapher;

/// Draws the sum of inputs
pub struct AddGrapher;

impl Draw<f32> for TrueGrapher {
    #[inline]
    fn apply(&self, _n: &[f32]) -> f32 {
        1.0
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
