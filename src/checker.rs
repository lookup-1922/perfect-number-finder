pub mod is_mersenne {
    use indicatif::{ProgressBar, ProgressStyle};
    use num_bigint::BigUint;
    use num_traits::One;
    use num_traits::Zero;
    use std::fs::File;
    use std::io::Write;
    use std::time::Instant;

    pub fn lucas_test(n: u64, progress_bar: &ProgressBar) -> bool {
        let m = BigUint::one() << n;
        let mut s = BigUint::new(vec![4]);
        let mut m_minus_one = m.clone();
        m_minus_one -= BigUint::one();

        for _ in 0..(n - 2) {
            s = (s.clone() * &s - BigUint::new(vec![2])) % &m;
            progress_bar.inc(1);
        }

        s == BigUint::zero()
    }

    pub fn test_by_number(number: u128) -> bool {
        if is_prime::simple(number) {
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

    pub fn test_by_p(p: u128) -> bool {
        if p <= 1 {
            return false;
        }

        let mersenne_prime: u128 = 2u128.pow(p as u32) - 1;

        return is_prime::simple(mersenne_prime);
    }

    pub mod is_prime {
        pub fn simple(number: u128) -> bool {
            if number <= 1 {
                return false;
            }
            if number == 2 || number == 3 {
                return true;
            }
            if number % 2 == 0 || number % 3 == 0 {
                return false;
            }

            let mut i = 5;
            while i * i <= number {
                if number % i == 0 || number % (i + 2) == 0 {
                    return false;
                }
                i += 6;
            }

            return true;
        }
    }
}

pub mod is_perfect_number {
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
}
