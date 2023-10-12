use std::io::{self, Read};

//use std::fs::File;
//use std::io::Write;

mod checker;
use checker::is_mersenne::lucas_lehmer_test;
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
        match input.trim() {
            "help" => help(),
            "lucas" => run_lucas_test(),
            &_ => println!("Unknown error has occured"),
        }
    }
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

fn check_number(number: u128) {
    let divisors = listup_divisors(number);
    let is_mersenne_prime = is_mersenne_prime_n(number);
    let is_perfect_number = is_pefect_number(number);
    println!("Divisors:{:?}", divisors);
    println!("Mersenne Prime:{}", is_mersenne_prime);
    println!("Pefect Number:{}", is_perfect_number);
}

fn run_lucas_test() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let p: u32 = input.trim().parse().ok().unwrap(); // メルセンヌ数の指数

    let result = lucas_lehmer_test(p);
    let result_message = if result {
        format!("2^{} - 1 is a Mersenne prime.", p)
    } else {
        format!("2^{} - 1 is not a Mersenne prime.", p)
    };

    // 結果をファイルに保存
    /*let mut file = File::create("mersenne_result.txt").expect("Failed to create file");
    file.write_all(result_message.as_bytes())
        .expect("Failed to write to file");*/

    println!("{}", result_message);
    exit_program();
}
