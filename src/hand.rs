use crate::tiles::{Tile, Dragon, Wind, Suit, TileHelpers};
use crate::errors::errors::{ScoringError, ParsingError};

///////////////////////
// structs and enums //
///////////////////////

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

#[derive(Debug)]
pub struct FullHand {
    pub melds: [Meld; 4],
    pub pair: Pair
}

#[derive(Debug, PartialEq)]
pub enum Meld {
    Triplet{
        open: bool,
        tile: Tile
    },
    Sequence{
        open: bool,
        tiles: [Tile; 3]
    },
    Kan{
        open: bool,
        tile: Tile
    },
}

#[derive(Debug, PartialEq)]
pub struct Triplet{
    pub open: bool,
    pub tile: Tile
}

#[derive(Debug, PartialEq)]
pub struct Sequence{
    pub open: bool,
    pub tiles: [Tile; 3]
}

#[derive(Debug, PartialEq)]
pub struct Kan{
    pub open: bool,
    pub tile: Tile
}

#[derive(Debug, PartialEq)]
pub struct Pair {
    pub open: bool,
    pub tile: Tile
}

///////////////
// functions //
///////////////

fn compose_hand() -> Result<Hand, ScoringError> {
    Err(ScoringError::Unimplemented)
}

////////////
// traits //
////////////

pub trait HandTools {
    fn has_any_honor(&self) -> bool;    // checks for honor tiles
    fn has_any_terminal(&self) -> bool; // checks for terminal tiles (1 & 9)
    fn has_any_simple(&self) -> bool;   // checks for simple tiles (2-8)
}

impl HandTools for Hand {
    fn has_any_honor(&self) -> bool {
        false
    }
    fn has_any_terminal(&self) -> bool {
        false
    }
    fn has_any_simple(&self) -> bool {
        false
    }
}


pub trait HandHelpers {
    fn count_suits(&self) -> i8;        // counts how many suits are present
    fn count_sequences(&self) -> i8;    // counts how many sequences are present
    fn count_triplets(&self) -> i8;     // counts how many triplets are present
    fn count_kans(&self) -> i8;         // counts how many kans are present
    fn count_dora(&self, dora_markers: Vec<Tile>) -> i8;    // counts how many dora are present
    fn count_dragons(&self) -> i8;      // counts dragons (in pair and melds)
    fn count_winds(&self) -> i8;        // counts all winds (in pair and melds)
    fn has_any_honor(&self) -> bool;    // checks for honor tiles
    fn has_any_terminal(&self) -> bool; // checks for terminal tiles (1 & 9)
    fn has_any_simple(&self) -> bool;   // checks for simple tiles (2-8)
} 

impl HandHelpers for FullHand {
    fn count_suits(&self) -> i8 {
        let mut suits: Vec<Suit> = vec![];
        suits.len() as i8
    }
    fn count_sequences(&self) -> i8 {
        let mut seqs: i8 = 0;
        seqs
    }
    fn count_triplets(&self) -> i8 {
        let mut trips: i8 = 0;
        trips
    }
    fn count_kans(&self) -> i8 {
        let mut kans: i8 = 0;
        kans
    }
    fn count_dora(&self, dora_markers: Vec<Tile>) -> i8 {
        let mut dora: i8 = 0;
        dora
    }
    fn count_dragons(&self) -> i8 {
        let mut dragons: i8 = 0;
        dragons
    }
    fn count_winds(&self) -> i8 {
        let mut winds: i8 = 0;
        winds
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

pub trait MeldHelpers {
    fn has_honor(&self) -> bool;
    fn has_terminal(&self) -> bool;
}

impl MeldHelpers for Meld {
    fn has_honor(&self) -> bool{
        match self {
            Meld::Triplet {open, tile} => {
                match tile {
                    Tile::Wind(_) | Tile::Dragon(_) => true,
                    _ => false
                }
            },
            Meld::Kan {open, tile} => {
                match tile {
                    Tile::Wind(_) | Tile::Dragon(_) => true,
                    _ => false
                }
            },
            _ => false,
        }
    }
    fn has_terminal(&self) -> bool{
        match self {
            Meld::Sequence {open, tiles} => {
                for tile in tiles {
                    if tile.is_terminal() {
                        return true
                    }
                }
                false
            },
            Meld::Kan {open, tile} => {
                tile.is_terminal()
            },
            Meld::Triplet {open, tile} => {
                tile.is_terminal()
            },
            _ => false,
        }
    }
}