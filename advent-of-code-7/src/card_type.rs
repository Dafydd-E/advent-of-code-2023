use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Hash)]
pub(crate) enum CardType {
    Joker = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl CardType {
    pub fn parse(input: char) -> Self {
        match input {
            'J' => Self::Joker,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Card type not found")
        }
    }

    pub fn parse_v1(input: char) -> Self {
        match input {
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Card type not found")
        }
    }
}

impl Eq for CardType {}

impl PartialEq<Self> for CardType {
    fn eq(&self, other: &Self) -> bool {
        let ot = *other as i32;
        let this = *self as i32;

        return this.eq(&ot);
    }
}

impl PartialOrd<Self> for CardType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let this = *self as i32;
        let ot = *other as i32;

        return Some(this.cmp(&ot));
    }
}

impl Ord for CardType {
    fn cmp(&self, other: &Self) -> Ordering {
        let this = *self as i32;
        let ot = *other as i32;

        return this.cmp(&ot);
    }
}
