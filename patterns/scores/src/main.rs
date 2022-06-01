fn main() {
    println!("{}", score("a"));
    println!("{}", score("ã ê Á?"));
    println!("{}", score("ThiS is A Test"));
}

pub fn score(s: &str) -> u64 {
    s.chars().map(|v| 
        match v.to_ascii_uppercase() {
            'A'|'E'|'I'|'O'|'U'|'L'|'N'|'R'|'S'|'T' => 1,
            'D'|'G' => 2,
            'B'|'C'|'M'|'P' => 3,
            'F'|'H'|'V'|'W'|'Y' => 4,
            'K' => 5,
            'J'|'X' => 8,
            'Q'|'Z' => 10,
            _ => 0
        }
    ).fold(0, |a, b| a + b)
}