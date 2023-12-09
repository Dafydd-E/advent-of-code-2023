use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use crate::card_type::CardType;

#[derive(Clone, Debug)]
pub(crate) struct Card {
    pub(crate) card_type: CardType,
    impersonating: Option<CardType>
}

impl Card {
    pub(crate) fn parse(input: char) -> Self {
        Self { card_type: CardType::parse(input), impersonating: None }
    }

    pub(crate) fn impersonate(&self, card_type: CardType) -> Self {
        Self { card_type: self.card_type, impersonating: Some(card_type) }
    }

    pub(crate) fn impersonate_mut(&mut self, card_type: CardType) {
        self.impersonating = Some(card_type);
    }
}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.card_type.eq(&other.card_type);
    }
}

impl Eq for Card {

}

impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.card_type.hash(state);
        if self.impersonating.is_some() {
            self.impersonating.hash(state);
        }
    }
}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.card_type.partial_cmp(&other.card_type);
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.card_type.cmp(&other.card_type);
    }
}