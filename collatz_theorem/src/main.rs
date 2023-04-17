fn main() {
    println!("Hello, world!");

    run_collatz_algorithm_till(2_u128.pow(10));
}

fn run_collatz_algorithm_till(target: u128) {
    for number in 1..target {
        collatz_recursive(number);
    }
}

fn collatz_recursive(number: u128) -> u128 {
    print!(" {} ", number);

    if number == 1 {
        println!();
        return number;
    }

    if number % 2 == 0 {
        return collatz_recursive(even_case(number));
    }

    return collatz_recursive(odd_case(number));
}

fn collatz_iterative(number: u128) -> u128 {
    if number == 1 {
        return number;
    }

    if number % 2 == 0 {
        return even_case(number);
    }

    return odd_case(number);
}

fn odd_case(number: u128) -> u128 { number * 3 + 1 }

fn even_case(number: u128) -> u128 { number / 2 }