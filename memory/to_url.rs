fn main() {
	let s = "Hello, world!";
	println!("{} to be use as an url is {}", s, to_url(s));
}

pub fn to_url(s: &str) -> String {
    return s.to_string().replace(" ", "%20")
}