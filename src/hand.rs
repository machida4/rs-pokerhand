use crate::card::{Card, Rank, Suit};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum StraightType {
    AceLow,
    Other,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight(StraightType),
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush(StraightType),
    RoyalFlush,
}

#[derive(Debug, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: &[Card]) -> Result<Self, ()> {
        if cards.len() == 5 {
            Ok(Hand {
                cards: cards.to_vec(),
            })
        } else {
            Err(())
        }
    }

    pub fn hand_type(&self) -> HandType {
        let mut sorted_hand = self.cards.clone();
        sorted_hand.sort();
        let sorted_hand = SortedHand { cards: sorted_hand };

        if let Some(hand_type) = sorted_hand.is_royal_flush() {
            hand_type
        } else if let Some(hand_type) = sorted_hand.is_straight_flush() {
            hand_type
        } else if let Some(hand_type) = sorted_hand.is_four_of_a_kind() {
            hand_type
        } else if let Some(hand_type) = sorted_hand.is_full_house() {
            hand_type
        } else if let Some(hand_type) = sorted_hand.is_flush() {
            hand_type
        } else if let Some(hand_type) = sorted_hand.is_straight() {
            HandType::Straight(hand_type)
        } else if let Some(hand_type) = sorted_hand.is_three_of_a_kind() {
            hand_type
        } else if let Some(hand_type) = sorted_hand.is_two_pair() {
            hand_type
        } else if let Some(hand_type) = sorted_hand.is_pair() {
            hand_type
        } else {
            HandType::HighCard
        }
    }
}

pub struct SortedHand {
    cards: Vec<Card>,
}

impl SortedHand {
    pub fn is_royal_flush(&self) -> Option<HandType> {
        match &self.cards[..] {
            [Card(Rank::Ten, _), Card(Rank::Jack, _), Card(Rank::Queen, _), Card(Rank::King, _), Card(Rank::Ace, _)]
                if self.is_flush().is_some() =>
            {
                Some(HandType::RoyalFlush)
            }
            _ => None,
        }
    }

    pub fn is_straight_flush(&self) -> Option<HandType> {
        if self.is_flush().is_some() && self.is_straight().is_some() {
            Some(HandType::StraightFlush(StraightType::Other))
        } else {
            None
        }
    }

    pub fn is_four_of_a_kind(&self) -> Option<HandType> {
        match &self.cards[..] {
            [Card(r1, _), Card(r2, _), Card(r3, _), Card(r4, _), _]
                if r1 == r2 && r2 == r3 && r3 == r4 =>
            {
                Some(HandType::FourOfAKind)
            }
            [_, Card(r2, _), Card(r3, _), Card(r4, _), Card(r5, _)]
                if r2 == r3 && r3 == r4 && r4 == r5 =>
            {
                Some(HandType::FourOfAKind)
            }
            _ => None,
        }
    }

    pub fn is_full_house(&self) -> Option<HandType> {
        match &self.cards[..] {
            [Card(r1, _), Card(r2, _), Card(r3, _), Card(r4, _), Card(r5, _)]
                if (r1 == r2 && r2 == r3) && r4 == r5 =>
            {
                Some(HandType::FullHouse)
            }
            [Card(r1, _), Card(r2, _), Card(r3, _), Card(r4, _), Card(r5, _)]
                if r1 == r2 && (r3 == r4 && r4 == r5) =>
            {
                Some(HandType::FullHouse)
            }
            _ => None,
        }
    }

    pub fn is_flush(&self) -> Option<HandType> {
        if let [Card(_, s1), Card(_, s2), Card(_, s3), Card(_, s4), Card(_, s5)] = &self.cards[..] {
            if [s1, s2, s3, s4, s5].iter().all(|&s| *s == *s1) {
                return Some(HandType::Flush);
            }
        }
        None
    }

    pub fn is_straight(&self) -> Option<StraightType> {
        match &self.cards[..] {
            [Card(Rank::Two, _), Card(Rank::Three, _), Card(Rank::Four, _), Card(Rank::Five, _), Card(Rank::Ace, _)] => {
                Some(StraightType::AceLow)
            }
            [Card(r1, _), Card(r2, _), Card(r3, _), Card(r4, _), Card(r5, _)]
                if (*r2 as u8) == (*r1 as u8 + 1)
                    && (*r3 as u8) == (*r2 as u8 + 1)
                    && (*r4 as u8) == (*r3 as u8 + 1)
                    && (*r5 as u8) == (*r4 as u8 + 1) =>
            {
                Some(StraightType::Other)
            }
            _ => None,
        }
    }

    pub fn is_three_of_a_kind(&self) -> Option<HandType> {
        match &self.cards[..] {
            [Card(r1, _), Card(r2, _), Card(r3, _), _, _] if r1 == r2 && r2 == r3 => {
                Some(HandType::ThreeOfAKind)
            }
            [_, Card(r2, _), Card(r3, _), Card(r4, _), _] if r2 == r3 && r3 == r4 => {
                Some(HandType::ThreeOfAKind)
            }
            [_, _, Card(r3, _), Card(r4, _), Card(r5, _)] if r3 == r4 && r4 == r5 => {
                Some(HandType::ThreeOfAKind)
            }
            _ => None,
        }
    }

    pub fn is_two_pair(&self) -> Option<HandType> {
        match &self.cards[..] {
            [Card(r1, _), Card(r2, _), Card(r3, _), Card(r4, _), _] if r1 == r2 && r3 == r4 => {
                Some(HandType::TwoPair)
            }
            [Card(r1, _), Card(r2, _), _, Card(r4, _), Card(r5, _)] if r1 == r2 && r4 == r5 => {
                Some(HandType::TwoPair)
            }
            [_, Card(r2, _), Card(r3, _), Card(r4, _), Card(r5, _)] if r2 == r3 && r4 == r5 => {
                Some(HandType::TwoPair)
            }
            _ => None,
        }
    }

    pub fn is_pair(&self) -> Option<HandType> {
        match &self.cards[..] {
            [Card(r1, _), Card(r2, _), _, _, _] if r1 == r2 => Some(HandType::Pair),
            [_, Card(r2, _), Card(r3, _), _, _] if r2 == r3 => Some(HandType::Pair),
            [_, _, Card(r3, _), Card(r4, _), _] if r3 == r4 => Some(HandType::Pair),
            [_, _, _, Card(r4, _), Card(r5, _)] if r4 == r5 => Some(HandType::Pair),
            _ => None,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type() != other.hand_type() {
            self.hand_type().partial_cmp(&other.hand_type())
        } else {
            self.cards.partial_cmp(&other.cards)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straight_flush() {
        let hand = Hand::new(&[
            Card(Rank::Two, Suit::Spade),
            Card(Rank::Three, Suit::Spade),
            Card(Rank::Four, Suit::Spade),
            Card(Rank::Five, Suit::Spade),
            Card(Rank::Six, Suit::Spade),
        ])
        .unwrap();
        assert_eq!(
            hand.hand_type(),
            HandType::StraightFlush(StraightType::Other)
        );
    }

    #[test]
    fn test_flush() {
        let hand = Hand::new(&[
            Card(Rank::Two, Suit::Spade),
            Card(Rank::Three, Suit::Spade),
            Card(Rank::Four, Suit::Spade),
            Card(Rank::Five, Suit::Spade),
            Card(Rank::Seven, Suit::Spade),
        ])
        .unwrap();
        assert_eq!(hand.hand_type(), HandType::Flush);
    }

    #[test]
    fn test_four_of_a_kind() {
        let hand = Hand::new(&[
            Card(Rank::Two, Suit::Spade),
            Card(Rank::Two, Suit::Heart),
            Card(Rank::Two, Suit::Diamond),
            Card(Rank::Two, Suit::Club),
            Card(Rank::Seven, Suit::Spade),
        ])
        .unwrap();
        assert_eq!(hand.hand_type(), HandType::FourOfAKind);
    }

    #[test]
    fn test_full_house() {
        let hand = Hand::new(&[
            Card(Rank::Two, Suit::Spade),
            Card(Rank::Two, Suit::Heart),
            Card(Rank::Two, Suit::Diamond),
            Card(Rank::Three, Suit::Club),
            Card(Rank::Three, Suit::Spade),
        ])
        .unwrap();
        assert_eq!(hand.hand_type(), HandType::FullHouse);
    }

    #[test]
    fn test_straight() {
        let hand = Hand::new(&[
            Card(Rank::Two, Suit::Spade),
            Card(Rank::Three, Suit::Spade),
            Card(Rank::Four, Suit::Spade),
            Card(Rank::Five, Suit::Spade),
            Card(Rank::Six, Suit::Heart),
        ])
        .unwrap();
        assert_eq!(hand.hand_type(), HandType::Straight(StraightType::Other));
    }

    #[test]
    fn test_ace_low_straight() {
        let hand = Hand::new(&[
            Card(Rank::Two, Suit::Spade),
            Card(Rank::Three, Suit::Spade),
            Card(Rank::Four, Suit::Spade),
            Card(Rank::Five, Suit::Spade),
            Card(Rank::Ace, Suit::Heart),
        ])
        .unwrap();
        assert_eq!(hand.hand_type(), HandType::Straight(StraightType::AceLow));
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand = Hand::new(&[
            Card(Rank::Two, Suit::Spade),
            Card(Rank::Two, Suit::Heart),
            Card(Rank::Two, Suit::Diamond),
            Card(Rank::Three, Suit::Club),
            Card(Rank::Four, Suit::Spade),
        ])
        .unwrap();
        assert_eq!(hand.hand_type(), HandType::ThreeOfAKind);
    }

    #[test]
    fn test_two_pair() {
        let hand = Hand::new(&[
            Card(Rank::Two, Suit::Spade),
            Card(Rank::Two, Suit::Heart),
            Card(Rank::Three, Suit::Diamond),
            Card(Rank::Three, Suit::Club),
            Card(Rank::Four, Suit::Spade),
        ])
        .unwrap();
        assert_eq!(hand.hand_type(), HandType::TwoPair);
    }

    #[test]
    fn test_pair() {
        let hand = Hand::new(&[
            Card(Rank::Two, Suit::Spade),
            Card(Rank::Two, Suit::Heart),
            Card(Rank::Three, Suit::Diamond),
            Card(Rank::Four, Suit::Club),
            Card(Rank::Five, Suit::Spade),
        ])
        .unwrap();
        assert_eq!(hand.hand_type(), HandType::Pair);
    }

    #[test]
    fn test_high_card() {
        let hand = Hand::new(&[
            Card(Rank::Two, Suit::Spade),
            Card(Rank::Three, Suit::Heart),
            Card(Rank::Four, Suit::Diamond),
            Card(Rank::Five, Suit::Club),
            Card(Rank::Seven, Suit::Spade),
        ])
        .unwrap();
        assert_eq!(hand.hand_type(), HandType::HighCard);
    }

    #[test]
    fn test_hand_strength_straight_flush_vs_flush() {
        let hand1 = Hand::new(&[
            Card(Rank::Two, Suit::Spade),
            Card(Rank::Three, Suit::Spade),
            Card(Rank::Four, Suit::Spade),
            Card(Rank::Five, Suit::Spade),
            Card(Rank::Six, Suit::Spade),
        ])
        .unwrap();

        let hand2 = Hand::new(&[
            Card(Rank::Two, Suit::Spade),
            Card(Rank::Three, Suit::Spade),
            Card(Rank::Four, Suit::Spade),
            Card(Rank::Five, Suit::Spade),
            Card(Rank::Seven, Suit::Spade),
        ])
        .unwrap();

        assert!(hand1 > hand2);
    }

    #[test]
    fn test_hand_ace_low_straight_vs_ace_high_straight() {
        let hand1 = Hand::new(&[
            Card(Rank::Two, Suit::Spade),
            Card(Rank::Three, Suit::Heart),
            Card(Rank::Four, Suit::Diamond),
            Card(Rank::Five, Suit::Club),
            Card(Rank::Ace, Suit::Spade),
        ])
        .unwrap();

        let hand2 = Hand::new(&[
            Card(Rank::Ten, Suit::Spade),
            Card(Rank::Jack, Suit::Heart),
            Card(Rank::Queen, Suit::Diamond),
            Card(Rank::King, Suit::Club),
            Card(Rank::Ace, Suit::Spade),
        ])
        .unwrap();

        assert!(hand1 < hand2);
    }

    #[test]
    fn test_hand_ace_low_straight_vs_standard_straight() {
        let hand1 = Hand::new(&[
            Card(Rank::Ace, Suit::Spade),
            Card(Rank::Two, Suit::Heart),
            Card(Rank::Three, Suit::Diamond),
            Card(Rank::Four, Suit::Club),
            Card(Rank::Five, Suit::Spade),
        ])
        .unwrap();

        let hand2 = Hand::new(&[
            Card(Rank::Two, Suit::Spade),
            Card(Rank::Three, Suit::Heart),
            Card(Rank::Four, Suit::Diamond),
            Card(Rank::Five, Suit::Club),
            Card(Rank::Six, Suit::Spade),
        ])
        .unwrap();

        assert!(hand1 < hand2);
    }
}
