use std::error::Error;

use ndarray::{Array2, ArrayBase};

pub trait Draw {
    fn apply(n: &[f32]) -> f32;

    fn draw_2d(size: usize) -> Result<Array2<f32>, Box<dyn Error>> {
        let mut result = ArrayBase::zeros([size, size]);
        result.indexed_iter_mut().for_each(|((x, y), n)| {
            let x = x as f32;
            let y = y as f32;
            *n = Self::apply(&[x, y]);
        });
    
        Ok(result)
    }
}

pub struct TrueGrapher;
pub struct CircleGrapher;

impl Draw for TrueGrapher {
    #[inline]
    fn apply(_n: &[f32]) -> f32 {
        1.0
    }
}

impl Draw for CircleGrapher {
    #[inline]
    fn apply(n: &[f32]) -> f32 {
        1.0 - n.iter().map(|x| x.powf(2.0)).sum::<f32>()
    }
}