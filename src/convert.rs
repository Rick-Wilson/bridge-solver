//! Conversion between bridge-types and internal solver types.
//!
//! The solver uses optimized internal representations (bitboards) for performance.
//! This module provides conversion at the API boundary.

use super::cards::*;
use super::hands::Hands;
use super::types::*;

/// Convert a bridge_types::Deal to solver Hands
impl Hands {
    /// Create Hands from a bridge_types::Deal
    pub fn from_deal(deal: &bridge_types::Deal) -> Self {
        let mut hands = Hands::new();

        // Map bridge_types Direction to solver Seat
        // bridge_types: North, East, South, West
        // solver: WEST=0, NORTH=1, EAST=2, SOUTH=3
        let directions = [
            (bridge_types::Direction::North, NORTH),
            (bridge_types::Direction::East, EAST),
            (bridge_types::Direction::South, SOUTH),
            (bridge_types::Direction::West, WEST),
        ];

        for (dir, seat) in directions {
            let hand = deal.hand(dir);
            for card in hand.cards() {
                let solver_card = convert_card(card);
                hands[seat].add(solver_card);
            }
        }

        hands
    }
}

/// Convert a bridge_types::Card to solver card index
fn convert_card(card: &bridge_types::Card) -> usize {
    let suit = convert_suit(card.suit);
    let rank = convert_rank(card.rank);
    card_of(suit, rank)
}

/// Convert bridge_types::Suit to solver Suit
/// bridge_types: Clubs=0, Diamonds=1, Hearts=2, Spades=3
/// solver: SPADE=0, HEART=1, DIAMOND=2, CLUB=3
fn convert_suit(suit: bridge_types::Suit) -> Suit {
    match suit {
        bridge_types::Suit::Spades => SPADE,
        bridge_types::Suit::Hearts => HEART,
        bridge_types::Suit::Diamonds => DIAMOND,
        bridge_types::Suit::Clubs => CLUB,
    }
}

/// Convert bridge_types::Rank to solver Rank
/// bridge_types: Two=2, Three=3, ..., Ace=14
/// solver: TWO=0, THREE=1, ..., ACE=12
fn convert_rank(rank: bridge_types::Rank) -> Rank {
    (rank as usize) - 2
}

/// Convert solver Seat to bridge_types::Direction
pub fn seat_to_direction(seat: Seat) -> bridge_types::Direction {
    match seat {
        NORTH => bridge_types::Direction::North,
        EAST => bridge_types::Direction::East,
        SOUTH => bridge_types::Direction::South,
        WEST => bridge_types::Direction::West,
        _ => unreachable!(),
    }
}

/// Convert bridge_types::Direction to solver Seat
pub fn direction_to_seat(dir: bridge_types::Direction) -> Seat {
    match dir {
        bridge_types::Direction::North => NORTH,
        bridge_types::Direction::East => EAST,
        bridge_types::Direction::South => SOUTH,
        bridge_types::Direction::West => WEST,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bridge_types::Deal;

    #[test]
    fn test_from_deal() {
        let pbn = "N:AKQT3.J6.KJ42.95 652.AK42.AQ87.T4 J74.QT95.T.AK863 98.873.9653.QJ72";
        let deal = Deal::from_pbn(pbn).unwrap();
        let hands = Hands::from_deal(&deal);

        // Each hand should have 13 cards
        assert_eq!(hands[NORTH].size(), 13);
        assert_eq!(hands[EAST].size(), 13);
        assert_eq!(hands[SOUTH].size(), 13);
        assert_eq!(hands[WEST].size(), 13);

        // Total should be 52
        assert_eq!(hands.all_cards().size(), 52);
    }

    #[test]
    fn test_from_deal_matches_from_pbn() {
        let pbn_str = "N:AKQT3.J6.KJ42.95 652.AK42.AQ87.T4 J74.QT95.T.AK863 98.873.9653.QJ72";

        // Parse via bridge_types
        let deal = Deal::from_pbn(pbn_str).unwrap();
        let hands_from_deal = Hands::from_deal(&deal);

        // Parse directly
        let hands_from_pbn = Hands::from_pbn(pbn_str).unwrap();

        // Should be identical
        assert_eq!(hands_from_deal, hands_from_pbn);
    }

    #[test]
    fn test_direction_conversion() {
        assert_eq!(direction_to_seat(bridge_types::Direction::North), NORTH);
        assert_eq!(direction_to_seat(bridge_types::Direction::East), EAST);
        assert_eq!(direction_to_seat(bridge_types::Direction::South), SOUTH);
        assert_eq!(direction_to_seat(bridge_types::Direction::West), WEST);

        assert_eq!(seat_to_direction(NORTH), bridge_types::Direction::North);
        assert_eq!(seat_to_direction(EAST), bridge_types::Direction::East);
        assert_eq!(seat_to_direction(SOUTH), bridge_types::Direction::South);
        assert_eq!(seat_to_direction(WEST), bridge_types::Direction::West);
    }
}
