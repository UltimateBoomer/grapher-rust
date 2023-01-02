use grapher::*;

fn main() {
    let grapher = AddGrapher;
    let result = grapher.draw_2d((10, 10), (-1.0, -1.0, 1.0, 1.0)).unwrap();
    println!("{:.3}", result);
}
