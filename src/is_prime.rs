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
