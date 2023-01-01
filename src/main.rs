use grapher::*;

fn main() {
    let grapher = CircleGrapher { radius: 1.0 };
    let result = grapher.draw_2d(10).unwrap();
    println!("{}", result);
}
