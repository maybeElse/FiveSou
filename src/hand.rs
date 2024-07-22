use crate::tiles::{Tile, Dragon, Wind, Suit, TileHelpers, make_tiles_from_string};
use crate::errors::errors::{HandError, ParsingError, CompositionError};
use crate::yaku::{Yaku, WinType, YakuHelpers};
use crate::yaku;
use crate::scoring;
use crate::rulesets::{RiichiRuleset, RuleVariations};
use core::fmt;
use core::iter::repeat;

///////////////////////
// structs and enums //
///////////////////////

#[derive(Debug, PartialEq, Clone)]
pub enum Hand {
    Standard{
        full_hand: FullHand,
        winning_tile: Tile,
        open: bool,
        yaku: Vec<Yaku>,
        dora: i8,
        han: i8,
        fu: i8
    },
    Chiitoi{
        full_hand: [Pair; 7],
        winning_tile: Tile,
        yaku: Vec<Yaku>,
        dora: i8,
        han: i8,
        fu: i8
    },
    Kokushi{
        full_hand: [Tile; 14],
        winning_tile: Tile,
        yaku: Vec<Yaku>,
        han: i8,
        fu: i8
    },
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FullHand {
    pub melds: [Meld; 4],
    pub pair: Pair
}

#[derive(Debug, Eq, PartialOrd, Ord, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Pair {
    pub tile: Tile
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartialHand {
    pub hanging_tiles: Vec<Tile>,
    pub melds: Vec<Meld>,
    pub pairs: Vec<Pair>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wait {
    pub tiles: Vec<Tile>,
    pub discard: Option<Tile>
}

// wrapper enum for recursion in compose_tiles()
#[derive(Debug, PartialEq, Clone)]
pub enum MeldOrPair {
    Meld(Meld),
    Pair(Pair)
}

///////////////
// functions //
///////////////

// comma and pipe separated, with closed kans additionally enclosed in '()'
// ie "dw,dw,dw|m1,m2,m3|(p5,p5,p5,p5r)"
// ... complicated and fragile, yeah, but I need something like this to simplify testing
pub fn make_melds_from_string(
    str: &str, open: bool
) -> Option<Vec<Meld>> {
    if str.is_empty() { return None }
    let mut melds: Vec<Meld> = vec![];
    let input: Vec<&str> = str.split('|').collect();
    for s in input {
        if s.chars().nth(0) == Some('!') {
            if let Ok(tiles) = make_tiles_from_string(&s[1..]) {
                if let Some(meld) = make_single_meld(tiles, false) {
                    melds.push(meld);
                } else { panic!("couldn't make a kan from tiles!")}
            } else { panic!("failed to read a kan!")}
        } else {
            if let Ok(tiles) = make_tiles_from_string(&s) {
                if let Some(meld) = make_single_meld(tiles, open) {
                    melds.push(meld);
                } else { panic!("couldn't make a meld from tiles!")}
            } else { panic!("failed to read tiles!")}
    } }
    if !melds.is_empty() { Some(melds) } else { None }
}

pub fn make_single_meld(mut tiles: Vec<Tile>, open: bool) -> Option<Meld> {
    if tiles.len() == 3 {
        if tiles.count_occurrences(&tiles[0]) == 3 {
            return Some(Meld::Triplet{tile: tiles[0], open})
        } else {
            tiles.sort();
            if let Some(adj) = tiles[0].adjacent_up() {
                if adj.contains(&tiles[1]) && adj.contains(&tiles[2]) && tiles[1] != tiles[2] {
                    return Some(Meld::Sequence{
                        tiles: [tiles[0], tiles[1], tiles[2]],
                        open: open
                    })
                }
            }
        }
    } else if tiles.len() == 4 && tiles.count_occurrences(&tiles[0]) == 4 {
        return Some(Meld::Kan{tile: tiles[0], open})
    } else { panic!("bad meld length!") }
    return None
}

// Given tiles and information about the game state, attempts to find the highest value hand possible.
// Only considers hands with valid yaku.
// Errors if no completed hands are found.
pub fn compose_hand(
    closed_tiles: Vec<Tile>, called_tiles: Option<Vec<Meld>>,
    // stuff we need for yaku testing
    winning_tile: Tile, win_type: WinType, seat_wind: Wind, round_wind: Wind,
    special_yaku: Option<Vec<Yaku>>, dora_markers: Option<Vec<Tile>>, ruleset: RiichiRuleset
) -> Result<Hand, HandError> {
    // if there are multiple ways to read the hand, we'll use this to decide which to return
    let mut possible_hands: Vec<Hand> = vec![];

    let dora: i8 = {
        if let Some(d) = dora_markers {
            let dora: Vec<Tile> = d.iter().map(|x| x.dora()).collect();
            closed_tiles.iter().fold(0, |acc, x| { if dora.contains(x) { acc + 1 } else { acc } }) + {
                if let Some(ref melds) = called_tiles {
                    melds.iter().fold(0, |acc, x| { 
                        match x {
                            Meld::Sequence {tiles, ..} => if tiles.iter().any(|&t| dora.contains(&t)) { return acc + 1},
                            Meld::Triplet {tile, ..} => if dora.contains(tile) { return acc + 3 },
                            Meld::Kan {tile, ..} =>  if dora.contains(tile) { return acc + 4 },
                        } acc } )
                } else { 0 } }
        } else { 0 } };
    
    // first of all, test for strange hand shapes
    if closed_tiles.len() == 14 && called_tiles.is_none() {
        // thirteen orphans
        let kokushi: Option<Vec<Yaku>> = compose_kokushi(closed_tiles.clone(), winning_tile);
        if let Some(hand) = kokushi {
            // a thirteen orphan hand can't be anything else (except special yakuman),
            // so we'll just return it after seeing if the Vec<Yaku> accepts any of the special yaku.
            let mut kokushi = hand;
            kokushi.append_checked(&special_yaku.clone().unwrap_or_default());
            return Ok(Hand::Kokushi{
                full_hand: closed_tiles.try_into().unwrap(),
                winning_tile: winning_tile,
                yaku: kokushi,
                han: 13,
                fu: 20
            })
        }
    }

    // now we'll test for normal hand shapes
    // the closed portion of the hand *must* contain the pair, and any melds which haven't been called. simple!

    let called_melds = called_tiles.unwrap_or_default();

    if let Some(partials) = compose_tiles(&closed_tiles, false) {
        for partial in partials {
            if partial.count_remaining_tiles() == 0 && partial.count_pairs() == 1 && (partial.count_melds() + called_melds.len()) == 4 {
                let mut melds: Vec<Meld> = {
                    if let Some(vec) = partial.get_melds() { vec
                    } else { vec![] } };
                let pair: Pair = partial.get_pairs().expect("no pair found")[0];
                for meld in &called_melds { melds.push(*meld) }
                melds.sort();
                let hand: FullHand = FullHand{melds: melds.try_into().unwrap(), pair: pair};
                let is_open: bool = called_melds.iter().any( |&x| !x.is_closed() );
                if let Ok(yaku) = yaku::find_yaku_standard(hand, winning_tile, &special_yaku, is_open, seat_wind, round_wind, win_type, ruleset) {
                    possible_hands.push(
                        Hand::Standard {
                            full_hand: hand,
                            winning_tile: winning_tile,
                            open: is_open,
                            yaku: yaku.clone(),
                            dora: dora,
                            han: scoring::count_han(&yaku, dora, !is_open, ruleset),
                            fu: scoring::count_fu(&hand, &winning_tile, is_open, &yaku, win_type, round_wind, seat_wind, ruleset)?
                        }
                    )
                }
            } else if partial.count_remaining_tiles() == 0 && partial.count_pairs() == 7 && (partial.count_melds() + called_melds.len()) == 0 {
                let yaku: Vec<Yaku> = yaku::find_yaku_chiitoi(partial.pairs.clone().try_into().unwrap(), winning_tile, &special_yaku, win_type)?;
                possible_hands.push(
                    Hand::Chiitoi{
                        full_hand: partial.pairs.try_into().unwrap(),
                        winning_tile: winning_tile, 
                        han: scoring::count_han(&yaku, dora, true, ruleset),
                        yaku: yaku,
                        dora: dora,
                        fu: 25
                    }
                );
            }
        }
    }

    match possible_hands.len() {
        0 => return Err(HandError::NoHands),
        1 => return Ok(possible_hands[0].clone()),
        _ => {
            let max_han: Option<Hand> = possible_hands.iter().max_by_key(|&p|
                scoring::calc_base_points(p.get_han(), p.get_fu(), &p.get_yaku(), ruleset).unwrap() ).cloned();
            if let Some(han) = max_han { return Ok(han)
            } else { return Err(HandError::NoHands) }
        },
    }
}

// Given a list of tiles, attempts to compose those tiles into melds and pairs.
// Returns a list of all possible compositions, including ones in which some tiles could not be assigned to a pair.
// When reading complete hands, check each PartialHand's count_melds() and count_pairs() to ensure that it is valid.
fn compose_tiles(remaining_tiles: &Vec<Tile>, open: bool) -> Option<Vec<PartialHand>> {
    if remaining_tiles.len() <= 1 { return None
    } else {
        let mut partials: Vec<PartialHand> = vec![];
        let subset: Vec<Tile> = remaining_tiles[1..].to_vec();
        let first_tile: Tile = remaining_tiles[0];
        // TODO: currently, we stop looking for melds if remaining_tiles[0] is not part of a valid meld.
        // probably change this behavior to make wait reading work properly.

        if subset.contains(&first_tile){
            // for readability of the if/elseif, we'll count dupes based on remaining_tiles instead of subset
            let dupe_count = remaining_tiles.count_occurrences(&first_tile);

            if dupe_count >= 2 {
                let temp: Pair = Pair{tile: first_tile};
                let mut subs: Vec<Tile> = subset.clone();

                subs.remove_x_occurrences(first_tile, 1);

                if let Some(recursions) = compose_tiles(&subs, open) {
                    for value in recursions {
                        partials.push( value.with_pair(temp) ) }
                } else {
                    partials.push( PartialHand{
                            hanging_tiles: subs.clone(),
                            melds: vec![],
                            pairs: vec![temp] } ) }
        } }

        for seq in first_tile.adjacent_all() {
            if (subset.contains(&seq[0]) && subset.contains(&seq[1]) && seq[0] != seq[1])
                || (seq[0] == seq[1] && subset.contains(&first_tile) && subset.count_occurrences(&first_tile) >= 2) {
                let mut subs: Vec<Tile> = subset.clone();
                let temp: Meld = {
                    if seq[0] == seq[1] { Meld::Triplet{tile: first_tile, open: open}
                    } else { Meld::Sequence{tiles: [first_tile, seq[0], seq[1]], open: open} } };

                subs.remove(subs.iter().position(|x| *x == seq[0])?);
                subs.remove(subs.iter().position(|x| *x == seq[1])?);

                if let Some(recursions) = compose_tiles(&subs, open) {
                    for value in recursions {
                        partials.push( value.with_meld(temp) ) }
                } else {
                    partials.push( PartialHand{
                            hanging_tiles: subs.clone(),
                            melds: vec![temp],
                            pairs: vec![] } ) }
        } }

        if partials.is_empty() { return None } else {
            return Some(partials)
        }
    }
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
    fn get_dora(&self) -> i8;
    fn get_suits(&self) -> Vec<Suit>;   // and again
    fn is_closed(&self) -> bool;
    fn get_fu(&self) -> i8;
    fn get_han(&self) -> i8;
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
    } } }
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
    } } }
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
    } } }
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
    } }
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
    fn get_dora(&self) -> i8 {
        match self {
            Hand::Standard {dora, ..} | Hand::Chiitoi {dora, ..} => *dora,
            Hand::Kokushi {..} => 0, // dora never matters for kokushi
    } }
    fn get_suits(&self) -> Vec<Suit> {
        match self {
            Hand::Standard {full_hand, ..} => full_hand.get_suits(),
            Hand::Chiitoi {full_hand, ..} => full_hand.get_suits(), // needs to be a separate case because full_hand's type is different
            Hand::Kokushi {..} => vec![Suit::Man, Suit::Pin, Suit::Sou]
    } }
    fn is_closed(&self) -> bool {
        // TODO: find a better way to grab the field
        match self { 
            Hand::Standard {open, ..} => !*open,
            _ => true,
    } }
    fn get_fu(&self) -> i8 {
        match self { Hand::Standard {fu, ..} | Hand::Chiitoi {fu, ..} | Hand::Kokushi {fu, ..} => *fu } }
    fn get_han(&self) -> i8 {
        match self { Hand::Standard {han, ..} | Hand::Chiitoi {han, ..} | Hand::Kokushi {han, ..} => *han } }
}

pub trait HandHelpers {
    // I'm using this trait for both FullHand and [Pair];
    // functions which don't apply to [Pair] have a default implementation that just panics.
    // doing it this way feels a bit tidier.
    fn count_suits(&self) -> i8;            // count how many suits are present
    fn count_sequence_suits(&self) -> i8 {panic!()}// ... or just the ones in sequences
    fn count_triplet_suits(&self) -> i8 {panic!()} // ... or just the ones in triplets
    fn get_suits(&self) -> Vec<Suit>;              // ... or get a vec of all the suits that are here.
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
    fn only_open(&self) -> Vec<Meld> {panic!()}         // ... or only the open part
    fn is_pure_green(&self, ruleset: RiichiRuleset) -> bool {false} // check for ryuisou eligibility
} 

impl HandHelpers for FullHand {
    fn count_suits(&self) -> i8 {
        self.get_suits().len() as i8
    }
    fn get_suits(&self) -> Vec<Suit> {
        let mut suits: Vec<Suit> = vec![];
        if let Tile::Number {suit, ..} = self.pair.tile { suits.push(suit) }
        for meld in self.melds { 
            if let Some(suit) = meld.get_suit() {
                if !suits.contains(&suit) { suits.push(suit) } } }
        suits
    }
    fn count_sequences(&self) -> i8 {
        self.only_sequences().len() as i8
    }
    fn count_sequence_suits(&self) -> i8 {
        let mut suits: Vec<Suit> = vec![];
        for meld in self.only_sequences() {
            if let Some(suit) = meld.get_suit() {
                if !suits.contains(&suit) { suits.push(suit) } } }
        suits.len() as i8
    }
    fn count_triplet_suits(&self) -> i8 {
        let mut suits: Vec<Suit> = vec![];
        for meld in self.without_sequences() {
            if let Some(suit) = meld.get_suit() {
                if !suits.contains(&suit) { suits.push(suit) } } }
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
        if self.pair.is_dragon() { dragons +=  1 }
        for meld in &self.melds {
            if meld.is_dragon() { dragons += 1 } }
        dragons
    }
    fn count_winds(&self) -> i8 {
        let mut winds: i8 = 0;
        if self.pair.is_wind() { winds += 1 }
        for meld in &self.melds {
            if meld.is_wind() { winds += 1 } }
        winds
    }
    fn has_any_honor(&self) -> bool {
        if self.pair.has_honor() { return true }
        for meld in &self.melds {
            if meld.has_honor() { return true } }
        false
    }
    fn has_any_terminal(&self) -> bool {
        if self.pair.has_terminal() { return true }
        for meld in &self.melds {
            if meld.has_terminal() { return true } }
        false
    }
    fn has_any_simple(&self) -> bool {
        if self.pair.has_simple() { return true }
        for meld in &self.melds {
            if meld.has_simple() { return true } }
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
    fn only_open(&self) -> Vec<Meld> {
        let mut vec: Vec<Meld> = vec![];
        for meld in self.melds {
            match meld {
                Meld::Triplet {open, ..} | Meld::Kan {open, ..} | Meld::Sequence {open, ..}
                    => if open { vec.push(meld)},
        } }
        vec
    }
    fn is_pure_green(&self, ruleset: RiichiRuleset) -> bool {
        if !self.pair.is_pure_green(ruleset) { return false }
        for meld in self.melds {
            if !meld.is_pure_green(ruleset) { return false } }
        if ruleset.requires_all_green_hatsu() { // an uncommon rule.
            self.has_any_honor()                // but it's easy to check for!
        } else { true }
    }
}

impl HandHelpers for [Pair] {
    fn count_suits(&self) -> i8 {
        self.get_suits().len() as i8
    }
    fn get_suits(&self) -> Vec<Suit> {
        let mut suits: Vec<Suit> = vec![];
        for pair in self { 
            if let Tile::Number {suit,..} = pair.tile { 
                if !suits.contains(&suit) { suits.push(suit); } } }
        suits
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

impl PartialEq for Meld {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Meld::Sequence {tiles, ..} => {
                let t1 = tiles;
                if let Meld::Sequence {tiles, ..} = other { t1 == tiles } else { false }
            },
            Meld::Triplet {tile, ..} => {
                let t1 = tile;
                if let Meld::Triplet {tile, ..} = other { t1 == tile } else { false }
            },
            Meld::Kan {tile, ..} => {
                let t1 = tile;
                if let Meld::Kan {tile, ..} = other { t1 == tile } else { false }
            },
        }
    }
}

pub trait MeldHelpers {
    fn has_honor(&self) -> bool;
    fn has_terminal(&self) -> bool;
    fn has_simple(&self) -> bool;
    fn is_dragon(&self) -> bool;
    fn is_wind(&self) -> bool;
    fn is_pure_green(&self, ruleset: RiichiRuleset) -> bool;
    fn is_closed(&self) -> bool {true}
    fn contains_tile(&self, tile: &Tile) -> bool;
    fn get_suit(&self) -> Option<Suit>;
    fn get_tile(&self) -> Result<Tile, HandError>;
    fn set_open(&self) {} // marks a meld as open
}

impl MeldHelpers for Meld {
    fn has_honor(&self) -> bool{
        match self {
            Meld::Triplet {tile, ..} | Meld::Kan {tile, ..} => {
                match tile {
                    Tile::Wind(_) | Tile::Dragon(_) => true,
                    _ => false } },
            _ => false,
    } }
    fn has_terminal(&self) -> bool {
        match self {
            Meld::Sequence {tiles, ..} => {
                for tile in tiles { if tile.is_terminal() { return true } }
                false
            },
            Meld::Kan {tile, ..} | Meld::Triplet {tile, ..} => { tile.is_terminal() },
    } }
    fn has_simple(&self) -> bool {
        match self {
            Meld::Sequence {tiles, ..} => { return true },
            Meld::Kan {tile, ..} | Meld::Triplet {tile, ..} => { !tile.is_terminal() && !tile.is_honor() },
    } }
    fn is_dragon(&self) -> bool {
        match self {
            Meld::Kan {tile, ..} | Meld::Triplet {tile, ..} => {
                if let Tile::Dragon(_) = tile { true } else { false } }
            _ => false
    } }
    fn is_wind(&self) -> bool {
        match self {
            Meld::Kan {tile, ..} | Meld::Triplet {tile, ..} => {
                if let Tile::Wind(_) = tile { true } else { false } }
            _ => false
    } }
    fn is_pure_green(&self, ruleset: RiichiRuleset) -> bool {
        match self {
            Meld::Kan {tile, ..} | Meld::Triplet {tile, ..} => tile.is_pure_green(ruleset),
            Meld::Sequence {tiles, ..} => {
                for tile in tiles { if !tile.is_pure_green(ruleset) { return false } }
                true
    } } }
    fn is_closed(&self) -> bool {
        match self { Meld::Kan {open, ..} | Meld::Triplet {open, ..} | Meld::Sequence {open, ..} => !open
    } }
    fn contains_tile(&self, t: &Tile) -> bool {
        match self {
            Meld::Kan {tile, ..} | Meld::Triplet {tile, ..} => t == tile,
            Meld::Sequence {tiles, ..} => {
                for tile in tiles { if t == tile { return true } }
                false
    } } }
    fn get_suit(&self) -> Option<Suit> {
        match self {
            Meld::Kan {tile, ..} | Meld::Triplet {tile, ..} => {
                if let Tile::Number {suit, ..} = *tile { return Some(suit) }
            }
            Meld::Sequence {tiles, ..} => {
                if let Tile::Number {suit, ..} = tiles[0] { return Some(suit) }
        } }
        None
    }
    fn get_tile(&self) -> Result<Tile, HandError> {
        match self {
            Meld::Kan {tile, ..} | Meld::Triplet {tile, ..} => Ok(*tile),
            _ => Err(HandError::WrongMeldType)
    } }
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
    fn has_simple(&self) -> bool {
        !self.tile.is_terminal() && !self.tile.is_dragon()
    }
    fn is_dragon(&self) -> bool {
        if let Tile::Dragon(_) = self.tile { true } else { false }
    }
    fn is_wind(&self) -> bool {
        if let Tile::Wind(_) = self.tile { true } else { false }
    }
    fn is_pure_green(&self, ruleset: RiichiRuleset) -> bool {
        self.tile.is_pure_green(ruleset)
    }
    fn contains_tile(&self, t: &Tile) -> bool {
        &self.tile == t
    }
    fn get_suit(&self) -> Option<Suit> {
        if let Tile::Number {suit, ..} = self.tile { return Some(suit) }
        else { None }
    }
    fn get_tile(&self) -> Result<Tile, HandError> {
        Ok(self.tile)
    }
}

pub trait SequenceHelpers {
    fn as_numbers(&self) -> [u8; 3];
    fn ittsuu_viable(&self) -> bool;
    fn is_middle(&self, tile: &Tile) -> bool;
}

impl SequenceHelpers for Meld {
    fn as_numbers(&self) -> [u8; 3] {
        if let Meld::Sequence {tiles, ..} = self {
            let mut a: [u8; 3] = [tiles[0].get_number().unwrap(), tiles[1].get_number().unwrap(), tiles[2].get_number().unwrap()];
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
    fn is_middle(&self, tile: &Tile) -> bool {
        if let Meld::Sequence {tiles, ..} = self {
            let mut a: [Tile; 3] = tiles.clone();
            a.sort();
            &a[1] == tile
        } else { panic!("called is_middle() on a non-sequence meld!") }
    }
}

pub trait VecTileHelpers {
    fn count_occurrences(&self, tile: &Tile) -> i8;
    fn remove_x_occurrences(&mut self, tile: Tile, count: i8) -> ();
}

impl VecTileHelpers for Vec<Tile> {
    fn count_occurrences(&self, tile: &Tile) -> i8 {
        self.iter().fold(0, |acc, value| {if tile == value { acc + 1 } else { acc }})
    }
    fn remove_x_occurrences(&mut self, tile: Tile, count: i8) -> () {
        for i in 0..count {
            self.remove(self.iter().position(|x| *x == tile).unwrap());
        }
    }
}

pub trait VecMeldHelpers {
    fn count_suits(&self) -> usize;
}

impl VecMeldHelpers for Vec<Meld> {
    fn count_suits(&self) -> usize {
        let mut suits: Vec<Suit> = vec![];
        for meld in self { if !suits.contains(&meld.get_suit().unwrap()) { suits.push(meld.get_suit().unwrap()) } }
        suits.len()
    }
}

impl VecMeldHelpers for [Meld] {
    fn count_suits(&self) -> usize {
        let mut suits: Vec<Suit> = vec![];
        for meld in self { if !suits.contains(&meld.get_suit().unwrap()) { suits.push(meld.get_suit().unwrap()) } }
        suits.len()
    }
}

impl fmt::Display for Meld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Meld::Kan {tile, open} => {
                if *open { write!(f, "{}", format!("{},", tile).repeat(4))
                } else { write!(f, "({})", format!("{},", tile).repeat(4)) }
            },
            Meld::Triplet {tile, ..} =>  write!(f, "{}", format!("{},", tile).repeat(3)),
            Meld::Sequence {tiles, ..} =>  write!(f, "{},{},{},", tiles[0], tiles[1], tiles[2]),
        }
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},", self.tile, self.tile)
} }

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

pub trait PartialHelpers {
    fn get_waits(&self) -> Option<Vec<Tile>>;
    fn get_melds(&self) -> Option<Vec<Meld>>;
    fn count_melds(&self) -> usize;
    fn get_pairs(&self) -> Option<Vec<Pair>>;
    fn count_pairs(&self) -> usize;
    fn get_remaining_tiles(&self) -> Option<Vec<Tile>>;
    fn count_remaining_tiles(&self) -> usize;
    fn is_complete(&self) -> bool;
    fn is_tenpai(&self) -> bool;
    fn with_pair(&self, pair: Pair) -> Self where Self: Sized;
    fn with_meld(&self, meld: Meld) -> Self where Self: Sized;
}

impl PartialHelpers for PartialHand {
    fn get_waits(&self) -> Option<Vec<Tile>> { panic!() }
    fn get_melds(&self) -> Option<Vec<Meld>> {
        if self.melds.is_empty() { None } else { Some(self.melds.clone()) } }
    fn count_melds(&self) -> usize { self.melds.len() }
    fn get_pairs(&self) -> Option<Vec<Pair>> {
        if self.pairs.is_empty() { None } else { Some(self.pairs.clone()) } }
    fn count_pairs(&self) -> usize { self.pairs.len() }
    fn get_remaining_tiles(&self) -> Option<Vec<Tile>> {
        if self.hanging_tiles.is_empty() { None } else { Some(self.hanging_tiles.clone()) } }
    fn count_remaining_tiles(&self) -> usize { self.hanging_tiles.len() }
    fn is_complete(&self) -> bool {
        if self.hanging_tiles.len() == 0 { 
            if (self.melds.len() == 4 && self.pairs.len() == 1) || (self.melds.len() == 0 && self.pairs.len() == 7) { return true }
        } false }
    fn is_tenpai(&self) -> bool {
        false
    }
    fn with_pair(&self, pair: Pair) -> PartialHand {
        let mut pairs = [self.pairs.clone(), vec![pair]].concat();
        pairs.sort();
        PartialHand{
            hanging_tiles: self.hanging_tiles.clone(),
            melds: self.melds.clone(),
            pairs: pairs
    } }
    fn with_meld(&self, meld: Meld) -> PartialHand {
        let mut melds = [self.melds.clone(), vec![meld]].concat();
        melds.sort();
        PartialHand{
            hanging_tiles: self.hanging_tiles.clone(),
            pairs: self.pairs.clone(),
            melds: melds
    } }
}

///////////
// tests //
///////////

mod tests {
    use super::*;
    use crate::tiles::{Tile, Dragon, Wind, Suit, TileHelpers, make_tiles_from_string, MakeTile};

    #[test]
    fn test_melds_from_string(){
        assert_eq!(make_melds_from_string("we,we,we,we", false), Some(vec![Meld::Kan{open: false, tile: Tile::Wind(Wind::East)}]));
        assert_eq!(make_melds_from_string("!we,we,we,we", false), Some(vec![Meld::Kan{open: false, tile: Tile::Wind(Wind::East)}]));
        assert_eq!(make_melds_from_string("we,we,we", true), Some(vec![Meld::Triplet{open: true, tile: Tile::Wind(Wind::East)}]));
        assert_eq!(make_melds_from_string("we,we,we|dr,dr,dr", true), Some(vec![Meld::Triplet{open: true, tile: Tile::Wind(Wind::East)},Meld::Triplet{open: true, tile: Tile::Dragon(Dragon::Red)}]));
        assert_eq!(make_melds_from_string("p5,p4,p6", true), Some(vec![Meld::Sequence{open: true, tiles: [Tile::from_string("p4").unwrap(), Tile::from_string("p5").unwrap(), Tile::from_string("p6").unwrap()]}]));
    }

    #[test]
    fn test_reading_hand_composition(){
        let mut tiles = compose_tiles(&make_tiles_from_string("we,we").unwrap(), true).unwrap();
        tiles.retain(|x| x.count_remaining_tiles() == 0);
        assert_eq!(tiles, vec![PartialHand {
                pairs: vec![ Pair{tile: Tile::Wind(Wind::East)} ],
                melds: vec![],
                hanging_tiles: vec![] }]);
        let mut tiles = compose_tiles(&make_tiles_from_string("dw,dw,dw,we,we,we").unwrap(), false).unwrap();
        tiles.retain(|x| x.count_remaining_tiles() == 0);
        assert_eq!(tiles, vec![PartialHand {
                pairs: vec![],
                melds: vec![Meld::Triplet{tile: Tile::Dragon(Dragon::White), open: false },
                            Meld::Triplet{tile: Tile::Wind(Wind::East), open: false }, ],
                hanging_tiles: vec![] }]);
        let mut tiles = compose_tiles(&make_tiles_from_string("dw,dw,dw,we,we").unwrap(), true).unwrap();
        tiles.retain(|x| x.count_remaining_tiles() == 0);
        assert_eq!(tiles, vec![PartialHand {
                pairs: vec![Pair{tile: Tile::Wind(Wind::East) }],
                melds: vec![Meld::Triplet{tile: Tile::Dragon(Dragon::White), open: false }],
                hanging_tiles: vec![] }]);
        let mut tiles = compose_tiles(&make_tiles_from_string("dw,dw,we,we,we").unwrap(), true).unwrap();
        tiles.retain(|x| x.count_remaining_tiles() == 0);
        assert_eq!(tiles, vec![PartialHand {
                pairs: vec![Pair{tile: Tile::Dragon(Dragon::White)}],
                melds: vec![Meld::Triplet{tile: Tile::Wind(Wind::East), open: false }],
                hanging_tiles: vec![] }]);
        let mut tiles = compose_tiles(&make_tiles_from_string("m1,m2,m3,p4,p5r,p3").unwrap(), false).unwrap();
        tiles.retain(|x| x.count_remaining_tiles() == 0);
        assert_eq!(tiles, (vec![PartialHand {
                pairs: vec![],
                melds: vec![Meld::Sequence{tiles: [Tile::from_string("m1").unwrap(), Tile::from_string("m2").unwrap(), Tile::from_string("m3").unwrap()], open: false},
                            Meld::Sequence{tiles: [Tile::from_string("p3").unwrap(), Tile::from_string("p4").unwrap(), Tile::from_string("p5r").unwrap()], open: false},],
                hanging_tiles: vec![] }]) );
        let mut tiles = compose_tiles(&make_tiles_from_string("dw,dr,p4,dw,p5r,p3,dr,dw,m2,m2,m2").unwrap(), true).unwrap();
        tiles.retain(|x| x.count_remaining_tiles() == 0);
        assert_eq!(tiles, vec![PartialHand {
                pairs: vec![Pair{tile: Tile::Dragon(Dragon::Red)}],
                melds: vec![Meld::Triplet{tile: Tile::Number{ suit: Suit::Man, number: 2, red: false}, open: false },
                            Meld::Triplet{tile: Tile::Dragon(Dragon::White), open: false },
                            Meld::Sequence{tiles: [Tile::from_string("p3").unwrap(), Tile::from_string("p4").unwrap(), Tile::from_string("p5r").unwrap()], open: false},],
                hanging_tiles: vec![] }]);
        let mut tiles = compose_tiles(&make_tiles_from_string("m1,m1,m1,m2,m2,m2,m3,m3,m3").unwrap(), false).unwrap();
        tiles.retain(|x| x.count_remaining_tiles() == 0 && x.count_pairs() == 0);
        assert_eq!(tiles, vec![PartialHand {
                    pairs: vec![],
                    melds: vec![Meld::Triplet{tile: Tile::Number{ suit: Suit::Man, number: 1, red: false }, open: false },
                                Meld::Triplet{tile: Tile::Number{ suit: Suit::Man, number: 2, red: false }, open: false },
                                Meld::Triplet{tile: Tile::Number{ suit: Suit::Man, number: 3, red: false }, open: false },],
                    hanging_tiles: vec![] },
                PartialHand {
                    pairs: vec![],
                    melds: vec![Meld::Sequence{tiles: [Tile::from_string("m1").unwrap(), Tile::from_string("m2").unwrap(), Tile::from_string("m3").unwrap()], open: false},
                                Meld::Sequence{tiles: [Tile::from_string("m1").unwrap(), Tile::from_string("m2").unwrap(), Tile::from_string("m3").unwrap()], open: false},
                                Meld::Sequence{tiles: [Tile::from_string("m1").unwrap(), Tile::from_string("m2").unwrap(), Tile::from_string("m3").unwrap()], open: false},],
                    hanging_tiles: vec![] },
                ]);
    }

    #[test]
    fn test_reading_kokushi(){
        let hand: Hand = compose_hand(make_tiles_from_string("m1,m1,m9,p1,p9,s1,s9,dw,dr,dg,we,ws,wn,ww").unwrap(), None,
            Tile::Number{ suit: Suit::Man, number: 1, red: false }, WinType::Tsumo, Wind::East, Wind::South, None, None, RiichiRuleset::Default).unwrap();

        assert!(matches!(hand, Hand::Kokushi {..}));
        assert_eq!(hand, Hand::Kokushi{full_hand: make_tiles_from_string("m1,m1,m9,p1,p9,s1,s9,dw,dr,dg,we,ws,wn,ww").unwrap().try_into().unwrap(),
            winning_tile: Tile::Number{ suit: Suit::Man, number: 1, red: false }, yaku: vec![Yaku::Kokushi, Yaku::SpecialWait], han: 13, fu: 20});

        let hand: Hand = compose_hand(crate::tiles::make_tiles_from_string("m1,m1,m9,p1,p9,s1,s9,dw,dr,dg,we,ws,wn,ww").unwrap(), None,
            Tile::Number{ suit: Suit::Man, number: 9, red: false }, WinType::Tsumo, Wind::East, Wind::South, None, None, RiichiRuleset::Default).unwrap();
        assert_eq!(hand, Hand::Kokushi{full_hand: make_tiles_from_string("m1,m1,m9,p1,p9,s1,s9,dw,dr,dg,we,ws,wn,ww").unwrap().try_into().unwrap(),
            winning_tile: Tile::Number{ suit: Suit::Man, number: 9, red: false }, yaku: vec![Yaku::Kokushi], han: 13, fu: 20});  
    }

    #[test]
    fn test_reading_chiitoi(){
        assert!(matches!(compose_hand(make_tiles_from_string("m1,m1,m2,m2,m4,m4,dw,dw,p6,p6,we,we,s5,s5").unwrap().try_into().unwrap(),
            None, Tile::Number{ suit: Suit::Man, number: 1, red: false }, WinType::Ron, Wind::East, Wind::South, None, None, RiichiRuleset::Default).unwrap(), Hand::Chiitoi {..}));
        
        let chiitoi_yaku: Hand = compose_hand(make_tiles_from_string("m2,m2,m3,m3,m4,m4,s2,s2,s5,s5,p3,p3,p6,p6").unwrap().try_into().unwrap(),
            None, Tile::Number{ suit: Suit::Man, number: 2, red: false }, WinType::Ron, Wind::East, Wind::South, None, None, RiichiRuleset::Default).unwrap();
        assert_eq!(chiitoi_yaku.get_yaku(), vec![Yaku::Chiitoi, Yaku::Tanyao]);

        let chiitoi_yaku: Hand = compose_hand(make_tiles_from_string("m1,m1,m9,m9,p1,p1,we,we,ww,ww,dw,dw,dr,dr").unwrap().try_into().unwrap(),
            None, Tile::Number{ suit: Suit::Man, number: 1, red: false }, WinType::Ron, Wind::East, Wind::South, None, None, RiichiRuleset::Default).unwrap();
        assert_eq!(chiitoi_yaku.get_yaku(), vec![Yaku::Chiitoi, Yaku::Honro]);

        let chiitoi_yaku: Hand = compose_hand(make_tiles_from_string("dw,dw,dr,dr,dg,dg,we,we,ww,ww,ws,ws,wn,wn").unwrap().try_into().unwrap(),
            None, Tile::Number{ suit: Suit::Man, number: 2, red: false }, WinType::Ron, Wind::East, Wind::South, None, None, RiichiRuleset::Default).unwrap();
        assert_eq!(chiitoi_yaku.get_yaku(), vec![Yaku::Daichiishin]);                      
    }

    #[test]
    fn test_dora_count(){
        let hand: Hand = compose_hand(make_tiles_from_string("m1,m2,m3,m1,m2,m3,p9,p9,p9,dr,dr").unwrap(),
            make_melds_from_string("s4,s4,s4,s4", false), Tile::Number{ suit: Suit::Man, number: 1, red: false }, WinType::Ron, Wind::East, Wind::South,
            None, Some(vec![Tile::Number{ suit: Suit::Pin, number: 8, red: false}]), RiichiRuleset::Default).unwrap();
        assert_eq!(hand.get_dora(), 3);
        assert_eq!(hand.get_han(), 4);

        let hand: Hand = compose_hand(make_tiles_from_string("m1,m2,m3,m1,m2,m3,p9,p9,p9,dr,dr").unwrap(),
            make_melds_from_string("s4,s4,s4,s4", false), Tile::Number{ suit: Suit::Man, number: 1, red: false }, WinType::Tsumo, Wind::East, Wind::South,
            None, Some(vec![Tile::Number{ suit: Suit::Sou, number: 3, red: false}]), RiichiRuleset::Default).unwrap();
        assert_eq!(hand.get_dora(), 4);
        assert_eq!(hand.get_han(), 6);

        let hand: Hand = compose_hand(make_tiles_from_string("m1,m2,m3,m1,m2,m3,p9,p9,p9,dr,dr").unwrap(),
            make_melds_from_string("s4,s4,s4,s4", false), Tile::Number{ suit: Suit::Man, number: 1, red: false }, WinType::Ron, Wind::East, Wind::South,
            None, Some(vec![Tile::Number{ suit: Suit::Sou, number: 3, red: false}, Tile::Number{ suit: Suit::Man, number: 2, red: false}]), RiichiRuleset::Default).unwrap();
        assert_eq!(hand.get_dora(), 6);
        assert_eq!(hand.get_han(), 7);
        
        let hand: Hand = compose_hand(make_tiles_from_string("m1,m2,m3,m1,m2,m3,p9,p9,p9,dr,dr").unwrap(),
            make_melds_from_string("s4,s4,s4,s4", false), Tile::Number{ suit: Suit::Man, number: 1, red: false }, WinType::Tsumo, Wind::East, Wind::South,
            None, Some(vec![Tile::Number{ suit: Suit::Sou, number: 3, red: false}, Tile::Dragon(Dragon::Green)]), RiichiRuleset::Default).unwrap();
        assert_eq!(hand.get_dora(), 6);
        assert_eq!(hand.get_han(), 8);
    }

    #[test]
    fn hand_ranking(){
        let hand: Hand = compose_hand(make_tiles_from_string("p6,p7,p8,s1,s1,s1,s2,s2,s2,s3,s3,s3,we,we").unwrap(),
            None, Tile::Number{ suit: Suit::Sou, number: 1, red: false }, WinType::Tsumo, Wind::East, Wind::East,
            None, None, RiichiRuleset::Default).unwrap();
        assert_eq!(hand.get_yaku(), vec![Yaku::ClosedTsumo, Yaku::Sananko]);

        let hand = compose_hand(make_tiles_from_string("p6,p7,p8,s1,s1,s2,s2,s3,s3,we,we,m1,m2,m3").unwrap(),
            None, Tile::Number{ suit: Suit::Sou, number: 1, red: false }, WinType::Ron, Wind::East, Wind::East,
            None, None, RiichiRuleset::Default).unwrap();
        assert_eq!(hand.get_yaku(), vec![Yaku::Ipeiko]);

        let hand = compose_hand(make_tiles_from_string("p6,p7,p8,s1,s1,s1,s2,s2,s2,s3,s3,s3,we,we").unwrap(),
            None, Tile::Number{ suit: Suit::Sou, number: 1, red: false }, WinType::Ron, Wind::East, Wind::East,
            None, None, RiichiRuleset::JPML2023).unwrap();
        assert_eq!(hand.get_yaku(), vec![Yaku::Ipeiko]);
    }
}