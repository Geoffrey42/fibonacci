use std::io;
// Generate the nth Fibonacci number.
// fn get_fibonacci_nb(num: u64) -> u64
// 1, 1, 2, 3, 5, 8, 13, 21 ...

fn get_fibonacci_nb(num: u64) -> u64 {
    let mut previous = 1;
    let mut older = 1;
    let mut result = 1;

    for _element in 1..num - 1 {
        result = previous + older;
        older = previous;
        previous = result;
    }
    result
}

fn get_user_input() -> String {
    let mut number = String::new();

    match io::stdin().read_line(&mut number) {
        Ok(_n) => number,
        Err(error) => format!("Error: {}", error),
    }
}

fn main() {
    println!("Enter requested element in fibonacci sequence:");
    let number = get_user_input();
    let number: u64 = number.trim().parse().unwrap();

    if number > 0 {
        let result = get_fibonacci_nb(number);
        println!("{}th number in fibonacci sequence is {}", number, result);
    } else {
        println!("Error: number must be non 0 positive integer.",)
    }
}
