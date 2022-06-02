fn main() {
    println!("{}", pig_latin(&String::from("igloo")));
    println!("{}", pig_latin(&String::from("apple")));
    println!("{}", pig_latin(&String::from("hello")));
    println!("{}", pig_latin(&String::from("square")));
    println!("{}", pig_latin(&String::from("xenon")));
    println!("{}", pig_latin(&String::from("chair")));
}

pub fn pig_latin(mut text: &str) -> String {
    let first = text.to_uppercase().chars().next().unwrap();
    return match first {
        'A' | 'E' | 'I' | 'O' | 'U' => text.to_string(),
        _ => {
            let mut start: String = String::new();
            let mut index = 0;

            for l in text.split("") {
                if l == "" { continue }
                if l.starts_with(['a', 'e', 'i', 'o', 'u']) { break } 
                if l != "q" {
                    &start.push(text.to_string().remove(index));
                    index += 1;
                }
            }

            let qu: String = text.chars().nth(1).unwrap().to_string() + 
                             &text.chars().nth(2).unwrap().to_string();
            if qu == "qu" {
                text[index+2..].to_string() + &start + &qu   
            } else {
                text[index..].to_string() + &start
            }
        }
    } + "ay"
}