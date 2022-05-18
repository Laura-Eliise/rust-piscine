fn main() {
    println!("The element in the position {} in fibonacci series is {}",2,  fibonacci(2));
    println!("The element in the position {} in fibonacci series is {}",4, fibonacci(4));
    println!("The element in the position {} in fibonacci series is {}",22, fibonacci(22));
    println!("The element in the position {} in fibonacci series is {}", 20, fibonacci(20));
}

pub fn fibonacci(n: u32) -> u32 {
    let mut memory: [u32; 25] = [0; 25];
    let mut i: usize = 2;
    memory[0] = 0;
    memory[1] = 1;
    while i <= (n as usize) {
        memory[i] = memory[i-1] + memory[i-2];
        i = i+1
    }
    return memory[(n as usize)]
}