fn main() {
    println!("{}", stars(1));
    println!("{}", stars(4));
    println!("{}", stars(5));
}

pub fn stars(n: u32) -> String {
    (0..2_i32.pow(n)).map(|_| "*").collect::<String>()
}