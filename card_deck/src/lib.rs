use rand::Rng;
#[derive(Eq, PartialEq, Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let num: u8 = rand::thread_rng().gen_range(1u8, 5u8);
        Suit::translate(num)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => Suit::Heart,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let num: u8 = rand::thread_rng().gen_range(1u8, 14u8);
        Rank::translate(num)
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
            13 => Rank::King,
            _ => Rank::Number(2),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}
