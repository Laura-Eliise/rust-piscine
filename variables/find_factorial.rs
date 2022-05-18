fn main() {
    println!("The factorial of 0 = {}", factorial(0));
    println!("The factorial of 1 = {}", factorial(1));
    println!("The factorial of 5 = {}", factorial(5));
    println!("The factorial of 10 = {}", factorial(10));
    println!("The factorial of 19 = {}", factorial(19));
}

pub fn factorial(num: u64) -> u64 {
    let mut sum: u64 = 1;
    let mut count: u64 = 1;
    loop {
        if count > num {break}
        sum *= count;
        count += 1;
    }
    return sum
}