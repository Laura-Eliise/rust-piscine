fn main() {
    println!(
        "{:?}",
        tic_tac_toe(vec![
            vec!["O", "X", "O"],
            vec!["O", "O", "X"],
            vec!["X", "#", "X"]
        ])
    );
    // "Tie"
    println!(
        "{:?}",
        tic_tac_toe(vec![
            vec!["X", "O", "O"],
            vec!["X", "O", "O"],
            vec!["#", "O", "X"]
        ])
    );
    // "player O won"

    let dig = vec![
            vec!["O", "O", "X"],
            vec!["O", "X", "O"],
            vec!["X", "#", "X"]
        ];

    println!("{:?}",tic_tac_toe(dig));
    // "player X won"
    
    let dig = vec![
            vec!["O", "O", "X"],
            vec!["X", "X", "X"],
            vec!["O", "#", "O"]
        ];

    println!("{:?}",tic_tac_toe(dig));
    // "player X won"
}

pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if diagonals("X", table) /*|| horizontal("X", table) || vertical("X", table)*/ {
        return String::from("player X won")
    }

    // if diagonals("O", table) || horizontal("O", table) || vertical("O", table) {
    //     return "player O won"
    // }

    return String::from("Tie")
}

pub fn diagonals(p: &str, t: &Vec<Vec<&str>>) -> bool {
    // if t.0.0 + t.1.1 + t.2.2 == p.to_owned()+p+p {
    //     return true
    // }
    return false
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    return false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    return false
}