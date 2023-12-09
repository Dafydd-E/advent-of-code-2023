#[derive(Clone, PartialOrd, PartialEq, Debug, Eq, Ord)]
pub enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    Pair = 1,
    HighCard = 0,
}
