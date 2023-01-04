use std::{io::{Write, Seek}, error::Error};

use image::{ImageBuffer, ImageOutputFormat, Pixel, Rgb};
use ndarray::{Axis, Array2};

pub trait Plotter2D {
    fn plot_2d(&mut self, graph: &Array2<f32>) -> Result<(), Box<dyn Error>>;
}

pub struct WritePlotter<W> {
    writer: W,
}

impl<W: Write> WritePlotter<W> {
    pub fn new(writer: W) -> WritePlotter<W> {
        WritePlotter { writer: writer }
    }
}

impl<W: Write> Plotter2D for WritePlotter<W> {
    fn plot_2d(&mut self, graph: &Array2<f32>) -> Result<(), Box<dyn Error>> {
        graph.axis_iter(Axis(0)).map(|a| {
            let row: String = a.iter().map(|n| {
                let n = n.abs();
                if n < 0.01 {"· "}
                else if n < 0.5 {"* "}
                else if n < 0.99 {"O "}
                else {"X "}
            }).collect();
            writeln!(&mut self.writer, "{}", row)?;
            Ok(())
        }).collect()
    }
}

pub struct ImagePlotter<W, P> {
    writer: W,
    color_map_fn: fn(f32) -> P,
}

impl<W: Write + Seek, P: Pixel> ImagePlotter<W, P> {
    pub fn new(writer: W, color_map_fn: fn(f32) -> P) -> ImagePlotter<W, P> {
        ImagePlotter { 
            writer: writer,
            color_map_fn: color_map_fn,
        }
    }
}

impl<W: Write + Seek> Plotter2D for ImagePlotter<W, Rgb<u8>> {
    fn plot_2d(&mut self, graph: &Array2<f32>) -> Result<(), Box<dyn Error>> {
        let img = ImageBuffer::from_fn(graph.len_of(Axis(0)) as u32, graph.len_of(Axis(1)) as u32, |x, y| {
            let pixel = *graph.get((x as usize, y as usize)).unwrap();
            (self.color_map_fn)(pixel)
        });
        
        img.write_to(&mut self.writer, ImageOutputFormat::Png)?;
        Ok(())
    }
}