use crate::tiles::{Tile, Dragon, Wind, Suit, TileHelpers};
use crate::errors::errors::{ScoringError, ParsingError, CompositionError};
use crate::yaku::{Yaku, YakuSpecial, WinType};
use crate::yaku;
use core::fmt;

///////////////////////
// structs and enums //
///////////////////////

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
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
    closed_tiles: Vec<Tile>, called_tiles: Option<Vec<Meld>>,
    // stuff we need for yaku testing
    winning_tile: Tile, win_type: WinType, seat_wind: Wind, round_wind: Wind,
) -> Result<Hand, ScoringError> {
    // if there are multiple ways to read the hand, we'll use this to decide which to return
    let mut possible_hands: Vec<Hand> = vec![];
    
    // first of all, test for strange hand shapes
    if closed_tiles.len() == 14 && called_tiles.is_none() {
        // thirteen orphans
        let kokushi: Option<Vec<Yaku>> = compose_kokushi(closed_tiles.clone(), winning_tile);
        if kokushi.is_some() {
            // a thirteen orphan hand can't be anything else, so we'll just return it
            return Ok(Hand::Kokushi{
                full_hand: closed_tiles.try_into().unwrap(),
                winning_tile: winning_tile,
                yaku: kokushi.unwrap()
            })
        }
        // seven pairs
        let chiitoi: Option<[Pair; 7]> = compose_chiitoi(closed_tiles.clone());
        if chiitoi.is_some() {
            possible_hands.push(
                Hand::Chiitoi{
                    full_hand: chiitoi.unwrap().try_into().unwrap(),
                    winning_tile: winning_tile, 
                    yaku: yaku::find_yaku_chiitoi(chiitoi.unwrap().try_into().unwrap(), winning_tile)?,
                }
            );
        }
    }

    // now we'll test for normal hand shapes
    let mut standard_hands: Vec<FullHand> = vec![];
    // the closed portion of the hand *must* contain the pair, and any melds which haven't been called. simple!
    let standard_partials: Option<Vec<PartialHand>> = compose_tiles(closed_tiles, 0, 1, (4 - called_tiles.unwrap_or_default().len()) as i8);

    if standard_partials.is_some() {
        for partial in standard_partials.unwrap() {
            if let PartialHand::Valid(partial_vec) = partial {
                let mut melds: Vec<Meld> = vec![];
                let mut pair: Pair;
                for meld_or_pair in partial_vec {
            }
            

            }
        }
    }

    match possible_hands.len() {
        0 => return Err(ScoringError::NoHands),
        1 => return Ok(possible_hands[0].clone()),
        _ => {
            for hand in possible_hands{
                println!(".{}", hand);
            }
            return Err(ScoringError::Unimplemented)
        },
    }
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

fn compose_chiitoi(
    mut tiles: Vec<Tile>
) -> Option<[Pair; 7]> {
    if tiles.len() != 14 { return None }
    tiles.sort();
    
    let mut pairs: Vec<Pair> = vec![];
    for i in 0..7 {
        if tiles[2*i] == tiles[(2*i)+1] && tiles[i] != tiles[i+2] {
            pairs.push(Pair{tile: tiles[i]});
        } else { return None }
    }

    if pairs.len() == 7 {
        return Some(pairs.try_into().unwrap())
    } else { return None }
}

fn compose_kokushi(
    tiles: Vec<Tile>,
    winning_tile: Tile
) -> Option<Vec<Yaku>> {
    let mut pair: bool = false;
    let mut won_on: bool = false;
    let mut tracker: Vec<Tile> = vec![];
    for tile in tiles {
        if tile.is_terminal() || tile.is_honor() {
            if !tracker.contains(&tile) { tracker.push(tile);
            } else if !pair {
                pair = true;
                if tile == winning_tile { won_on = true; }
            } else { break }
        } else { break }
    }
    if tracker.len() == 13 && pair {
        let mut yaku: Vec<Yaku> = vec![Yaku::Kokushi];
        if won_on { yaku.push(Yaku::SpecialWait) }
        return Some(yaku)
    }
    None
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
    fn get_yaku(&self) -> Vec<Yaku>;    // just to make my life a bit easier
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
        panic!("tiles in hands may lose or gain redness");
        let mut dora_count: i8 = 0;
        let vec: Vec<Tile> = self.as_tiles();
        for marker in dora_markers {
            let dora = marker.dora();
            dora_count += vec.iter().fold(0, |acc, value| { if *value == dora {acc + 1} else {acc} })
        }
        dora_count
    }
    fn get_yaku(&self) -> Vec<Yaku> {
        // TODO: find a better way to grab the field
        match self { Hand::Standard {yaku, ..} | Hand::Chiitoi {yaku, ..} | Hand::Kokushi {yaku, ..} => yaku.clone(), }
    }
}

pub trait HandHelpers {
    // I'm using this trait for both FullHand and [Pair];
    // functions which don't apply to [Pair] have a default implementation that just panics.
    // doing it this way feels a bit tidier.
    fn count_suits(&self) -> i8 {panic!()}         // count how many suits are present
    fn count_sequence_suits(&self) -> i8 {panic!()}// ... or just the ones in sequences
    fn count_triplet_suits(&self) -> i8 {panic!()} // ... or just the ones in triplets
    fn count_sequences(&self) -> i8 {panic!()}     // counts how many sequences are present
    fn count_triplets(&self) -> i8 {panic!()}      // counts how many triplets are present
    fn count_kans(&self) -> i8 {panic!()}          // counts how many kans are present
    fn count_dragons(&self) -> i8;          // counts types of dragon (in pair and melds)
    fn count_winds(&self) -> i8;            // counts types of wind (in pair and melds)
    fn has_any_honor(&self) -> bool;        // checks for honor tiles
    fn has_any_terminal(&self) -> bool;     // checks for terminal tiles (1 & 9)
    fn has_any_simple(&self) -> bool;       // checks for simple tiles (2-8)
    fn only_sequences(&self) -> Vec<Meld> {panic!()}    // returns only sequences
    fn without_sequences(&self) -> Vec<Meld> {panic!()} // returns only triplets and kans
    fn as_tiles(&self) -> Vec<Tile>;        // pulls tiles out of melds/pairs into an array
    fn only_closed(&self) -> Vec<MeldOrPair> {panic!()} // only the closed part of the hand
} 

impl HandHelpers for FullHand {
    fn count_suits(&self) -> i8 {
        let mut suits: Vec<Suit> = vec![];
        if let Tile::Number {suit, ..} = self.pair.tile { suits.push(suit) }
        for meld in self.melds { 
            if let Suit = meld.get_suit().unwrap() { 
                if !suits.contains(&Suit) { suits.push(Suit) } } }
        suits.len() as i8
    }
    fn count_sequences(&self) -> i8 {
        self.only_sequences().len() as i8
    }
    fn count_sequence_suits(&self) -> i8 {
        let mut suits: Vec<Suit> = vec![];
        for meld in self.only_sequences() {
            if let Suit = meld.get_suit().unwrap() { 
                if !suits.contains(&Suit) { suits.push(Suit) } } }
        suits.len() as i8
    }
    fn count_triplet_suits(&self) -> i8 {
        let mut suits: Vec<Suit> = vec![];
        for meld in self.without_sequences() {
            if let Suit = meld.get_suit().unwrap() { 
                if !suits.contains(&Suit) { suits.push(Suit) } } }
        suits.len() as i8
    }
    fn count_triplets(&self) -> i8 {
        let mut trips: i8 = 0;
        for meld in self.melds {
            match meld {
                Meld::Triplet {..} | Meld::Kan {..} => trips += 1,
                _ => () } }
        trips
    }
    fn count_kans(&self) -> i8 {
        let mut kans: i8 = 0;
        for meld in self.melds {
            if let Meld::Kan{..} = meld { kans += 1 } }
        kans
    }
    fn count_dragons(&self) -> i8 {
        let mut dragons: i8 = 0;
        for meld in &self.melds {
            if meld.is_dragon() { dragons += 1 } }
        dragons
    }
    fn count_winds(&self) -> i8 {
        let mut winds: i8 = 0;
        for meld in &self.melds {
            if meld.is_wind() { winds += 1 } }
        winds
    }
    fn has_any_honor(&self) -> bool {
        for meld in &self.melds {
            if meld.has_honor() { return true } }
        false
    }
    fn has_any_terminal(&self) -> bool {
        for meld in &self.melds {
            if meld.has_terminal() { return true } }
        false
    }
    fn has_any_simple(&self) -> bool {
        for meld in &self.melds {
            if !meld.has_honor() && !meld.has_terminal() { return true } }
        false
    }
    fn only_sequences(&self) -> Vec<Meld> {
        let mut vec: Vec<Meld> = vec![];
        for meld in &self.melds {
            if let &Meld::Sequence {..} = meld { vec.push(*meld) } }
        vec
    }
    fn without_sequences(&self) -> Vec<Meld> {
        let mut vec: Vec<Meld> = vec![];
        for meld in &self.melds {
            if let &Meld::Sequence {..} = meld { } else { vec.push(*meld) } }
        vec
    }

    fn as_tiles(&self) -> Vec<Tile> {
        let mut vec: Vec<Tile> = vec![];
        vec.push(self.pair.tile); vec.push(self.pair.tile);
        for meld in self.melds {
            match meld {
                Meld::Sequence {tiles, ..} => {
                    vec.push(tiles[0]); vec.push(tiles[1]); vec.push(tiles[2]); },
                Meld::Triplet {tile, ..} => {
                    for _ in 0..3 { vec.push(tile); } },
                Meld::Kan {tile, ..} => {
                    for _ in 0..4 { vec.push(tile); } },
        } }
        vec
    }

    fn only_closed(&self) -> Vec<MeldOrPair> {
        let mut vec: Vec<MeldOrPair> = vec![];
        vec.push(MeldOrPair::Pair(self.pair));
        for meld in self.melds {
            match meld {
                Meld::Triplet {open, ..} | Meld::Kan {open, ..} | Meld::Sequence {open, ..}
                    => if !open { vec.push(MeldOrPair::Meld(meld))},
        } }
        vec
    }
}

impl HandHelpers for [Pair] {
    fn count_suits(&self) -> i8 {
        let mut suits: Vec<Suit> = vec![];
        for pair in self { 
            if let Tile::Number {suit,..} = pair.tile { 
                if !suits.contains(&suit) { suits.push(suit) } } }
        suits.len() as i8
    }
    fn count_dragons(&self) -> i8 {
        let mut count: i8 = 0;
        for pair in self { if pair.tile.is_dragon() { count += 1 } }
        count
    }
    fn count_winds(&self) -> i8 {
        let mut count: i8 = 0;
        for pair in self { if pair.tile.is_wind() { count += 1 } }
        count
    }
    fn has_any_honor(&self) -> bool {
        for pair in self { if pair.tile.is_honor() { return true } }
        return false 
    }
    fn has_any_terminal(&self) -> bool {
        for pair in self { if pair.tile.is_terminal() { return true } }
        return false 
    }
    fn has_any_simple(&self) -> bool {
        for pair in self {
            if !pair.tile.is_honor() && !pair.tile.is_terminal() { return true } }
        return false 
    }
    fn as_tiles(&self) -> Vec<Tile> {
        let mut vec: Vec<Tile> = vec![];
        for pair in self { vec.push(pair.tile); vec.push(pair.tile); }
        vec
    }
}

pub trait MeldHelpers {
    fn has_honor(&self) -> bool;
    fn has_terminal(&self) -> bool;
    fn is_dragon(&self) -> bool;
    fn is_wind(&self) -> bool;
    fn contains_tile(&self, tile: Tile) -> bool;
    fn get_suit(&self) -> Result<Suit, ScoringError>;
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
    fn get_suit(&self) -> Result<Suit, ScoringError> {
        match self {
            Meld::Kan {tile, ..} | Meld::Triplet {tile, ..} => {
                if let Tile::Number {suit, ..} = *tile { return Ok(suit) }
            }
            Meld::Sequence {tiles, ..} => {
                if let Tile::Number {suit, ..} = tiles[0] { return Ok(suit) }
            }
        }
        Err(ScoringError::TileError)
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
    fn get_suit(&self) -> Result<Suit, ScoringError> {
        if let Tile::Number {suit, ..} = self.tile { return Ok(suit) }
        else { Err(ScoringError::TileError) }
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

// TODO
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hand::Standard {full_hand, winning_tile, yaku, ..} => {
                write!(f, "standard",)
            },
            Hand::Chiitoi {full_hand, winning_tile, yaku, ..} => {
                write!(f, "chiitoi",)
            },
            Hand::Kokushi {full_hand, winning_tile, yaku, ..} => {
                write!(f, "kokushi",)
            }
        }
    }
}

///////////
// tests //
///////////

mod tests {
    use super::*;
    use crate::tiles::{Tile, Dragon, Wind, Suit, TileHelpers, make_tiles_from_string};

    #[test]
    fn test_reading_hand_composition(){
        assert_eq!(compose_tiles(make_tiles_from_string("dw,dw,dw,we,we,we").unwrap(), 0, 0, 2).unwrap(),
            vec![PartialHand::Valid(vec![ MeldOrPair::Meld(Meld::Triplet{tile: Tile::Dragon(Dragon::White), open: false }), 
                                          MeldOrPair::Meld(Meld::Triplet{tile: Tile::Wind(Wind::East), open: false })])]
            );
        assert_eq!(compose_tiles(make_tiles_from_string("dw,dw,dw,we,we").unwrap(), 0, 1, 2).unwrap(),
            vec![PartialHand::Valid(vec![ MeldOrPair::Meld(Meld::Triplet{tile: Tile::Dragon(Dragon::White), open: false }), 
                                          MeldOrPair::Pair(Pair{tile: Tile::Wind(Wind::East) }) ] )]
            );
        assert_eq!(compose_tiles(make_tiles_from_string("dw,dw,we,we,we").unwrap(), 0, 1, 2).unwrap(),
            vec![PartialHand::Valid(vec![ MeldOrPair::Pair(Pair{tile: Tile::Dragon(Dragon::White) }),
                                          MeldOrPair::Meld(Meld::Triplet{tile: Tile::Wind(Wind::East), open: false }), ] )]
            );
        assert_eq!(compose_tiles(make_tiles_from_string("m1,m2,m3,p4,p5r,p3").unwrap(), 0, 0, 2).unwrap(),
            vec![PartialHand::Valid(vec![ MeldOrPair::Meld(Meld::Sequence{tiles: [Tile::Number{ suit: Suit::Man, number: 1, red: false },
                                                                                  Tile::Number{ suit: Suit::Man, number: 2, red: false },
                                                                                  Tile::Number{ suit: Suit::Man, number: 3, red: false } ], open: false}),
                                          MeldOrPair::Meld(Meld::Sequence{tiles: [Tile::Number{ suit: Suit::Pin, number: 3, red: false },
                                                                                  Tile::Number{ suit: Suit::Pin, number: 4, red: false },
                                                                                  Tile::Number{ suit: Suit::Pin, number: 5, red: true } ], open: false}) ])]
            );
        assert_eq!(compose_tiles(make_tiles_from_string("dw,dr,p4,dw,p5r,p3,dr,dw,m2,m2,m2,m2").unwrap(), 1, 1, 4).unwrap(), 
            vec![PartialHand::Valid(vec![ MeldOrPair::Pair(Pair{tile: Tile::Dragon(Dragon::Red) }),
                                          MeldOrPair::Meld(Meld::Triplet{tile: Tile::Dragon(Dragon::White), open: false }),
                                          MeldOrPair::Meld(Meld::Kan{tile: Tile::Number{ suit: Suit::Man, number: 2, red: false}, open: false }),
                                          MeldOrPair::Meld(Meld::Sequence{tiles: [Tile::Number{ suit: Suit::Pin, number: 3, red: false },
                                                                                  Tile::Number{ suit: Suit::Pin, number: 4, red: false },
                                                                                  Tile::Number{ suit: Suit::Pin, number: 5, red: true } ], open: false}) ] )] 
            );
        assert_eq!(compose_tiles(make_tiles_from_string("m1,m1,m1,m2,m2,m2,m3,m3,m3").unwrap(), 0, 0, 3).unwrap(),
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
    fn test_reading_kokushi(){
        let mut hand: Hand = compose_hand(make_tiles_from_string("m1,m1,m9,p1,p9,s1,s9,dw,dr,dg,we,ws,wn,ww").unwrap(),
                                    None, Tile::Number{ suit: Suit::Man, number: 1, red: false }, WinType::Tsumo, Wind::East, Wind::South).unwrap();

        assert!(matches!(hand, Hand::Kokushi {..}));
        assert_eq!(hand, Hand::Kokushi{full_hand: make_tiles_from_string("m1,m1,m9,p1,p9,s1,s9,dw,dr,dg,we,ws,wn,ww").unwrap().try_into().unwrap(),
                                        winning_tile: Tile::Number{ suit: Suit::Man, number: 1, red: false },
                                        yaku: vec![Yaku::Kokushi, Yaku::SpecialWait]});

        let hand = compose_hand(crate::tiles::make_tiles_from_string("m1,m1,m9,p1,p9,s1,s9,dw,dr,dg,we,ws,wn,ww").unwrap(),
                                    None, Tile::Number{ suit: Suit::Man, number: 9, red: false }, WinType::Tsumo, Wind::East, Wind::South).unwrap();
        assert_eq!(hand, Hand::Kokushi{full_hand: make_tiles_from_string("m1,m1,m9,p1,p9,s1,s9,dw,dr,dg,we,ws,wn,ww").unwrap().try_into().unwrap(),
                                        winning_tile: Tile::Number{ suit: Suit::Man, number: 9, red: false },
                                        yaku: vec![Yaku::Kokushi]});
    
    }

    #[test]
    fn test_reading_chiitoi(){
        assert!(matches!(compose_hand(make_tiles_from_string("m1,m1,m2,m2,m4,m4,dw,dw,p6,p6,we,we,s5,s5").unwrap().try_into().unwrap(),
                                    None, Tile::Number{ suit: Suit::Man, number: 1, red: false }, WinType::Ron, Wind::East, Wind::South).unwrap(), Hand::Chiitoi {..}));
        
        let mut chiitoi_yaku = compose_hand(make_tiles_from_string("m2,m2,m3,m3,m4,m4,s2,s2,s5,s5,p3,p3,p6,p6").unwrap().try_into().unwrap(),
                                    None, Tile::Number{ suit: Suit::Man, number: 2, red: false }, WinType::Ron, Wind::East, Wind::South).unwrap();
        assert_eq!(chiitoi_yaku.get_yaku(), vec![Yaku::Chiitoi, Yaku::Tanyao]);

        chiitoi_yaku = compose_hand(make_tiles_from_string("m1,m1,m9,m9,p1,p1,we,we,ww,ww,dw,dw,dr,dr").unwrap().try_into().unwrap(),
                                    None, Tile::Number{ suit: Suit::Man, number: 1, red: false }, WinType::Ron, Wind::East, Wind::South).unwrap();
        assert_eq!(chiitoi_yaku.get_yaku(), vec![Yaku::Chiitoi, Yaku::Honro]);

        chiitoi_yaku = compose_hand(make_tiles_from_string("dw,dw,dr,dr,dg,dg,we,we,ww,ww,ws,ws,wn,wn").unwrap().try_into().unwrap(),
                                    None, Tile::Number{ suit: Suit::Man, number: 2, red: false }, WinType::Ron, Wind::East, Wind::South).unwrap();
        assert_eq!(chiitoi_yaku.get_yaku(), vec![Yaku::Daichiishin])
    }

    #[test]
    #[ignore]
    fn test_dora_count(){
        //TODO: add test cases once I don't need to manually generate hands
    }
}