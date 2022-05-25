extern crate meval;

fn main() {
	let mut a = String::from("bpp--o+er+++sskroi-++lcw");
	let mut b: Vec<&str> = vec!["2+2=4", "3+2=5", "10-3=3", "5+5=10"];

	// - If a value does **not implement Copy**, it must be **borrowed** and so will be passed by **reference**.
	delete_and_backspace(&mut a); // the reference of  the value
	let per = is_correct(&mut b); // the reference of  the value

	println!("{:?}", (a, b, per));
	// output: ("borrow", ["✔", "✔", "✘", "✔"], 75)
}

pub fn delete_and_backspace(s: &mut String) {
    let mut index: usize = 0;
    for letter in s.clone().chars() {
        if letter == '-' { 
            s.replace_range(index-1..index+1, ""); 
            index -= 2 
        }
        index += 1;
    }
    index -= 1;
    while index > 0 {
        let letter= s.clone().as_bytes()[index] as char;
        if letter == '+' { 
            s.replace_range(index..index+2, ""); 
            index += 1 
        }
        index -= 1;
    }
}

pub fn is_correct(v: &mut Vec<&str>) -> usize {
    let mut index: usize = 0;
    let mut correct: usize = 0;
    let mut incorrect: usize = 0;
    for val in v.clone() {
        let arr = val.split(" = ").collect();
        if meval::eq_str(arr.0).unwrap() == arr.1 {
            correct += 1;
            v[index] = "✔";
        } else {
            incorrect += 1;
            v[index] = "✘";
        }
    }
    return correct/incorrect*100usize
}