fn main() {
    let a: i32 = 0;
    let b = String::from("1 2 4 5 6");
    let c = vec![1, 2, 4, 5];

    println!("{:?}", nbr_function(a));
    // output : (0, 1.0, inf)
    println!("{:?}", str_function(b));
    // output : ("1 2 4 5 6", "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351")
    println!("{:?}", vec_function(c));
    //  output : ([1, 2, 4, 5], [0.0, 0.6931471805599453, 1.3862943611198906, 1.6094379124341003])
}

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    return ( c.clone(), f64::from(c.clone()).exp(), (c.clone() as f64).ln().abs() )
}

pub fn str_function(a: String) -> (String, String) {
    let mut expo = String::new();
    for num in a.clone().split(" ") {
        expo.push_str(num.parse::<f64>().unwrap().exp().to_string().as_str());
        expo.push(' ');
    }
    expo.pop();
    return ( a.clone().into(), expo )
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut expo = Vec::<f64>::new();
    for num in b.clone() {
        expo.push((num as f64).ln().abs());
    }
    return ( b.clone(), expo )
}