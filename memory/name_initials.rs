fn main() {
    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    println!("{:?}", initials(names));
}

pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut inits = Vec::<String>::new();
    for name in names {
        let mut init = String::new();
        for test in name.split(" ") {
            init.push_str(&(test.split_at(1).0.to_owned() +". "));
        }
        init.pop();
        inits.push(init);
    }
    return inits
}