use std::cmp::Ordering;
use crate::hand::Hand;

#[derive(Debug, Clone)]
pub(crate) struct HandWithBid {
    pub(crate) hand: Hand,
    pub(crate) bid: usize
}

impl HandWithBid {
    pub(crate) fn parse(input: String) -> Self {
        let mut split = input.split(" ");
        let value = split.nth(0).unwrap().to_string();
        let bid = split.nth(0).unwrap().to_string().parse::<usize>().unwrap();

        Self { hand: Hand::parse(value), bid}
    }
}

impl PartialEq<Self> for HandWithBid {
    fn eq(&self, other: &Self) -> bool {
        return self.hand.value == other.hand.value && self.bid == other.bid;
    }
}

impl Eq for HandWithBid {}

impl PartialOrd<Self> for HandWithBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.hand.cmp(&(other.hand)));
    }
}

impl Ord for HandWithBid {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.hand.cmp(&(other.hand));
    }
}