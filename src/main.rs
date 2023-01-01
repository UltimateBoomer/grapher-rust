use grapher::{draw_2d, func_true};

fn main() {
    let result = draw_2d(&func_true, 10).unwrap();
    println!("{}", result);
}
