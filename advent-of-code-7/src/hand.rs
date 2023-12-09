use std::cmp::Ordering;
use std::collections::BTreeMap;
use itertools::Itertools;
use crate::card::Card;
use crate::card_type::CardType;
use crate::hand_type::HandType;
use crate::hand_type::HandType::{FiveOfAKind, FourOfAKind, FullHouse, Pair, ThreeOfAKind, HighCard};
use crate::handing::Handing;

#[derive(Clone, Debug, Default)]
pub(crate) struct Hand {
    pub(crate) value: String,
    cards: Handing,
    pub(crate) hand_type: Option<HandType>,
    high_card: Option<CardType>,
}

impl Hand {
    pub(crate) fn parse(input: String) -> Self {
        let mut cards: BTreeMap<Card, usize> = BTreeMap::new();
        for character in input.chars().into_iter() {
            let card = Card::parse(character);
            let mut existing = cards.get_mut(&card);
            if let Some(ex) = existing {
                *ex = *ex + (1usize);
                existing = Some(ex);
            } else {
                cards.insert(card, 1);
            }
        }

        Self { cards, high_card: None, hand_type: None, value: input }
    }
}

impl Hand {
    pub(crate) fn get_hand_type_mut(&mut self) -> HandType {
        if let Some(hand_type) = &self.hand_type {
            return (*hand_type).clone();
        }

        if self.value.contains("J") {
            let jacks : Vec<(&Card, &usize)> = self.cards.iter()
                .filter(|(ref x, _)| x.card_type == CardType::Joker)
                .collect_vec();
            let other_cards = &self.cards.iter()
                .filter(|(x, _)| x.card_type != CardType::Joker)
                .map(|(x, size)| (x, size))
                .collect_vec();

            if jacks.iter().any(|(_, &amount)| amount == 5)  {
                self.hand_type = Some(FiveOfAKind);
                return (*self).hand_type.clone().unwrap();
            }

            let remaining_cards = self.value.clone().replace("J", "");
            let mut handing = Hand::parse(remaining_cards);
            let ref handing_type = handing.get_hand_type_mut();
            let jack_count = jacks.last().unwrap().1;
            match (jack_count, handing_type) {
                (5, _) => {
                    self.hand_type = Some(FiveOfAKind);
                }
                (4, _) => {
                    self.hand_type = Some(FiveOfAKind);
                    if let Some((ref other, _)) = other_cards.last() {
                        for (jack, _) in jacks {
                            jack.impersonate(other.card_type);
                        }
                    } else {
                        panic!("uh oh");
                    }
                }
                (3, Pair) => {
                    self.hand_type = Some(FiveOfAKind);
                    if let Some((ref other, _)) = other_cards.last() {
                        for (jack, _) in jacks {
                            jack.impersonate(other.card_type);
                        }
                    } else {
                        panic!("uh oh");
                    }
                }
                (3, HighCard) => {
                    self.hand_type = Some(FourOfAKind);
                    let (other, _) = other_cards.iter().max_by_key(|(x, _)| x.card_type).unwrap();
                    for (jack, _) in jacks {
                        jack.impersonate(other.card_type);
                    }
                }
                (2, ThreeOfAKind) => {
                    self.hand_type = Some(FiveOfAKind);
                    if let Some((ref other, _)) = other_cards.last() {
                        for (jack, _) in jacks {
                            jack.impersonate(other.card_type);
                        }
                    } else {
                        panic!("uh oh");
                    }
                }
                (2, Pair) => {
                    self.hand_type = Some(FourOfAKind);
                    let (ref pair, _) = other_cards.iter().filter(|(_, &amount)| amount == 2).last().unwrap();
                    for (jack, _) in jacks {
                        jack.impersonate(pair.card_type);
                    }
                }
                (2, HighCard) => {
                    self.hand_type = Some(ThreeOfAKind);
                    let (other, _) = other_cards.iter().max_by_key(|(x, _)| x.card_type).unwrap();
                    for (jack, _) in jacks {
                        jack.impersonate(other.card_type);
                    }
                }
                (1, FourOfAKind) => {
                    self.hand_type = Some(FiveOfAKind);
                    if let Some((ref other, _)) = other_cards.last() {
                        for (jack, _) in jacks {
                            jack.impersonate(other.card_type);
                        }
                    } else {
                        panic!("uh oh");
                    }
                }
                (1, ThreeOfAKind) => {
                    self.hand_type = Some(FourOfAKind);
                    let (ref pair, _) = other_cards.iter().filter(|(_, &amount)| amount == 3).last().unwrap();
                    for (jack, _) in jacks {
                        jack.impersonate(pair.card_type);
                    }
                }
                (1, HandType::TwoPair) => {
                    self.hand_type = Some(FullHouse);
                    let (other, _) = other_cards.iter().max_by_key(|(x, _)| x.card_type).unwrap();
                    for (jack, _) in jacks {
                        jack.impersonate(other.card_type);
                    }
                }
                (1, Pair) => {
                    self.hand_type = Some(ThreeOfAKind);
                    let (ref pair, _) = other_cards.iter().filter(|(_, &amount)| amount == 2).last().unwrap();
                    for (jack, _) in jacks {
                        jack.impersonate(pair.card_type);
                    }
                }
                (1, HighCard) => {
                    self.hand_type = Some(Pair);
                    let (other, _) = other_cards.iter().max_by_key(|(x, _)| x.card_type).unwrap();
                    for (jack, _) in jacks {
                        jack.impersonate(other.card_type);
                    }
                }
                (_, _) => {
                    panic!("invalid card configuration");
                }
            }
        } else {
            if let Some((_, &count)) = self.cards.iter().max_by_key(|(_, &amount)| amount) {
                match count {
                    5 => {
                        self.hand_type = Some(FiveOfAKind);
                    }
                    4 => {
                        self.hand_type = Some(FourOfAKind);
                    }
                    3 => {
                        if *(self.cards.iter().min_by_key(|(_, &amount)| amount).unwrap().1) == 2 {
                            self.hand_type = Some(FullHouse);
                        } else {
                            self.hand_type = Some(ThreeOfAKind);
                        }
                    }
                    2 => {
                        if self.cards.iter().counts_by(|(&ref x, _)| x).len() == (3 - (5 - self.value.len())) {
                            self.hand_type = Some(HandType::TwoPair);
                        } else {
                            self.hand_type = Some(Pair);
                        }
                    }
                    1 => {
                        self.hand_type = Some(HighCard);
                    }
                    _ => panic!("Something went wrong.")
                };
            } else {
                panic!("uh oh");
            }
        }

        return (*self).hand_type.clone().unwrap();
    }

    pub(crate) fn get_high_card(&mut self) -> CardType {
        if let Some(high_card) = &self.high_card {
            return high_card.clone();
        }

        let (high_card, _) = self.cards.iter().max_by_key(|(_, &amount)| amount).unwrap();
        self.high_card = Some(high_card.card_type);
        return (*self).high_card.clone().unwrap();
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type.is_none() || other.hand_type.is_none() {
            panic!("need to call get_hand_type before comparing");
        }
        let hand_type_compare = self.hand_type.cmp(&other.hand_type);
        if hand_type_compare.is_eq() {
            let mut iterator = other.value.chars().into_iter().map(|x| CardType::parse(x));
            let mut other_card = iterator.next().unwrap();
            for card in self.value.chars().into_iter().map(|x| CardType::parse(x)) {
                let ord = card.cmp(&other_card);
                if ord.is_eq() {
                    let next = iterator.next();
                    if next.is_none() {
                        continue;
                    }
                    other_card = next.unwrap();
                    continue;
                }

                return Some(ord);
            }
        }

        return Some(hand_type_compare);
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(&other).unwrap();
    }
}