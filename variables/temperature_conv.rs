fn main() {
	println!("{} F = {} C", -459.67, fahrenheit_to_celsius(-459.67)); // -273.15
	println!("{} C = {} F", 0.0, celsius_to_fahrenheit(0.0)); // -32
}

pub fn fahrenheit_to_celsius(f: f64) -> f64 { (f -32f64) *5f64/9f64 }

pub fn celsius_to_fahrenheit(c: f64) -> f64 { c *1.8f64 +32f64 }