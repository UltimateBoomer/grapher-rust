use std::error::Error;

use ndarray::{Array2, ArrayBase};

pub trait Draw {
    fn apply(&self, n: &[f32]) -> f32;

    fn draw_2d(&self, size: usize) -> Result<Array2<f32>, Box<dyn Error>> {
        let mut result = ArrayBase::zeros([size, size]);
        result.indexed_iter_mut().for_each(|((x, y), n)| {
            let x = x as f32;
            let y = y as f32;
            *n = self.apply(&[x, y]);
        });

        Ok(result)
    }
}

pub struct TrueGrapher;
pub struct CircleGrapher {
    pub radius: f32,
}

impl Draw for TrueGrapher {
    #[inline]
    fn apply(&self, _n: &[f32]) -> f32 {
        1.0
    }
}

impl Draw for CircleGrapher {
    #[inline]
    fn apply(&self, n: &[f32]) -> f32 {
        self.radius - n.iter().map(|x| x.powf(2.0)).sum::<f32>()
    }
}
