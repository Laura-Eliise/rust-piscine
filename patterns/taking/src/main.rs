fn main() {
    println!("{:?}", talking("JUST DO IT!"));
    println!("{:?}", talking("Hello how are you?"));
    println!("{:?}", talking("WHAT'S GOING ON?"));
    println!("{:?}", talking("something"));
    println!("{:?}", talking(""));
}

pub fn talking(text: &str) -> &str {
    let last: String = text.chars().rev().take(1).collect();
    if text == "" {return "Just say something!"}
    else if text == text.to_uppercase() {
        if last == "?" {return "Quiet, I am thinking!"}
        return "There is no need to yell, calm down!"
    }
    else if last == "?" {return "Sure."}
    return "Interesting"
}