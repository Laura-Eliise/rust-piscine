fn main() {
    // // if uncommented, the below line will give an expect "ERROR "
    // println!("{:?}", expect(vec![1, 3, 2, 5]));

    println!("{:?}", unwrap_or(vec![1, 3, 2, 5]));
    println!("{:?}", unwrap_or(vec![1, 3, 5]));

    println!("{:?}", unwrap_err(vec![1, 3, 2, 5]));

    // // if uncommented, the below line will give an unwraped error
    // println!("{:?}", unwrap_err(vec![1, 3, 5]));

    println!("{:?}", unwrap(vec![1, 3, 5]));

    //// if uncommented, the below line will give an error
    // println!("{:?}", unwrap(vec![1, 3, 2, 5]));

    println!("{:?}", unwrap_or_else(vec![1, 3, 5]));
    println!("{:?}", unwrap_or_else(vec![3, 2, 6, 5]));
}

pub fn odd_to_even(data: Vec<u32>) -> Result<Vec<u32>, (String, Vec<u32>)> {
    let mut a = Vec::new();
    a.extend(data.iter().filter(|&value| value % 2 == 0));
    if a.len() != 0 {
        return Err(("There is a even value in the vector!".to_string(), a));
    }
    a.extend(data.iter().map(|&value| {
        value + 1
    }));
    Ok(a)
}

pub fn expect(v: Vec<u32>) -> Vec<u32> {
    odd_to_even(v).expect("ERROR ")
}

pub fn unwrap_or(v: Vec<u32>) -> Vec<u32> {
    odd_to_even(v).unwrap_or(Vec<u32>::new())
}

pub fn unwrap_err(v: Vec<u32>) -> (String, Vec<u32>) {
    odd_to_even(v).unwrap_err()
}

pub fn unwrap(v: Vec<u32>) -> Vec<u32> {
    odd_to_even(v).unwrap()
}

fn c(x: (String, Vec<u32>)) -> Vec<u32> {x.1}

pub fn unwrap_or_else(v: Vec<u32>) -> Vec<u32> {
    odd_to_even(v).unwrap_or_else(c)
}
