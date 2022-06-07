#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Game {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nbr_of_games: u16
}

impl Game {
    pub fn new(i: u32, pl1: String, pl2: String, n: u16) -> Box<Game> {
        return Box::new(Game {
            id: i,
            p1: (pl1, 0),
            p2: (pl2, 0),
            nbr_of_games: n
        })
    }
    pub fn read_winner(&self) -> (String, u16) {
        if self.p1.1 ==  self.p2.1 {
            return ("Same score! tied".to_string(), self.p1.1)
        } else if self.p1.1 > self.p2.1 {
            return self.p1.clone()
        }
        return self.p2.clone()
    }
    pub fn update_score(&mut self, user_name: String) {
        if self.nbr_of_games > 0 {
            if self.p1.0 == user_name {
                self.p1.1 += 1
            } else {
                self.p2.1 += 1
            }
            self.nbr_of_games -= 1
        }
    }
    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}

fn main() {
    let mut game = Game::new(0, String::from("Joao"), String::from("Susana"), 5);
    println!("{:?}", game.read_winner());
    // output : ("Same score! tied", 0)

    game.update_score(String::from("Joao"));
    game.update_score(String::from("Joao"));
    game.update_score(String::from("Susana"));
    game.update_score(String::from("Susana"));
    println!("{:?}", game.read_winner());
    // output : ("Same score! tied", 2)

    game.update_score(String::from("Joao"));
    // this one will not count because it already 5 games played, the nbr_of_games
    game.update_score(String::from("Susana"));

    println!("{:?}", game.read_winner());
    // output : ("Joao", 3)

    println!("{:?}", game.delete());
    // output : "game deleted: id -> 0"

    // game.read_winner();
    // this will give error
    // because the game was dropped, no longer exists on the heap
}