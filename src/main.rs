use std::io::{self, Read};

mod checker;
use checker::is_mersenne::test_by_number as is_mersenne_prime_n;
use checker::is_perfect_number::is_pefect_number;
use checker::is_perfect_number::listup_divisors;

fn main() {
    let mut input = String::new();

    println!("Waiting for you to enter.");
    io::stdin().read_line(&mut input).ok();

    let is_number = match input.trim().parse::<u128>() {
        Ok(_) => true,
        Err(_) => false,
    };

    if is_number == true {
        let number: u128 = input.trim().parse().ok().unwrap();
        check_number(number);
        exit_program();
    } else if is_number == false {
    }
}

fn check_number(number: u128) {
    let divisors = listup_divisors(number);
    let is_mersenne_prime = is_mersenne_prime_n(number);
    let is_perfect_number = is_pefect_number(number);
    println!("Divisors:{:?}", divisors);
    println!("Mersenne Prime:{}", is_mersenne_prime);
    println!("Pefect Number:{}", is_perfect_number);
}

fn exit_program() {
    println!("Press any key to continue...");
    // 標準入力からバイトを読み取る
    let mut buffer = [0; 1];
    let _ = io::stdin().read_exact(&mut buffer); // 1バイト読み取る
    println!("Exit the program.");
}

fn help() {}
