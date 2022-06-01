use rand::Rng;

pub enum Suit {
    u8,
    str,
}

pub enum Rank {
    u8,
    str,
}

impl Suit {
	pub fn random() -> Suit {
        return translate(rand::thread_rng().gen_range(1..4))
	}

	pub fn translate(value: u8) -> Suit {
        match value {

        }
	}
}

impl Rank {
	pub fn random() -> Rank {
	}

	pub fn translate(value: u8) -> Rank {
	}
}

pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
}

fn main() {
	let your_card = Card {
		rank: Rank::random(),
		suit: Suit::random(),
	};

	println!("Your card is {:?}", your_card);

	// Now if the card is an Ace of Spades print "You are the winner"
	if card_deck::winner_card(your_card) {
		println!("You are the winner!");
	}
}

pub fn winner_card(car: Card) -> bool{
    if car.suit.translate() == "Spade" && 
       car.rank.translate() == "Ace" {
        return true
    }
    return false
}