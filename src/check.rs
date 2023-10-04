pub mod is_mersenne {
    pub fn Lucas_test() {}

    pub fn is_mersenne_n(number: u128) -> bool {
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

    pub fn is_mersenne_prime_p(p: u128) -> bool {
        if p <= 1 {
            return false;
        }

        let mersenne_prime: u128 = 2u128.pow(p as u32) - 1;

        return is_prime::moderator(mersenne_prime);
    }

    pub mod is_prime {
        pub fn moderator(number: u128) -> bool {
            return simple(number);
        }

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

pub mod is_perfect_number {}
