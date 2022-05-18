fn main() {
    println!("{}", is_empty(""));
    println!("{}", is_ascii("rust"));
    println!("{}", contains("rust", "ru"));
    println!("{:?}", split_at("rust", 2));
    println!("{}", find("rust", 'u'));
}

pub fn is_empty(v: &str) -> bool {
    if v == "" {return true}
    return false
}

pub fn is_ascii(v: &str) -> bool {
    if v.is_ascii() {return true}
    return false
}

pub fn contains(v: &str, pat: &str) -> bool {
    if v.contains(pat) {return true}
    return false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    return v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    return v.find(pat).unwrap()
}