pub fn is_pefect_number(number: u128) -> bool {
    let divisors = listup_divisors(number);
    let sum_divisors: u128 = divisors.iter().sum();
    return sum_divisors == number * 2;
}

pub fn listup_divisors(number: u128) -> Vec<u128> {
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