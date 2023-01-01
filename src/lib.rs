use std::{error::Error};

use ndarray::{Array2, ArrayBase};

pub fn draw_2d(func: &(dyn Fn(&[f32]) -> f32), size: usize) -> Result<Array2<f32>, Box<dyn Error>> {
    let mut result = ArrayBase::zeros([size, size]);
    result.indexed_iter_mut().for_each(|((x, y), n)| {
        let x = x as f32;
        let y = y as f32;
        *n = func(&[x, y]);
    });

    Ok(result)
}

pub fn func_true(_n: &[f32]) -> f32 {
    1.0
}