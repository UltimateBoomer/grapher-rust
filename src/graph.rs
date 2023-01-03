use std::{io::{Write}, error::Error};

use ndarray::{Axis, Array2};

pub trait Plotter2D {
    fn plot_2d(&mut self, graph: Array2<f32>) -> Result<(), Box<dyn Error>>;
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
    fn plot_2d(&mut self, graph: Array2<f32>) -> Result<(), Box<dyn Error>> {
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

pub struct ImagePlotter {

}
