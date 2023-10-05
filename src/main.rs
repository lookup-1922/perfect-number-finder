use std::io::{self, Read};

use std::time::Instant;
use std::fs::File;
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};

mod checker;
use checker::is_mersenne::test_by_number as is_mersenne_prime_n;
use checker::is_perfect_number::is_pefect_number;
use checker::is_perfect_number::listup_divisors;
use checker::is_mersenne::lucas_test;

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
        match input.trim() {
            "help" => help(),
            "lucas" => do_lucas_test(13),
            &_ => println!("Unknown error has occured"),
        }
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

fn help() {
    println!("help");
    exit_program();
}

fn do_lucas_test(exponent: u64) {
    let n = exponent; // メルセンヌ数の指数
    let start_time = Instant::now();

    // プログレスバーを初期化
    let progress_bar = ProgressBar::new(n - 2);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40}] {pos}/{len} ({eta})")
            .progress_chars("=> "),
    );

    let result = lucas_test(n, &progress_bar);

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);

    let result_message = if result {
        format!("2^{} - 1 is a Mersenne prime.", n)
    } else {
        format!("2^{} - 1 is not a Mersenne prime.", n)
    };

    // 結果をファイルに保存
    let mut file = File::create("mersenne_result.txt").expect("Failed to create file");
    file.write_all(result_message.as_bytes())
        .expect("Failed to write to file");

    progress_bar.finish();
}
