use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let number:i128 = input.trim().parse().ok().unwrap();
    println!("{}",number);
}
