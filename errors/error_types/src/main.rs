pub use chrono::{Utc, NaiveDate};
use chrono::format::strftime::StrftimeItems;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FErr {
    form_values: (String, String),
    date: String,
    err: String
}

/// Error here with the date
impl FErr {
    pub fn new(name: String, error: String, err: String) -> FErr {
        let t = Utc::now();
        let fmt = StrftimeItems::new("%Y-%m-%d %H:%M:%S");
        FErr {
            form_values: (name, error),
            date: t.format_with_items(fmt.clone()).to_string(),
            err: err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub fav_color: Color,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        fav_color: Color,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name: first_name,
            last_name: last_name,
            birth: birth,
            fav_color: fav_color,
            birth_location: birth_location,
            password: password,
        }
    }
    
    pub fn validate(&self) -> Result<Vec<&str>, FErr> {
        let name = self.first_name.clone();
        if name == "" {
            return Err(FErr::new("first name".to_string(), name, "No user name".to_string()))
        }

        let password = self.password.clone();
        if password.chars().count() < 8 {
            return Err(FErr::new("password".to_string(), password, "At least 8 characters".to_string()))
        }

        let mut num: bool = false;
        let mut lett: bool = false;
        let mut nonLett: bool = false;
        for val in password.chars() {
            if val.is_digit(10) {
                num = true
            } else if val.is_alphabetic() {
                lett = true
            } else {
                nonLett = true
            }
        }
        if !num || !lett || !nonLett {
            return Err(FErr::new("password".to_string(), password, "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string()))
        }

        let mut resp: Vec<&str> = Vec::new();
        resp.push("Valid first name");
        resp.push("Valid password");
        return Ok(resp)
    }
}

fn main() {
    let mut form_output = Form::new(
        String::from("Lee"),
        String::from("Silva"),
        NaiveDate::from_ymd(2015, 9, 5),
        Color::Red,
        String::from("Africa"),
        String::from("qwqwsa1dty_"),
    );

    println!("{:?}", form_output);
    println!("{:?}", form_output.validate().unwrap());

    form_output.first_name = String::from("");
    println!("{:?}", form_output.validate().unwrap_err());

    form_output.first_name = String::from("as");
    form_output.password = String::from("dty_1");
    println!("{:?}", form_output.validate().unwrap_err());

    form_output.password = String::from("asdasASd(_");
    println!("{:?}", form_output.validate().unwrap_err());

    form_output.password = String::from("asdasASd123SA");
    println!("{:?}", form_output.validate().unwrap_err());
}

// Form { first_name: "Lee", last_name: "Silva", birth: 2015-09-05, fav_colour: Red, birth_location: "Africa", password: "qwqwsa1dty_" }
// ["Valid first name", "Valid password"]
// FErr { form_values: ("first_name", ""), date: "2022-04-21 09:18:12", err: "No user name" }
// FErr { form_values: ("password", "dty_1"), date: "2022-04-21 09:18:12", err: "At least 8 characters" }
// FErr { form_values: ("password", "asdasASd(_"), date: "2022-04-21 09:18:12", err: "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)" }
// FErr { form_values: ("password", "asdasASd123SA"), date: "2022-04-21 09:18:12", err: "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)" }