fn main() {
    let new_str = String::from("5.5k 8.9k 32");

    // creating a variable and we save it in the Heap
    let a_h = transform_and_save_on_heap(new_str);
    println!("Box value : {:?}", &a_h);
    println!("size occupied in the stack : {:?} bytes", (std::mem::size_of_val(&a_h)));

    let a_b_v = take_value_ownership(a_h);
    println!("value : {:?}", &a_b_v);
    println!("size occupied in the stack : {:?} bytes", (std::mem::size_of_val(&a_b_v)));
    // whenever the box, in this case "a_h", goes out of scope it will be deallocated, freed
}

pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut arr: Vec<u32> = Vec::new();
    for v in s.split(" ") {
        let mut num = v.to_string();
        let size: f64 = if v.chars().last().unwrap() == 'k' {
            num.pop();
            1000f64
        } else {
            1f64
        };
        println!("{} {}", num, size);
        arr.push((num.parse::<f64>().unwrap()*size) as u32)
    }

    return Box::new(arr)
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}