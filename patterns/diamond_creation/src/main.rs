fn main() {
    println!("{:?}", get_diamond('A'));
    println!("{:?}", get_diamond('C'));
    let dia = get_diamond('D');
    for l in dia {
        println!("{}", l);
    }
    // println!("{:?}", get_diamond('D'));
}

pub fn get_diamond(c: char) -> Vec<String> {
    let mut dia = Vec::new();
    let mut halfway = false;
    let mut center: usize = 0;
    let space: usize = ((c as u32) - ('A' as u32)).try_into().unwrap();

    if space == 0 { return vec!["A".to_string()] } 

    while dia.len() < space*2+1 {
        if dia.len() == 0 || dia.len() == space*2 {
            dia.push(String::from(
                " ".repeat(space) + 
                "A" + 
                &" ".repeat(space)
            ))
        } else {
            dia.push(String::from(
                " ".repeat(space-center) +
                &char::from_u32(('A' as u32) + (center as u32)).unwrap().to_string() +
                &" ".repeat(center*2-1) +
                &char::from_u32(('A' as u32) + (center as u32)).unwrap().to_string() +
                &" ".repeat(space-center)
            ))
        }

        if center == space { 
            halfway = true 
        }
        if halfway && dia.len() < space*2+1 { 
            center -= 1
        } else {
            center += 1
        }
    }

    return dia
}