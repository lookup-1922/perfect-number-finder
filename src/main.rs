use std::io::{self, Read};
mod checker;
use checker::is_mersenne::test_by_number as is_mersenne_prime_n;
use checker::is_perfect_number::is_pefect_number as is_pefect_number;
use checker::is_perfect_number::listup_divisors as listup_divisors;

fn main() {
    let mut input = String::new();

    println!("数値を入力してください");
    io::stdin().read_line(&mut input).ok();
    let number: u128 = input.trim().parse().ok().unwrap();

    check_number(number);

    println!("任意のキーを押してください...");
    // 標準入力からバイトを読み取る
    let mut buffer = [0; 1];
    let _ = io::stdin().read_exact(&mut buffer); // 1バイト読み取る
    println!("プログラムを終了します。");
}

fn check_number(number: u128) {
    let divisors = listup_divisors(number);
    let is_mersenne_prime = is_mersenne_prime_n(number);
    let is_perfect_number = is_pefect_number(number);
    println!("Divisors:{:?}", divisors);
    println!("Mersenne Prime:{}", is_mersenne_prime);
    println!("Pefect Number:{}", is_perfect_number);
}
