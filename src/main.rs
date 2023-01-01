use grapher::{TrueGrapher, Draw, CircleGrapher};

fn main() {
    let result = CircleGrapher::draw_2d(10).unwrap();
    println!("{}", result);
}
