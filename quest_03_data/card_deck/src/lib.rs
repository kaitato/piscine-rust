use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Diamond,
    Heart,
    Club,
    Spade,
}

#[derive(Debug, PartialEq)]

pub enum Rank {
    Ace,
    Number(u8),
    King,
    Queen,
    Jack,
}

impl Suit {
	pub fn random() -> Suit {
        let suit = rand::thread_rng().gen_range(0,4);
        Suit::translate(suit)
	}

	pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        }
	}
}

impl Rank {
	pub fn random() -> Rank {
        let rank = rand::thread_rng().gen_range(0,14);
        Rank::translate(rank)
	}

	pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2 => Rank::Number(2),
            3 => Rank::Number(3),
            4 => Rank::Number(4),
            5 => Rank::Number(5),
            6 => Rank::Number(6),
            7 => Rank::Number(7),
            8 => Rank::Number(8),
            9 => Rank::Number(9),
            10 => Rank::Number(10),
            11 => Rank::Jack,
            12 => Rank::Queen,
            _ => Rank::King,
        }
	}
}

#[derive(Debug, PartialEq)]

pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    if card.suit == Suit::Spade && card.rank == Rank::Ace {
        true
    } else {
        false
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
// 	let your_card = Card {
// 		rank: Rank::random(),
// 		suit: Suit::random(),
// 	};

// 	println!("Your card is {:?}", your_card);

// 	// Now if the card is an Ace of Spades print "You are the winner"
// 	if card_deck::winner_card(&your_card) {
// 		println!("You are the winner!");
// 	}
//     }
// }
