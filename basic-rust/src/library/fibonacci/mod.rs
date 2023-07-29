pub fn fibonacci_recursive(steps: u64) -> u128 {
    match steps {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(steps - 1) + fibonacci_recursive(steps - 2),
    }
}

pub fn fibonacci_iterative(mut steps: u64) -> u128 {
    if steps == 0 {
        return 0;
    }
    if steps == 1 {
        return 1;
    }

    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut c: u128 = 0;

    while steps > 1 {
        c = a + b;
        a = b;
        b = c;
        steps -= 1;
    }
    c
}
