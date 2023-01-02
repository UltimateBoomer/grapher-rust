use std::error::Error;

use ndarray::{Array2, ArrayBase};

pub trait Draw {
    /// Applies the graph function to `n`.
    /// Inline of possible.
    fn apply(&self, n: &[f32]) -> f32;

    /// Produces a 2D array of values received from graphing.
    fn draw_2d(
        &self,
        size: (usize, usize),
        bounds: (f32, f32, f32, f32),
    ) -> Result<Array2<f32>, Box<dyn Error>> {
        let (xa, ya, xb, yb) = bounds;
        let xr = xb - xa;
        let yr = yb - ya;
        let (size_x, size_y) = size;

        let mut result = ArrayBase::zeros(size);
        result.indexed_iter_mut().for_each(|((x, y), n)| {
            let x = x as f32 / size_x as f32 * xr + xa;
            let y = y as f32 / size_y as f32 * yr + ya;
            *n = self.apply(&[x, y]);
        });

        Ok(result)
    }
}

/// Draws value `1.0` for all inputs.
pub struct TrueGrapher;

/// Draws the distance to origin
pub struct DistToGrapher;

impl Draw for TrueGrapher {
    #[inline]
    fn apply(&self, _n: &[f32]) -> f32 {
        1.0
    }
}

impl Draw for DistToGrapher {
    #[inline]
    fn apply(&self, n: &[f32]) -> f32 {
        n.iter().map(|x| x.powi(2)).sum::<f32>().sqrt()
    }
}
