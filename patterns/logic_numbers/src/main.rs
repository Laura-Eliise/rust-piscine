fn main() {
    let array = [9, 10, 153, 154];
    for pat in &array {
        if number_logic(*pat) == true {
            println!(
                "this number returns {} because the number {} obey the rules of the sequence",
                number_logic(*pat),
                pat
            )
        }
        if number_logic(*pat) == false {
            println!("this number returns {} because the number {} does not obey the rules of the sequence", number_logic(*pat),pat )
        }
    }
}

pub fn number_logic(num: u32) -> bool {
    let mut arr: Vec<u32> = vec![];
    let mut sum = 0;
    for raw in num.to_string().split("") { 
        if raw != "" {arr.push(raw.parse().unwrap())} 
    }
    for v in &arr { sum += v.pow(arr.len() as u32) }
    return sum == num
}