fn main() {
    println!("'{}'", scytale_cypher(String::from("Scytale Code"), 6));
    println!("'{}'", scytale_cypher(String::from("Scytale Code"), 8));
}

pub fn scytale_cypher(s: String, size: u32) -> String {
    let message:Vec<char> = s.chars().collect();
    let range: usize = ((message.len() as f64) /(size as f64)).ceil() as usize;
    let mut ans: String = String::new();
    let mut index: usize = 0;

    for i in 0..size {
        for _ in 0..range {
            let place = (i as usize) +index;

            if place >= message.len() {
                ans += " "
            } else {
                ans += &message[place].to_string()
            }
            
            index += size as usize;
        }
        index = 0;
    }

    return ans.trim().to_string()
}