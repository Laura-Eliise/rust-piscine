fn main() {
	let s1 = String::from("helloWorld");
	let s2 = String::from("snake_case");
	let s3 = String::from("CamelCase");
	let s4 = String::from("just");

	println!("first_subword({}) = {}", s1.clone(), first_subword(s1));
	println!("first_subword({}) = {}", s2.clone(), first_subword(s2));
	println!("first_subword({}) = {}", s3.clone(), first_subword(s3));
	println!("first_subword({}) = {}", s4.clone(), first_subword(s4));
}

// $ cargo run
// first_subword(helloWorld) = hello
// first_subword(snake_case) = snake
// first_subword(CamelCase) = Camel
// first_subword(just) = just
// $

pub fn first_subword(s: String) -> String {
    let mut first = String::new();
    let mut index = 0;
    let arr: Vec<char> = s.chars().collect();
    for letter in arr {
        if letter == '_' || (index != 0 && letter.is_uppercase()) { break }
        index += 1;
        first.push(letter);
    }
    return first
}