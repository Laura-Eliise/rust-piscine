fn main() {
    println!("{:?}", arrange_phrase("is2 Thi1s T4est 3a"));
}

pub fn arrange_phrase(phrase: &str) -> String {
    let arr = phrase.split(" ").collect::<Vec<&str>>();
    let mut ordered = vec![String::new(); arr.len()];

    for raw in arr {
        let index: usize;
        for letter in raw.chars() {
            if letter.is_digit(10) {
                index = letter.to_string().parse::<usize>().unwrap();
                ordered[index-1] = raw.replace(&index.to_string(), "");
                break
            }
        }
    }

    return String::from(ordered.join(" "))
}