use num_bigint::{BigUint, ToBigUint};

fn pow(base: BigUint, exponent: BigUint) -> BigUint {
    if exponent == 0.to_biguint() {
        return 1.0;
    } else {
        let half_pow = pow(base, exponent / 2);
        if exponent % 2 == 0 {
            return half_pow * half_pow;
        } else {
            return base * half_pow * half_pow;
        }
    }
}