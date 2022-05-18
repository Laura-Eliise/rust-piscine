use std::io::stdin;

fn main() {
    let mut tries: u64 = 0;
    let mut guess = String::new();
    let mut correct = String::from("The letter e\n");
    
    while guess != correct {
        tries = tries +1;
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        stdin().read_line(&mut guess).expect("Something went wrong");
    }
    println!("It took you {} trials to get the right answer", tries);
}