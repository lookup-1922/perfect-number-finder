use std::io::{self, Read};
mod is_prime;

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
    let is_mersenne_prime = is_mersenne_prime_number(number);
    let is_perfect_number = is_pefect_number(number);
    println!("Divisors:{:?}", divisors);
    println!("Mersenne Prime:{}", is_mersenne_prime);
    println!("Pefect Number:{}", is_perfect_number);
}

fn Lucas_test() {

}

fn is_mersenne_prime_number(number: u128) -> bool {
    
    if is_prime::moderator(number) {
        let mersenne_exponent: u32 = (number as f64).log2() as u32;
        let mersenne_number: u128 = 2u128.pow(mersenne_exponent) - 1;

        if mersenne_number == number {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn is_mersenne_prime_p(p: u128) -> bool {
    if p <= 1 {
        return false;
    }

    let mersenne_prime: u128 = 2u128.pow(p as u32) - 1;

    return is_prime::moderator(mersenne_prime);
}

fn is_pefect_number(number: u128) -> bool {
    let divisors = listup_divisors(number);
    let sum_divisors: u128 = divisors.iter().sum();
    return sum_divisors == number * 2;
}

fn listup_divisors(number: u128) -> Vec<u128> {
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
