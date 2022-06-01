fn main() {
    println!(
        "{}",
        is_pangram("the quick brown fox jumps over the lazy dog!")
    );
    println!("{}", is_pangram("this is not a pangram!"));
}

pub fn is_pangram(raw: &str) -> bool {
    let s = raw.to_uppercase();
    return ('A'..'Z').filter( |v| s.contains(*v) ).count() == 25
}