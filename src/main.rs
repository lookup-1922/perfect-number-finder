use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let number_p: u128 = input.trim().parse().ok().unwrap();
    let result = is_prime(&number_p);
    println!("{}", result);
}

fn is_prime(number: &u128) -> bool {
    if number <= &1 {
        return false;
    }
    for i in 2..*number {
        if number % i == 0 {
            return false;
        }
    }
    return true;
}

fn is_pefect_number(number: i64) {}
