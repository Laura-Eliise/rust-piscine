fn main() {
    println!("{}", num_to_ordinal(1));
    println!("{}", num_to_ordinal(21));
    println!("{}", num_to_ordinal(43));
    println!("{}", num_to_ordinal(47));
}

pub fn num_to_ordinal(x: u32) -> String {
    let s = x.to_string();
    let lastTwo: String = s.chars().rev().take(2).collect();
    if lastTwo == "21" || lastTwo == "11" {return s + "th"}
    return s.clone() + match s.chars().last().unwrap() {
        '1' => "st",
        '2' => "nd",
        '3' => "rd",
        _ => "th"
    }
}