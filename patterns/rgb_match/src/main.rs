#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let mut ans = self.clone();

        if self.r == first {ans.r = second}
        if self.g == first {ans.g = second}
        if self.b == first {ans.b = second}
        if self.a == first {ans.a = second}

        if self.r == second {ans.r = first}
        if self.g == second {ans.g = first}
        if self.b == second {ans.b = first}
        if self.a == second {ans.a = first}

        return ans
    }
}

fn main() {
    let c = Color {
        r: 255,
        g: 200,
        b: 10,
        a: 30,
    };

    println!("{:?}", c.swap(c.r, c.b));
    println!("{:?}", c.swap(c.r, c.g));
    println!("{:?}", c.swap(c.r, c.a));
    println!();
    println!("{:?}", c.swap(c.g, c.r));
    println!("{:?}", c.swap(c.g, c.b));
    println!("{:?}", c.swap(c.g, c.a));
    println!();
    println!("{:?}", c.swap(c.b, c.r));
    println!("{:?}", c.swap(c.b, c.g));
    println!("{:?}", c.swap(c.b, c.a));
    println!();
    println!("{:?}", c.swap(c.a, c.r));
    println!("{:?}", c.swap(c.a, c.b));
    println!("{:?}", c.swap(c.a, c.g));
}