use crate::tiles::{Tile, Dragon, Wind, Suit, TileHelpers, DoraOf};
use crate::errors::errors::{ScoringError, ParsingError, CompositionError};
use crate::yaku::{Yaku, YakuSpecial, WinType};

///////////////////////
// structs and enums //
///////////////////////

#[derive(Debug)]
pub enum Hand {
    Standard{
        full_hand: FullHand,
        winning_tile: Tile,
        open: bool,
        yaku: Vec<Yaku>
    },
    Chiitoi{
        full_hand: [Pair; 7],
        winning_tile: Tile,
        yaku: Vec<Yaku>
    },
    Kokushi{
        full_hand: [Tile; 14],
        winning_tile: Tile,
        yaku: Vec<Yaku>
    },
}

#[derive(Debug)]
pub struct FullHand {
    pub melds: [Meld; 4],
    pub pair: Pair
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pair {
    pub tile: Tile
}

// wrapper enum for recursion in compose_tiles()
#[derive(Debug, PartialEq, Clone)]
pub enum MeldOrPair {
    Meld(Meld),
    Pair(Pair)
}

#[derive(Debug, PartialEq)]
pub enum PartialHand {
    Valid(Vec<MeldOrPair>),
    Invalid
}

///////////////
// functions //
///////////////

fn compose_hand(
    closed_tiles: Vec<Tile>,
    called_tiles: Vec<Meld>,
    winning_tile: Tile // relevant for yaku testing
) -> Result<Hand, ScoringError> {
    Err(ScoringError::Unimplemented)
}

// only for standard hands
fn compose_tiles(remaining_tiles: Vec<Tile>, remaining_kans: i8, remaining_pairs: i8, remaining_sets: i8) -> Option<Vec<PartialHand>> {
    if remaining_sets == 0 { 
        if remaining_tiles.is_empty() { return None } else { return Some(vec![PartialHand::Invalid]) }
    }
    
    let mut partials: Vec<PartialHand> = vec![];
    let mut subset: Vec<Tile> = remaining_tiles[1..].to_vec();
    
    if remaining_tiles[1 ..].contains(&remaining_tiles[0]) {
        let dupe_count = remaining_tiles.count_occurances(remaining_tiles[0]);
        if dupe_count >= 2 && remaining_pairs > 0 {
            let temp: MeldOrPair = MeldOrPair::Pair(Pair{tile: remaining_tiles[0]});
            let mut subs: Vec<Tile> = subset.clone();
            
            subs.remove_x_occurances(remaining_tiles[0], 1);

            let recursions: Option<Vec<PartialHand>> = compose_tiles(subs, remaining_kans, remaining_pairs - 1, remaining_sets - 1);
            if recursions.is_some() {
                for mut hand in recursions.unwrap() {
                    if let PartialHand::Valid(value) = hand {
                        partials.push(PartialHand::Valid([vec![temp.clone()], value].concat()));
                    }
                }
            } else {
                partials.push(PartialHand::Valid(vec![temp]));
            }
        }
        if dupe_count >= 3 {
            let temp: MeldOrPair = MeldOrPair::Meld(Meld::Triplet{tile: remaining_tiles[0], open: false});
            let mut subs: Vec<Tile> = subset.clone();
            
            subs.remove_x_occurances(remaining_tiles[0], 2);
            
            let recursions: Option<Vec<PartialHand>> = compose_tiles(subs, remaining_kans, remaining_pairs, remaining_sets - 1);
            if recursions.is_some() {
                for mut hand in recursions.unwrap() {
                    if let PartialHand::Valid(value) = hand {
                        partials.push(PartialHand::Valid([vec![temp.clone()], value].concat()));
                    } 
                }
            } else {
                partials.push(PartialHand::Valid(vec![temp]));
            }  
        }
        if dupe_count == 4 && remaining_kans > 0 {
            // this code should never be used, since kans are always called! there's no such thing as an uncalled kan
            // I'm keeping it in here for completeness, I guess? going through compose_hand() will prevent it from ever being called.
            let temp: MeldOrPair = MeldOrPair::Meld(Meld::Kan{tile: remaining_tiles[0], open: false});
            let mut subs: Vec<Tile> = subset.clone();
            
            // as only four copies of each tile exist, removing *every* occurence is appropriate
            subs.retain(|x| *x != remaining_tiles[0]);

            let recursions: Option<Vec<PartialHand>> = compose_tiles(subs, remaining_kans - 1, remaining_pairs, remaining_sets - 1);
            if recursions.is_some() {
                for mut hand in recursions.unwrap() {
                    if let PartialHand::Valid(value) = hand {
                        partials.push(PartialHand::Valid([vec![temp.clone()], value].concat()));
                    }
                }
            } else {
                partials.push(PartialHand::Valid(vec![temp]));
            }  
        }
    }

    if remaining_tiles[0].is_numbered() {
        let possible_sequences: Vec<[Tile; 2]> = remaining_tiles[0].adjacent_all();
        
        for seq in possible_sequences {
            let mut subset: Vec<Tile> = remaining_tiles[1..].to_vec();
            if subset.contains(&seq[0]) && subset.contains(&seq[1]) {
                let temp: MeldOrPair = MeldOrPair::Meld(Meld::Sequence{tiles: [remaining_tiles[0], seq[0], seq[1]], open: false});

                subset.remove(subset.iter().position(|x| *x == seq[0])?);
                subset.remove(subset.iter().position(|x| *x == seq[1])?);

                let recursions: Option<Vec<PartialHand>> = compose_tiles(subset, remaining_kans - 1, remaining_pairs, remaining_sets - 1);
                if recursions.is_some() {
                    for mut hand in recursions.unwrap() {
                        if let PartialHand::Valid(value) = hand {
                            partials.push(PartialHand::Valid([vec![temp.clone()], value].concat()));
                        }
                    }
                } else {
                    partials.push(PartialHand::Valid(vec![temp]));
                }
            }
        } 
    }

    if partials.is_empty() { return Some(vec![PartialHand::Invalid]) } else { return Some(partials) }
}

fn compose_chiitoi() -> Result<[Pair; 7], CompositionError> {
    Err(CompositionError::NotImplemented)
}

////////////
// traits //
////////////

pub trait HandTools {
    fn has_any_honor(&self) -> bool;    // checks for honor tiles
    fn has_any_terminal(&self) -> bool; // checks for terminal tiles (1 & 9)
    fn has_any_simple(&self) -> bool;   // checks for simple tiles (2-8)
    fn as_tiles(&self) -> Vec<Tile>;    // gets an array of tiles, mostly for dora counting
    fn count_dora(&self, dora_markers: Vec<Tile>) -> i8;    // counts how many dora are present
}

impl HandTools for Hand {
    fn has_any_honor(&self) -> bool {
        match self {
            Hand::Standard {full_hand, ..} => {
                full_hand.has_any_honor()
            },
            Hand::Chiitoi {full_hand, ..} => {
                for pair in full_hand {
                    if pair.has_honor() { return true }
                }
                false
            },
            Hand::Kokushi {full_hand, ..} => {
                // this should always be true, but let's sanity check it ...
                for tile in full_hand {
                    if !tile.is_honor() { return true }
                }
                false
            }
        }
    }
    fn has_any_terminal(&self) -> bool {
        match self {
            Hand::Standard {full_hand, ..} => {
                full_hand.has_any_terminal()
            },
            Hand::Chiitoi {full_hand, ..} => {
                for pair in full_hand {
                    if pair.has_terminal() { return true }
                }
                false
            }, 
            Hand::Kokushi {full_hand, ..} => {
                // this should always be true, but let's sanity check it ...
                for tile in full_hand {
                    if tile.is_terminal() { return true }
                }
                false
            }
        }
    }
    fn has_any_simple(&self) -> bool {
        match self {
            Hand::Standard {full_hand, ..} => {
                full_hand.has_any_simple()
            },
            Hand::Chiitoi {full_hand, ..} => {
                for pair in full_hand {
                    if !pair.has_honor() || !pair.has_terminal() { return true }
                }
                false
            },
            Hand::Kokushi {full_hand, ..} => {
                // this should always be false, but let's sanity check it ...
                for tile in full_hand {
                    if !tile.is_terminal() || !tile.is_honor() { return true }
                }
                false
            }
        }
    }
    fn as_tiles(&self) -> Vec<Tile> {
        match self {
            Hand::Standard {full_hand, ..} => full_hand.as_tiles(),
            Hand::Chiitoi {full_hand, ..} => {
                vec![ // ugly, but it works
                    full_hand[0].tile, full_hand[0].tile,
                    full_hand[1].tile, full_hand[1].tile,
                    full_hand[2].tile, full_hand[2].tile,
                    full_hand[3].tile, full_hand[3].tile,
                    full_hand[4].tile, full_hand[4].tile,
                    full_hand[5].tile, full_hand[5].tile,
                    full_hand[6].tile, full_hand[6].tile,
                ]
            },
            Hand::Kokushi {full_hand, ..} => (*full_hand).to_vec()
        }
    }
    fn count_dora(&self, dora_markers: Vec<Tile>) -> i8 {
        let mut dora_count: i8 = 0;
        let vec: Vec<Tile> = self.as_tiles();
        for marker in dora_markers {
            let dora = marker.dora();
            dora_count += vec.iter().fold(0, |acc, value| { if *value == dora {acc + 1} else {acc} })
        }
        dora_count
    }
}

pub trait HandHelpers {  
    fn count_sequence_suits(&self) -> i8; // count how many suits are present in ...
    fn count_triplet_suits(&self) -> i8;  // only sequences or only suits.
    fn count_sequences(&self) -> i8;    // counts how many sequences are present
    fn count_triplets(&self) -> i8;     // counts how many triplets are present
    fn count_kans(&self) -> i8;         // counts how many kans are present
    fn count_dragons(&self) -> i8;      // counts dragons (in pair and melds)
    fn count_winds(&self) -> i8;        // counts all winds (in pair and melds)
    fn has_any_honor(&self) -> bool;    // checks for honor tiles
    fn has_any_terminal(&self) -> bool; // checks for terminal tiles (1 & 9)
    fn has_any_simple(&self) -> bool;   // checks for simple tiles (2-8)
    fn only_sequences(&self) -> Vec<Meld>;      // returns only sequences
    fn without_sequences(&self) -> Vec<Meld>;   // returns only triplets and kans
    fn as_tiles(&self) -> Vec<Tile>;   // pulls tiles out of melds/pairs into an array
    fn only_closed(&self) -> Vec<MeldOrPair>;   // only the closed part of the hand
} 

impl HandHelpers for FullHand {
    fn count_sequences(&self) -> i8 {
        let mut seqs: i8 = 0;
        for meld in self.melds {
            if let Meld::Sequence{..} = meld { seqs += 1 }
        }
        seqs
    }
    fn count_sequence_suits(&self) -> i8 {
        let mut suits: Vec<Suit> = vec![];
        for meld in self.only_sequences() {

        }
        suits.len() as i8
    }
    fn count_triplet_suits(&self) -> i8 {
        let mut suits: Vec<Suit> = vec![];
        for meld in self.without_sequences() {

        }
        suits.len() as i8
    }
    fn count_triplets(&self) -> i8 {
        let mut trips: i8 = 0;
        for meld in self.melds {
            match meld {
                Meld::Triplet {..} | Meld::Kan {..} => trips += 1,
                _ => ()
            }
        }
        trips
    }
    fn count_kans(&self) -> i8 {
        let mut kans: i8 = 0;
        for meld in self.melds {
            if let Meld::Kan{..} = meld { kans += 1 }
        }
        kans
    }
    fn count_dragons(&self) -> i8 {
        let mut dragons: i8 = 0;
        for meld in &self.melds {
            if meld.is_dragon() { dragons += 1 }
        }
        dragons
    }
    fn count_winds(&self) -> i8 {
        let mut winds: i8 = 0;
        for meld in &self.melds {
            if meld.is_wind() { winds += 1 }
        }
        winds
    }
    fn has_any_honor(&self) -> bool {
        for meld in &self.melds {
            if meld.has_honor() { return true }
        }
        false
    }
    fn has_any_terminal(&self) -> bool {
        for meld in &self.melds {
            if meld.has_terminal() { return true }
        }
        false
    }
    fn has_any_simple(&self) -> bool {
        for meld in &self.melds {
            if !meld.has_honor() && !meld.has_terminal() { return true }
        }
        false
    }
    fn only_sequences(&self) -> Vec<Meld> {
        let mut vec: Vec<Meld> = vec![];
        for meld in &self.melds {
            if let &Meld::Sequence {..} = meld { vec.push(*meld) }
        }
        vec
    }
    fn without_sequences(&self) -> Vec<Meld> {
        let mut vec: Vec<Meld> = vec![];
        for meld in &self.melds {
            if let &Meld::Sequence {..} = meld { vec.push(*meld) }
        }
        vec
    }

    fn as_tiles(&self) -> Vec<Tile> {
        let mut vec: Vec<Tile> = vec![];
        vec.push(self.pair.tile); vec.push(self.pair.tile);
        for meld in self.melds {
            match meld {
                Meld::Sequence {tiles, ..} => {
                    vec.push(tiles[0]); vec.push(tiles[1]); vec.push(tiles[2]);
                },
                Meld::Triplet {tile, ..} => {
                    for _ in 0..3 { vec.push(tile); }
                },
                Meld::Kan {tile, ..} => {
                    for _ in 0..4 { vec.push(tile); }
                },
            }
        }
        vec
    }

    fn only_closed(&self) -> Vec<MeldOrPair> {
        let mut vec: Vec<MeldOrPair> = vec![];
        vec.push(MeldOrPair::Pair(self.pair));
        for meld in self.melds {
            match meld {
                Meld::Triplet {open, ..} | Meld::Kan {open, ..} | Meld::Sequence {open, ..}
                    => if !open { vec.push(MeldOrPair::Meld(meld))},
                _ => ()
            }
        }
        vec
    }
}

pub trait MeldHelpers {
    fn has_honor(&self) -> bool;
    fn has_terminal(&self) -> bool;
    fn is_dragon(&self) -> bool;
    fn is_wind(&self) -> bool;
    fn contains_tile(&self, tile: Tile) -> bool;
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
    fn has_terminal(&self) -> bool {
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
    fn is_dragon(&self) -> bool {
        match self {
            Meld::Kan {open, tile} | Meld::Triplet {open, tile} => {
                if let Tile::Dragon(_) = tile { true } else { false }
            }
            _ => false
        }
    }
    fn is_wind(&self) -> bool {
        match self {
            Meld::Kan {open, tile} | Meld::Triplet {open, tile} => {
                if let Tile::Wind(_) = tile { true } else { false }
            }
            _ => false
        }
    }
    fn contains_tile(&self, t: Tile) -> bool {
        match self {
            Meld::Kan {tile, ..} | Meld::Triplet {tile, ..} => t == *tile,
            Meld::Sequence {tiles, ..} => {
                for tile in tiles {
                    if t == *tile { return true }
                }
                false
            }
        }
    }
}

impl MeldHelpers for Pair {
    fn has_honor(&self) -> bool {
        match self.tile {
            Tile::Wind(_) | Tile::Dragon(_) => true,
            _ => false
        }
    }
    fn has_terminal(&self) -> bool {
        self.tile.is_terminal()
    }
    fn is_dragon(&self) -> bool {
        if let Tile::Dragon(_) = self.tile { true } else { false }
    }
    fn is_wind(&self) -> bool {
        if let Tile::Wind(_) = self.tile { true } else { false }
    }
    fn contains_tile(&self, t: Tile) -> bool {
        self.tile == t
    }
}

pub trait SequenceHelpers {
    fn as_numbers(&self) -> [i8; 3];
    fn ittsuu_viable(&self) -> bool;
    fn is_middle(&self, tile: Tile) -> bool;
}

impl SequenceHelpers for Meld {
    fn as_numbers(&self) -> [i8; 3] {
        if let Meld::Sequence {tiles, ..} = self {
            let mut a: [i8; 3] = [tiles[0].get_number().unwrap(), tiles[1].get_number().unwrap(), tiles[2].get_number().unwrap()];
            a.sort();
            a
        } else { panic!("called as_numbers() on a non-sequence meld!") } 
    }
    fn ittsuu_viable(&self) -> bool {
        match self.as_numbers()[0] {
            1 | 4 | 7 => true,
            _ => false
        }
    }
    fn is_middle(&self, tile: Tile) -> bool {
        if let Meld::Sequence {tiles, ..} = self {
            let mut a: [Tile; 3] = tiles.clone();
            a.sort();
            a[1] == tile
        } else { panic!("called is_middle() on a non-sequence meld!") }
    }
}

pub trait VecTileHelpers {
    fn count_occurances(&self, tile: Tile) -> i8;
    fn remove_x_occurances(&mut self, tile: Tile, count: i8) -> ();
}

impl VecTileHelpers for Vec<Tile> {
    fn count_occurances(&self, tile: Tile) -> i8 {
        self.iter().fold(0, |acc, value| {if tile == *value { acc + 1 } else { acc }})
    }
    fn remove_x_occurances(&mut self, tile: Tile, count: i8) -> () {
        for i in 0..count {
            self.remove(self.iter().position(|x| *x == tile).unwrap());
        }
    }
}

mod tests {
    use super::*;
    use crate::tiles::{Tile, Dragon, Wind, Suit, TileHelpers, DoraOf};

    #[test]
    fn test_reading_hand_composition(){
        assert_eq!(compose_tiles(crate::tiles::make_tiles_from_string("dw,dw,dw,we,we,we").unwrap(), 0, 0, 2).unwrap(),
            vec![PartialHand::Valid(vec![ MeldOrPair::Meld(Meld::Triplet{tile: Tile::Dragon(Dragon::White), open: false }), 
                                          MeldOrPair::Meld(Meld::Triplet{tile: Tile::Wind(Wind::East), open: false })])]
            );
        assert_eq!(compose_tiles(crate::tiles::make_tiles_from_string("dw,dw,dw,we,we").unwrap(), 0, 1, 2).unwrap(),
            vec![PartialHand::Valid(vec![ MeldOrPair::Meld(Meld::Triplet{tile: Tile::Dragon(Dragon::White), open: false }), 
                                          MeldOrPair::Pair(Pair{tile: Tile::Wind(Wind::East) }) ] )]
            );
        assert_eq!(compose_tiles(crate::tiles::make_tiles_from_string("dw,dw,we,we,we").unwrap(), 0, 1, 2).unwrap(),
            vec![PartialHand::Valid(vec![ MeldOrPair::Pair(Pair{tile: Tile::Dragon(Dragon::White) }),
                                          MeldOrPair::Meld(Meld::Triplet{tile: Tile::Wind(Wind::East), open: false }), ] )]
            );
        assert_eq!(compose_tiles(crate::tiles::make_tiles_from_string("m1,m2,m3,p4,p5r,p3").unwrap(), 0, 0, 2).unwrap(),
            vec![PartialHand::Valid(vec![ MeldOrPair::Meld(Meld::Sequence{tiles: [Tile::Number{ suit: Suit::Man, number: 1, red: false },
                                                                                  Tile::Number{ suit: Suit::Man, number: 2, red: false },
                                                                                  Tile::Number{ suit: Suit::Man, number: 3, red: false } ], open: false}),
                                          MeldOrPair::Meld(Meld::Sequence{tiles: [Tile::Number{ suit: Suit::Pin, number: 3, red: false },
                                                                                  Tile::Number{ suit: Suit::Pin, number: 4, red: false },
                                                                                  Tile::Number{ suit: Suit::Pin, number: 5, red: true } ], open: false}) ])]
            );
        assert_eq!(compose_tiles(crate::tiles::make_tiles_from_string("dw,dr,p4,dw,p5r,p3,dr,dw,m2,m2,m2,m2").unwrap(), 1, 1, 4).unwrap(), 
            vec![PartialHand::Valid(vec![ MeldOrPair::Pair(Pair{tile: Tile::Dragon(Dragon::Red) }),
                                          MeldOrPair::Meld(Meld::Triplet{tile: Tile::Dragon(Dragon::White), open: false }),
                                          MeldOrPair::Meld(Meld::Kan{tile: Tile::Number{ suit: Suit::Man, number: 2, red: false}, open: false }),
                                          MeldOrPair::Meld(Meld::Sequence{tiles: [Tile::Number{ suit: Suit::Pin, number: 3, red: false },
                                                                                  Tile::Number{ suit: Suit::Pin, number: 4, red: false },
                                                                                  Tile::Number{ suit: Suit::Pin, number: 5, red: true } ], open: false}) ] )] 
            );
        assert_eq!(compose_tiles(crate::tiles::make_tiles_from_string("m1,m1,m1,m2,m2,m2,m3,m3,m3").unwrap(), 0, 0, 3).unwrap(),
        vec![PartialHand::Valid(vec![ MeldOrPair::Meld(Meld::Triplet{tile: Tile::Number{ suit: Suit::Man, number: 1, red: false }, open: false }),
                                      MeldOrPair::Meld(Meld::Triplet{tile: Tile::Number{ suit: Suit::Man, number: 2, red: false }, open: false }),
                                      MeldOrPair::Meld(Meld::Triplet{tile: Tile::Number{ suit: Suit::Man, number: 3, red: false }, open: false }) ]),
             PartialHand::Valid(vec![ MeldOrPair::Meld(Meld::Sequence{tiles: [Tile::Number{ suit: Suit::Man, number: 1, red: false },
                                                                              Tile::Number{ suit: Suit::Man, number: 2, red: false },
                                                                              Tile::Number{ suit: Suit::Man, number: 3, red: false } ], open: false}),
                                      MeldOrPair::Meld(Meld::Sequence{tiles: [Tile::Number{ suit: Suit::Man, number: 1, red: false },
                                                                              Tile::Number{ suit: Suit::Man, number: 2, red: false },
                                                                              Tile::Number{ suit: Suit::Man, number: 3, red: false } ], open: false}),
                                      MeldOrPair::Meld(Meld::Sequence{tiles: [Tile::Number{ suit: Suit::Man, number: 1, red: false },
                                                                              Tile::Number{ suit: Suit::Man, number: 2, red: false },
                                                                              Tile::Number{ suit: Suit::Man, number: 3, red: false } ], open: false}),])]
            );
    }

    #[test]
    fn test_dora_count(){
        //TODO: add test cases once I don't need to manually generate hands
    }
}