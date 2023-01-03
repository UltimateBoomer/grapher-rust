use grapher::*;

fn main() {
    let grapher = MandelbrotGrapher {
        cutoff: 1.0,
        iterations: 50,
    };
    let result = grapher.draw_2d((10, 10), (-1.0, -1.0, 1.0, 1.0)).unwrap();
    println!("{:.3}", result);
}
