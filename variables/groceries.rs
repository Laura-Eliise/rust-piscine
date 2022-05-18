fn main() {
	let mut groceries = vec![
		"yogurt".to_string(),
		"panetone".to_string(),
		"bread".to_string(),
		"cheese".to_string(),
	];
	insert(&mut groceries, String::from("nuts"));
	println!("The groceries list now = {:?}", &groceries);
	println!(
		"The second element of the grocery  list is {:?}",
		at_index(&groceries, 1)
	);
}

pub fn insert(vec: &mut Vec<String>, val: String) { vec.push(val) }

pub fn at_index(vec: &Vec<String>, index: usize) -> String { vec.get(index).unwrap().to_string() }