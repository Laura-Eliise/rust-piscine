// Work in progress

fn main() {
	let mut s = String::from("Hello");

	println!("Before changing the string: {}", s);

	doubtful(&s);

	println!("After changing the string: {}", s);
}

pub fn doubtful<T>(s: T) where T: std::fmt::Display {
    s.to_string().push('?')
}