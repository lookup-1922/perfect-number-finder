use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    println!("数値を入力してください");
    io::stdin().read_line(&mut input).ok();
    let number_p: u128 = input.trim().parse().ok().unwrap();

    let is_prime = is_prime(number_p);
    let divisors = listup_divisors(number_p);
    println!("{}", is_prime);
    println!("{:?}",divisors);

    println!("任意のキーを押してください...");
    
    // 標準入力からバイトを読み取る
    let mut buffer = [0; 1];
    let _ = io::stdin().read_exact(&mut buffer); // 1バイト読み取る
    println!("プログラムを終了します。");
}

fn is_prime(number: u128) -> bool {
    if number <= 1 {
        return false;
    }
    if number == 2 {
        return true;
    }
    if number % 2 == 0 {
        return false;
    }

    let mut i = 3;
    while i * i <= number {
        if number % i == 0 {
            return false;
        }
        i += 2;
    }

    return true;
}

fn is_pefect_number(number: u128) {}

fn listup_divisors(number: u128) -> Vec<u128>{
    let mut divisors: Vec<u128> = vec![];
    let mut i = 1;
    while i * i <= number {
        if number % i == 0 {
            divisors.push(i);
            if i * i != number {
                divisors.push(number / i);
            }
        }
        i += 1;
    }
    divisors.sort();
    return divisors;
}
