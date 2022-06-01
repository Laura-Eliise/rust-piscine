fn main() {
    println!("{}", num_to_ordinal(1));
    println!("{}", num_to_ordinal(22));
    println!("{}", num_to_ordinal(43));
    println!("{}", num_to_ordinal(47));
}

pub fn num_to_ordinal(x: u32) -> String {
    let s = x.to_string();
    return s.clone() + match s.chars().last().unwrap() {
        '1' => "st",
        '2' => "nd",
        '3' => "rd",
        _ => "th"
    }
}