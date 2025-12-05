use std::fs::read_to_string;

const POKER_FILE: &str = "poker.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Clubs,
    Spades,
    Diamond,
}

impl Suit {
    #[must_use]
    pub fn from_char(c: char) -> Self {
        match c {
            'H' => Self::Heart,
            'C' => Self::Clubs,
            'S' => Self::Spades,
            'D' => Self::Diamond,
            _ => panic!("Invalid suit {c}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CardValue {
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

impl CardValue {
    #[must_use]
    pub fn from_char(c: char) -> Self {
        match c {
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
            'K' => Self::King,
            'Q' => Self::Queen,
            'A' => Self::Ace,
            _ => panic!("Not a valid card value"),
        }
    }

    #[must_use]
    pub const fn next(self) -> Option<Self> {
        match self {
            Self::Two => Some(Self::Three),
            Self::Three => Some(Self::Four),
            Self::Four => Some(Self::Five),
            Self::Five => Some(Self::Six),
            Self::Six => Some(Self::Seven),
            Self::Seven => Some(Self::Eight),
            Self::Eight => Some(Self::Nine),
            Self::Nine => Some(Self::Ten),
            Self::Ten => Some(Self::Jack),
            Self::Jack => Some(Self::Queen),
            Self::Queen => Some(Self::King),
            Self::King => Some(Self::Ace),
            Self::Ace => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    suit: Suit,
    value: CardValue,
}

impl Card {
    #[must_use]
    pub fn parse_card(card: &str) -> Self {
        let mut char_iter = card.chars();
        let value = CardValue::from_char(char_iter.next().unwrap());
        let suit = Suit::from_char(char_iter.next().unwrap());
        Self { suit, value }
    }
}

pub struct Hand {
    cards: [Card; 5],
}

impl Hand {
    #[must_use]
    pub fn from_slice(cards: &[&str]) -> Self {
        Self {
            cards: cards
                .iter()
                .map(|c| Card::parse_card(c))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    #[must_use]
    pub fn card_vals_num(&self) -> Vec<(CardValue, u8)> {
        let mut vals = vec![];
        for card in self.cards {
            match vals.iter().position(|(c, _)| card.value == *c) {
                None => vals.push((card.value, 1)),
                Some(ind) => vals[ind] = (card.value, vals[ind].1 + 1),
            }
        }
        vals
    }

    #[must_use]
    pub fn value(&self) -> HandValue {
        let card_vals = self.card_vals_num();
        let vals: Vec<CardValue> = card_vals.iter().map(|(card, _)| card).copied().collect();
        // Royal Flush
        if card_vals.contains(&(CardValue::Ten, 1))
            && card_vals.contains(&(CardValue::Jack, 1))
            && card_vals.contains(&(CardValue::Queen, 1))
            && card_vals.contains(&(CardValue::King, 1))
            && card_vals.contains(&(CardValue::Ace, 1))
        {
            return HandValue {
                rank: HandRank::RoyalFlush,
                values: vals,
            };
        }

        // Straight Flush
        let suit_1 = self.cards[0].suit;
        let all_same_suit = self.cards.iter().all(|card| card.suit == suit_1);

        let mut lowest = *vals.iter().min().unwrap();
        let mut is_straight = true;
        for _ in 0..4 {
            lowest = if let Some(n) = lowest.next() {
                n
            } else {
                is_straight = false;
                break;
            };

            println!("{lowest:?}");
            is_straight = is_straight && vals.contains(&lowest);
        }

        if all_same_suit && is_straight {
            return HandValue {
                rank: HandRank::StraightFlush,
                values: vals,
            };
        }

        // Four of a Kind
        if let Some((val, _)) = card_vals.iter().find(|(_, num)| *num == 4) {
            return HandValue {
                rank: HandRank::FourOfAKind,
                values: vec![*val],
            };
        }

        // Full House
        if let (Some((val1, _)), Some((_, _))) = (
            card_vals.iter().find(|(_, num)| *num == 3),
            card_vals.iter().find(|(_, num)| *num == 2),
        ) {
            return HandValue {
                rank: HandRank::FullHouse,
                values: vec![*val1],
            };
        }

        // Flush
        if all_same_suit {
            return HandValue {
                rank: HandRank::Flush,
                values: vals,
            };
        }

        // Straight
        if is_straight {
            return HandValue {
                rank: HandRank::Straight,
                values: vals,
            };
        }

        // Three of a kind
        if let Some((val, _)) = card_vals.iter().find(|(_, num)| *num == 3) {
            return HandValue {
                rank: HandRank::ThreeOfAKind,
                values: vec![*val],
            };
        }

        //Two Pairs
        let pairs: Vec<CardValue> = card_vals
            .iter()
            .filter_map(|(card, num)| (*num == 2).then_some(card))
            .copied()
            .collect();
        if pairs.len() == 2 {
            return HandValue {
                rank: HandRank::TwoPairs,
                values: pairs,
            };
        }

        //One Pair
        if pairs.len() == 1 {
            return HandValue {
                rank: HandRank::OnePair,
                values: pairs,
            };
        }

        //High Card
        HandValue {
            rank: HandRank::HighCard,
            values: vals,
        }
    }
}

pub struct Game {
    player1: Hand,
    player2: Hand,
}

impl Game {
    #[must_use]
    pub fn parse_game(line: &str) -> Self {
        let mut cards1: Vec<&str> = line.split(' ').collect();
        let cards2 = cards1.split_off(5);
        let player1 = Hand::from_slice(&cards1);
        let player2 = Hand::from_slice(&cards2);
        Self { player1, player2 }
    }

    #[must_use]
    pub fn player1_win(&self) -> bool {
        let player1_value = self.player1.value();
        let player2_value = self.player2.value();
        if player1_value.rank == player2_value.rank {
            let mut player1_sorted = player1_value.values;
            player1_sorted.sort();
            player1_sorted.reverse();
            let mut player2_sorted = player2_value.values;
            player2_sorted.sort();
            player2_sorted.reverse();

            let max_ind = player1_sorted.len().min(player2_sorted.len());
            for ind in 0..max_ind {
                if player1_sorted[ind] > player2_sorted[ind] {
                    return true;
                }
                if player2_sorted[ind] > player1_sorted[ind] {
                    return false;
                }
            }

            let mut high_cards1 = self.player1.cards;
            high_cards1.sort_by(|card1, card2| card1.value.cmp(&card2.value));
            high_cards1.reverse();
            let mut high_cards2 = self.player2.cards;
            high_cards2.sort_by(|card1, card2| card1.value.cmp(&card2.value));
            high_cards2.reverse();

            for (card1, card2) in high_cards1.iter().zip(high_cards2.iter()) {
                if card1.value > card2.value {
                    return true;
                }

                if card2.value > card1.value {
                    return false;
                }
            }

            panic!("Cannot have draws")
        }
        player1_value.rank > player2_value.rank
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum HandRank {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

#[derive(Debug)]
pub struct HandValue {
    rank: HandRank,
    values: Vec<CardValue>,
}

fn load_hands() -> Vec<Game> {
    let contents = read_to_string(POKER_FILE).unwrap();
    let mut hands = Vec::with_capacity(1000);
    for line in contents.lines() {
        hands.push(Game::parse_game(line));
    }
    hands
}

fn test_game(game: &str) -> bool {
    let game = Game::parse_game(game);
    game.player1_win()
}

fn main() {
    let mut args = std::env::args();
    args.next();
    if let Some(arg) = args.next() {
        println!("{}", test_game(&arg));
        return;
    }

    let games = load_hands();
    let mut num_wins = 0;
    for game in games {
        if game.player1_win() {
            num_wins += 1;
        }
    }
    println!("Player 1 wins {num_wins}");
}

#[cfg(test)]
mod win_tests {
    use super::Game;

    #[test]
    fn test_hand1() {
        let game = Game::parse_game("5H 5C 6S 7S KD 2C 3S 8S 8D TD");
        assert!(!game.player1_win());
    }

    #[test]
    fn test_hand2() {
        let game = Game::parse_game("5D 8C 9S JS AC 2C 5C 7D 8S QH");
        assert!(game.player1_win());
    }

    #[test]
    fn test_hand3() {
        let game = Game::parse_game("2D 9C AS AH AC 3D 6D 7D TD QD");

        assert!(!game.player1_win());
    }

    #[test]
    fn test_hand4() {
        let game = Game::parse_game("4D 6S 9H QH QC 3D 6D 7H QD QS");
        assert!(game.player1_win());
    }

    #[test]
    fn test_hand5() {
        let game = Game::parse_game("2H 2D 4C 4D 4S 3C 3D 3S 9S 9D");
        assert!(game.player1_win());
    }

    #[test]
    fn test_random1() {
        let game = Game::parse_game("3C 8C 2D 7D 4D 9S 4S QH 4H JD");
        assert!(!game.player1_win());
    }

    #[test]
    fn test_random2() {
        let game = Game::parse_game("TD 2D QH 4S 6H JH KD 3C QD 8C");
        assert!(!game.player1_win());
    }

    #[test]
    fn test_random3() {
        let game = Game::parse_game("JC 6H 6S JS 4H QH 9H AH 4C 3C");
        assert!(game.player1_win());
    }

    #[test]
    fn test_random4() {
        let game = Game::parse_game("6H 4H 5C 3H 2H 3S QH 5S 6S AS");
        assert!(game.player1_win());
    }
}
