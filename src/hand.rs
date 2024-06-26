use crate::tiles::TileHelpers;
use crate::tiles::{Tile, NumberTile, HonorTile, DragonTile, WindTile, Suit};
use crate::errors::errors::{ScoringError, ParsingError};

fn compose_hand() -> Result<Hand, ScoringError> {
    Err(ScoringError::Unimplemented)
}

#[derive(Debug)]
pub enum Hand {
    Standard{
        full_hand: FullHand,
        winning_tile: Tile,
        open: bool,
    },
    Chiitoi{
        full_hand: [Pair; 7],
        winning_tile: Tile,
    },
    Kokushi{
        full_hand: [Tile; 14],
        winning_tile: Tile,
    },
}

trait HandHelpers {
    fn count_suits(&self) -> Option<i8>;        // counts how many suits are present
    fn count_sequences(&self) -> Option<i8>;    // counts how many sequences are present
    fn count_dora(&self, dora_markers: Vec<Tile>) -> Option<i8>;    // counts how many dora are present
    fn count_dragons(&self) -> Option<i8>;  // counts dragons (in pair and melds)
    fn count_winds(&self) -> Option<i8>;    // counts all winds (in pair and melds)
    fn has_any_honor(&self) -> bool;        // checks for honor tiles
    fn has_any_terminal(&self) -> bool;     // checks for terminal tiles (1 & 9)
    fn has_any_simple(&self) -> bool;       // checks for simple tiles (2-8)
} 

impl HandHelpers for FullHand {
    fn count_suits(&self) -> Option<i8> {
        let mut suits: Vec<Suit> = vec![];
        None
    }
    fn count_sequences(&self) -> Option<i8> {
        let mut seqs: i8 = 0;
        None
    }
    fn count_dora(&self, dora_markers: Vec<Tile>) -> Option<i8> {
        let mut dora: i8 = 0;
        None
    }
    fn count_dragons(&self) -> Option<i8> {
        let mut dragons: i8 = 0;
        None
    }
    fn count_winds(&self) -> Option<i8> {
        let mut winds: i8 = 0;
        None
    }
    fn has_any_honor(&self) -> bool {
        //if self.pair.tile == Tile::HonorTile || self.melds.
        false
    }
    fn has_any_terminal(&self) -> bool {
        false
    }
    fn has_any_simple(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct FullHand {
    melds: [Meld; 4],
    pair: Pair
}

trait MeldHelpers {
    fn has_honor(&self) -> bool;
    fn has_terminal(&self) -> bool;
}

impl MeldHelpers for Meld {
    fn has_honor(&self) -> bool{
        match self {
            Meld::Triplet(trip) => {
                match trip.tile {
                    Tile::Honor(_) => true,
                    _ => false
                }
            },
            Meld::Kan(kan) => {
                match kan.tile {
                    Tile::Honor(_) => true,
                    _ => false
                }
            },
            _ => false,
        }
    }
    fn has_terminal(&self) -> bool{
        match self {
            Meld::Sequence(seq) => {
                for tile in seq.tiles {
                    if tile.is_terminal() {
                        return true
                    }
                }
                false
            },
            Meld::Kan(kan) => {
                match kan.tile {
                    Tile::Number(tile) => tile.is_terminal(),
                    _ => false
                }
            },
            Meld::Triplet(trip) => {
                match trip.tile {
                    Tile::Number(tile) => tile.is_terminal(),
                    _ => false
                }
            },
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Meld {
    Triplet(Triplet),
    Sequence(Sequence),
    Kan(Kan),
}

#[derive(Debug, PartialEq)]
pub struct Triplet{
    open: bool,
    tile: Tile
}

#[derive(Debug, PartialEq)]
pub struct Sequence{
    open: bool,
    tiles: [NumberTile; 3]
}

#[derive(Debug, PartialEq)]
pub struct Kan{
    open: bool,
    tile: Tile
}

#[derive(Debug, PartialEq)]
pub struct Pair {
    open: bool,
    tile: Tile
}