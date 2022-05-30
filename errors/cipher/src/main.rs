#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    validation: bool,
    expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation: validation,
            expected: expected,
        }
    }
}


fn main() {
    println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
    println!("{:?}", cipher("1Hello 2world!", "svool"));
    println!("{:?}", cipher("", "svool"));
}

// Some(Ok(true))
// Some(Err(CipherError { validation: false, expected: "1Svool 2dliow!" }))
// None

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original == "" { return None }
    
    let new: String = original.chars().map( |letter| {
        if letter.is_alphabetic() {
            if letter.is_ascii_uppercase() {
                (b'A' + b'Z' - letter as u8) as char
            } else {
                (b'a' + b'z' - letter as u8) as char
            }
        } else {
            letter
        }
    }).collect();

    if new == ciphered {
        return Some(Ok(true))
    } else {
        return Some(Err(CipherError::new(false, new)))
    }
}