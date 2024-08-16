use std::fmt;

pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn from_str(rank: &str) -> Option<Rank> {
        match rank {
            "2" => Some(Rank::Two),
            "3" => Some(Rank::Three),
            "4" => Some(Rank::Four),
            "5" => Some(Rank::Five),
            "6" => Some(Rank::Six),
            "7" => Some(Rank::Seven),
            "8" => Some(Rank::Eight),
            "9" => Some(Rank::Nine),
            "10" => Some(Rank::Ten),
            "J" => Some(Rank::Jack),
            "Q" => Some(Rank::Queen),
            "K" => Some(Rank::King),
            "A" => Some(Rank::Ace),
            _ => None,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_str = match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        write!(f, "{}", rank_str)
    }
}
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl Suit {
    fn from_str(suit: &str) -> Option<Suit> {
        match suit {
            "hearts" => Some(Suit::Hearts),
            "diamonds" => Some(Suit::Diamonds),
            "spades" => Some(Suit::Spades),
            "clubs" => Some(Suit::Spades),
            _ => None,
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit_str = match self {
            Suit::Hearts => "\u{2665}",
            Suit::Diamonds => "\u{2666}",
            Suit::Clubs => "\u{2663}",
            Suit::Spades => "\u{2660}",
        };
        write!(f, "{}", suit_str)
    }
}

pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }

    fn from_str(rank: &str, suit: &str) -> Result<Self, &'static str> {
        /* If Rank::from_str(rank) returns None,
        ok_or("Invalid rank") turns it into an Err("Invalid rank"),
        and ? makes the from_str method return that error immediately. */

        let rank = Rank::from_str(rank).ok_or("Invalid rank")?;
        let suit = Suit::from_str(suit).ok_or("Invalid suit")?;

        /* This is equivalent to:

           let rank = match Rank::from_str(rank) {
               Some(rank) => rank,
               None => return Err("Invalid rank"),
           };

           let suit = match Suit::from_str(suit) {
               Some(suit) => suit,
               None => return Err("Invalid suit"),
           };

        */
        Ok(Card::new(rank, suit))
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

#[cfg(test)]
mod test {
    use super::{Card, Rank, Suit};

    //  todo ("Write tests for Rank and Suit");

    #[test]
    fn basics() {
        let card = Card::new(Rank::Ace, Suit::Clubs);
        assert_eq!(card.to_string(), "\u{2663}A");

        let card = Card::from_str("88", "clubs");
        assert!(card.is_err());
    }
}
