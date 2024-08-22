use crate::tiles::{Tile, Dragon, Wind, Suit, TileIs, TileRelations, TileVecTrait};
use crate::state::{GameState, SeatState, Win, WinType, TileType, SeatAccess};
use crate::errors::errors::{HandError, ParsingError};
use crate::yaku::{Yaku, YakuHelpers, FindYaku};
use crate::scoring::{Payment, CountFu, CountHan, calc_base_points};
use crate::rulesets::{RiichiRuleset, RuleVariations};
use crate::conversions::{TileConversions, StringConversions};
use core::fmt;
use core::iter::repeat;

///////////////////////
// structs and enums //
///////////////////////

#[derive(Debug, PartialEq, Clone)]
pub enum Hand {
    Agari {
        hand_tiles: Vec<Tile>,
        hand_shape: HandShape,
        latest_tile: Tile,
        open: bool,
        yaku: Vec<Yaku>,
        dora: u8,
        han: u8,
        fu: u8,
    },
    Tenpai, // TODO
    // {
    //     hand_tiles: Vec<Tile>,
    //     hand_shape: HandShape,
    //     open: bool,
    //     waits: Vec<Wait>
    // },
    Shanten // TODO
}

#[derive(Debug, PartialEq, Clone)]
pub enum HandShape {
	Standard {
		melds: [Meld; 4],
		pair: Pair
	},
	Chiitoi {
		pairs: [Pair; 7]
	},
	Kokushi(Vec<Yaku>),
    Tenpai, // TODO
    Shanten // TODO
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct Meld {
	pub tiles: [Option<Tile>; 4],
	pub is_open: bool
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct Pair {
	pub tiles: [Tile; 2]
}

#[derive(Debug, PartialEq, Clone)]
pub struct Wait {
    // TODO
}

// Used for recursion; see fn compose_tiles()
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PartialHand {
    pub hanging_tiles: Vec<Tile>,
    pub melds: Vec<Meld>,
    pub pairs: Vec<Pair>,
}

////////////
// traits //
////////////

pub trait HandTrait {
	fn new(game_state: GameState, seat_state: SeatState) -> Self where Self: Sized;
}

pub trait HandShapeVecTrait {
    fn find_best(&self) -> Option<(HandShape, Vec<Yaku>, i8, i8)>;
}

pub trait MeldIs {
	fn is_quad(&self) -> bool;
	fn is_trip(&self) -> bool;
	fn is_seq(&self) -> bool;
}

pub trait MeldHas {
    fn has_terminal(&self) -> bool;
    fn has_simple(&self) -> bool;
    fn base_fu(&self) -> u8;
    fn contains(&self, tile: &Tile) -> bool;
    fn as_tiles(&self) -> Vec<Tile>;
}

pub trait PairTrait {
    fn tile(&self) -> Tile;
    fn contains(&self, tile: &Tile) -> bool;
}

pub trait TileVecConversions {
    fn to_meld(&self) -> Option<Meld>;
    fn to_hand(&self) -> Option<HandShape>;
}

pub trait MeldVecHas {
    fn has_any_honor(&self) -> bool;
    fn has_any_simple(&self) -> bool;
    fn has_any_terminal(&self) -> bool;
    fn contains_tile(&self, tile: &Tile) -> bool;
    fn count_suits(&self) -> usize;
}

pub trait PartialHandTrait {
    fn sort(&mut self);
    fn is_complete(&self) -> bool;
    fn is_tenpai(&self) -> bool;
    fn with_pair(&self, pair: Pair) -> Self where Self: Sized;
    fn with_meld(&self, meld: Meld) -> Self where Self: Sized;
}

/////////////////////
// implementations //
/////////////////////

impl HandTrait for Hand {
    // Create a new Hand struct from the current GameState and SeatState.
    // Prefers to return the most complete hand: agari > tenpai > shanten.
    // For deeper investigation of potential hands, call read_shanten() and read_tenpai() directly.
    fn new(game_state: GameState, seat_state: SeatState) -> Self where Self: Sized {
        // first, we'll consider only winning hands:
        if let Some(possible_wins) = read_win(seat_state.closed_tiles.clone(), seat_state.called_melds.clone(), seat_state.latest_tile.clone().unwrap()) {
            match possible_wins.len() {
                0 => panic!("read_win() should not return Some(empty vec)"),
                _ => {
                    if let Some((best_hand, best_yaku)) = possible_wins.iter()
                        .map(|h| (h, h.yaku(&game_state, &seat_state)))
                        .max_by_key(|(h, y)| calc_base_points(
                            y.han(seat_state.called_melds.is_some(), game_state.ruleset),
                            h.fu(&game_state, &seat_state, &y).unwrap_or(0), &y, game_state.ruleset).unwrap_or(0)
                    ) {
                        return Hand::Agari {
                            hand_tiles: seat_state.all_tiles(),
                            hand_shape: best_hand.clone(),
                            latest_tile: seat_state.latest_tile.unwrap(),
                            open: seat_state.called_melds.is_some(),
                            dora: seat_state.all_tiles().count_dora(&game_state.dora_markers),
                            han: best_yaku.han(seat_state.called_melds.is_some(), game_state.ruleset),
                            fu: best_hand.fu(&game_state, &seat_state, &best_yaku).unwrap(),
                            yaku: best_yaku,
                        }
                    }
                }
            }
        }
        panic!("couldn't find a completed hand, reached unimplemented section")
    }
}

impl TileIs for Meld {
    fn is_numbered(&self) -> bool { self.tiles[0].is_some_and(|t| t.is_numbered()) }
    fn is_terminal(&self) -> bool { self.has_terminal() }
    fn is_simple(&self) -> bool { self.has_simple() }
    fn is_honor(&self) -> bool { self.tiles[0].is_some_and(|t| t.is_honor()) }
    fn is_wind(&self) -> bool { self.tiles[0].is_some_and(|t| t.is_wind()) }
    fn is_dragon(&self) -> bool { self.tiles[0].is_some_and(|t| t.is_dragon()) }
    fn is_pure_green(&self, ruleset: &RiichiRuleset) -> bool {self.tiles.iter().all(|t| t.is_none() || t.is_some_and(|t| t.is_pure_green(ruleset))) }
    fn suit(&self) -> Option<Suit> { if let Some(tile) = self.tiles[0] { tile.suit() } else { None } }
    fn number(&self) -> Option<i8> { if !self.is_seq() { self.tiles[0].unwrap().number() } else { None } }
    fn wind(&self) -> Option<Wind> { if let Some(tile) = self.tiles[0] { tile.wind() } else { None } }
    fn dragon(&self) -> Option<Dragon> { if let Some(tile) = self.tiles[0] { tile.dragon() } else { None } }
}

impl TileIs for Pair {
    fn is_numbered(&self) -> bool { self.tiles[0].is_numbered() }
    fn is_terminal(&self) -> bool { self.tiles[0].is_terminal() }
    fn is_simple(&self) -> bool { self.tiles[0].is_simple() }
    fn is_honor(&self) -> bool { self.tiles[0].is_honor() }
    fn is_wind(&self) -> bool { self.tiles[0].is_wind() }
    fn is_dragon(&self) -> bool { self.tiles[0].is_dragon() }
    fn is_pure_green(&self, ruleset: &RiichiRuleset) -> bool { self.tiles[0].is_pure_green(ruleset) }
    fn suit(&self) -> Option<Suit> { self.tiles[0].suit() }
    fn number(&self) -> Option<i8> { self.tiles[0].number() }
    fn wind(&self) -> Option<Wind> { self.tiles[0].wind() }
    fn dragon(&self) -> Option<Dragon> { self.tiles[0].dragon() }
}

impl MeldIs for Meld {
	fn is_quad(&self) -> bool { self.tiles[3].is_some() }
	fn is_trip(&self) -> bool { self.tiles[3].is_none() && self.tiles[0] == self.tiles[2] }
	fn is_seq(&self) -> bool { self.tiles[3].is_none() && self.tiles[0] != self.tiles[2] }

}

impl MeldHas for Meld {
    fn has_terminal(&self) -> bool { self.tiles.iter().any(|t| t.is_some_and(|t| t.is_terminal())) }
    fn has_simple(&self) -> bool { self.tiles.iter().any(|t| t.is_some_and(|t| t.is_simple())) }
    fn base_fu(&self) -> u8 {
        if self.is_seq() { return 1 }
        else {
            return 2 // base value of a non-sequence meld
                * { if self.is_quad() { 4 } else { 1 }}         // quadrupled for quads.
                * { if !self.has_simple() { 2 } else { 1 }} // doubled again for honor/terminal.
                // being truly closed adds another 2x. determining that requires looking at the entire hand's shape
                // and the winning tile, so including it here would be cumbersome
        }
    }
    fn contains(&self, tile: &Tile) -> bool { self.tiles.binary_search(&Some(*tile)).is_ok() }
    fn as_tiles(&self) -> Vec<Tile> {
        self.tiles.iter().filter(|t| t.is_some()).map(|t| t.unwrap()).collect()
    } 
}

impl PairTrait for Pair {
    fn tile(&self) -> Tile { self.tiles[0] }
    fn contains(&self, tile: &Tile) -> bool { self.tile() == *tile }
}

macro_rules! impl_MeldVecHas {
    (for $($t:ty),+) => {
        $(impl MeldVecHas for $t {
            fn has_any_honor(&self) -> bool { self.iter().any(|m| m.is_honor()) }
            fn has_any_simple(&self) -> bool { self.iter().any(|m| m.is_simple()) }
            fn has_any_terminal(&self) -> bool { self.iter().any(|m| m.is_terminal()) }
            fn contains_tile(&self, tile: &Tile) -> bool { self.iter().any(|m| m.contains(tile)) }
            fn count_suits(&self) -> usize {
                let mut suits: Vec<Option<Suit>> = self.iter().map(|x| x.suit()).filter(|x| x.is_some()).collect();
                suits.sort();
                suits.dedup();
                suits.len()
            }
        })*
    }
}

impl_MeldVecHas!(for Vec<Meld>, Vec<&Meld>, [Meld], Vec<Pair>, [Pair]);

impl PartialHandTrait for PartialHand {
    fn sort(&mut self) {
        self.pairs.sort();
        self.melds.sort();
    }
    fn is_complete(&self) -> bool {
        self.hanging_tiles.is_empty() && ((self.melds.len() == 4 && self.pairs.len() == 1) || (self.melds.is_empty() && self.pairs.len() == 7))
    }
    fn is_tenpai(&self) -> bool { panic!() }
    fn with_pair(&self, pair: Pair) -> PartialHand {
        PartialHand{
            hanging_tiles: self.hanging_tiles.clone(),
            melds: self.melds.clone(),
            pairs: [self.pairs.clone(), vec![pair]].concat()
    } }
    fn with_meld(&self, meld: Meld) -> PartialHand {
        PartialHand{
            hanging_tiles: self.hanging_tiles.clone(),
            pairs: self.pairs.clone(),
            melds: [self.melds.clone(), vec![meld]].concat()
    } }
}

///////////////
// functions //
///////////////

// Returns only reads in which a hand is complete, ignoring yaku.
// Attempts to dedup.
fn read_win(closed_tiles: Vec<Tile>, called_melds: Option<Vec<Meld>>, latest_tile: Tile) -> Option<Vec<HandShape>> {
    fn compose_kokushi(all_tiles: &Vec<Tile>, latest_tile: &Tile) -> Option<Vec<Yaku>> {
        // TODO: rewrite to use hashset?
        if !all_tiles.has_any_simple() {
            let mut tiles = all_tiles.clone();
            tiles.sort();
            tiles.dedup();
            if tiles.len() == 13 {
                let mut yaku = vec![Yaku::Kokushi];
                if all_tiles.contains(&latest_tile) { yaku.push(Yaku::SpecialWait) }
                return yaku.into()
            }
        } None
    }

    // if there are multiple ways to read the hand, we'll use this to decide which to return
    let mut possible_hands: Vec<HandShape> = Vec::new();

    let called_melds = {if let Some(ref melds) = called_melds { melds.clone() } else { Vec::new() }};

    // add in the latest call ...
    let mut closed_tiles = [closed_tiles, vec![latest_tile]].concat();
    // ... and sort the tiles before passing them to compose_tiles()
    closed_tiles.sort();

    if let Some(partials) = compose_tiles(&closed_tiles, false, None, true) {
        for partial in partials {
            // standard hands
            if partial.hanging_tiles.is_empty() && partial.pairs.len() == 1 && (partial.melds.len() + called_melds.len()) == 4 {
                possible_hands.push(
                    HandShape::Standard {
                        melds: [partial.melds, called_melds.clone()].concat().try_into().expect("Wrong number of melds in hand???"),
                        pair: partial.pairs[0]
                    }
                )

            // chiitoi
            } else if partial.hanging_tiles.is_empty() && partial.pairs.len() == 7 && partial.melds.is_empty() && called_melds.is_empty() {
                possible_hands.push(
                    HandShape::Chiitoi{ pairs: partial.pairs.try_into().expect("Wrong number of pairs in chiitoi???") }
                );
            // kokushi
            // only test for thirteen orphans if it's possible, and if no other hands have been found; otherwise it's a waste of time
            } else if called_melds.is_empty() && possible_hands.is_empty() && partial.hanging_tiles.len() == 12 && partial.pairs.len() == 1 {
                if let Some(yaku) = compose_kokushi(&closed_tiles, &latest_tile) {
                    // a thirteen orphan hand can't be anything else (except special yakuman),
                    // so we'll just return it
                    return vec![HandShape::Kokushi(yaku)].into()
                }
            }
        }
    }
    if possible_hands.is_empty() {  None }
    else { possible_hands.into() }
    
}

// Returns only reads in which a hand is in tenpai, along with their waits.
// If latest_tile is present, also includes which tiles would need to be discarded to enter different tenpais.
// Attempts to dedup.
fn read_tenpai(mut closed_tiles: Vec<Tile>, called_melds: Option<Vec<Meld>>, latest_tile: Option<Tile>) -> Option<Vec<HandShape>> {
    panic!()
}

// Returns only shanten reads, ordered by a naive shanten count.
// If latest_tile is present, also includes which tiles would need to be discarded for different scenarios.
// Does not include information about how a hand might be completed.
// Attempts to dedup.
fn read_shanten(mut closed_tiles: Vec<Tile>, called_melds: Option<Vec<Meld>>, latest_tile: Option<Tile>) -> Option<Vec<HandShape>> {
    panic!()
}

// Given a sorted list of tiles, attempts to compose those tiles into melds and pairs.
// Has hooks to control what it returns (consider_waits and consider_kokushi).
// Does not check for hand validity.
// Requires remaining_tiles to be sorted. Will misbehave otherwise.
fn compose_tiles(remaining_tiles: &Vec<Tile>, open: bool, consider_waits: Option<i8>, consider_kokushi: bool) -> Option<Vec<PartialHand>> {
    let len: usize = remaining_tiles.len();

    if len <= 1 { return None
    } else {
        let mut partials: Vec<PartialHand> = Vec::new();
        //let subset: Vec<Tile> = remaining_tiles[1..].to_vec();
        let first_tile: Tile = remaining_tiles[0];

        if let Some(pair) = remaining_tiles[0..=1].make_pair() {
            let temp: Pair = pair;
            let subs: Vec<Tile> = remaining_tiles[2..].to_vec();

            if let Some(recursions) = compose_tiles(&subs, open, consider_waits, consider_kokushi && (first_tile.is_honor() || first_tile.is_terminal())) {
                for value in recursions {
                    partials.push( value.with_pair(temp) ) }
            } else if consider_waits.is_some() || consider_kokushi || len == 2 {
                partials.push( PartialHand{
                    hanging_tiles: subs,
                    melds: Vec::new(),
                    pairs: vec![temp] } ) 
        } }
        
        if len >= 3 {
            for seq in [first_tile.adjacent_up(), first_tile.adjacent_aside()] {
                if let Some(seq) = seq {
                    let mut subs: Vec<Tile> = Vec::new();
                    let mut temp: Option<Meld> = None;

                    if seq[0] != seq[1] {
                        if let (Ok(index1), Ok(index2)) = (remaining_tiles.binary_search(&seq[0]), remaining_tiles.binary_search(&seq[1])) {
                            temp = [first_tile, remaining_tiles[index1], remaining_tiles[index2]].make_meld(open);
                            subs = remaining_tiles[1..].to_vec();

                            // index1 should always be less than index2, but just in case ...
                            subs.remove(std::cmp::max(index1,index2)-1);
                            subs.remove(std::cmp::min(index1,index2)-1);
                    } } else if remaining_tiles[1] == first_tile && remaining_tiles[2] == first_tile {
                        temp = remaining_tiles[..=2].make_meld(open);

                        // slice away the first two values.
                        subs = remaining_tiles[3..].to_vec();
                    } 

                    if let Some(found) = temp {
                        if let Some(recursions) = compose_tiles(&subs, open, consider_waits, false) {
                            for value in recursions { partials.push( value.with_meld(found) ) }
                        } else if consider_waits.is_some() || subs.len() == 0 {
                            partials.push( PartialHand{
                                hanging_tiles: subs.clone(),
                                melds: vec![found],
                                pairs: Vec::new() } ) }
        } } } }

        // consider the case where the current tile is hanging
        // necessary for wait counting and kokushi
        // ... but only if we explicitly said to consider waits, or if we might be checking for kokushi
        // (makes a pretty significant difference in how long it takes to run test cases)
        if consider_kokushi || consider_waits.is_some() {
            if let Some(recursion) = compose_tiles(&remaining_tiles[1..].to_vec(), open, if let Some(num) = consider_waits { if num > 1 { Some(num-1) } else { None } } else { None }, first_tile.is_honor() || first_tile.is_terminal() ) {
                for value in recursion {
                    partials.push( PartialHand{
                        hanging_tiles: [vec![first_tile], value.hanging_tiles].concat(),
                        melds: value.melds,
                        pairs: value.pairs } )
        } } } 

        partials.sort();
        partials.dedup();

        if partials.is_empty() { return None } else {
            return partials.into()
        }
    }
}

///////////
// tests //
///////////

mod tests {
    use super::*;
    use crate::tiles::{Tile, Dragon, Wind, Suit};
    use crate::conversions::{StringConversions};

    #[test]
    fn test_reading_hand_composition(){
        assert_eq!(compose_tiles(&("we,we").to_tiles().unwrap(), true, None, false), 
            vec![PartialHand {
                pairs: vec![ Pair{tiles: [Tile::Wind(Wind::East); 2]} ],
                melds: Vec::new(), hanging_tiles: Vec::new() }].into());
        assert_eq!(compose_tiles(&("dw,dw,dw,we,we,we").to_tiles().unwrap(), true, None, false), 
            vec![PartialHand {
                melds: vec!["we,we,we".to_meld().unwrap(), "dw,dw,dw".to_meld().unwrap()],
                pairs: Vec::new(), hanging_tiles: Vec::new() }].into());
        assert_eq!(compose_tiles(&("dw,dw,dw,we,we").to_tiles().unwrap(), true, None, false), 
            vec![PartialHand {
                pairs: vec!["we,we".to_tiles().unwrap().make_pair().unwrap()],
                melds: vec!["dw,dw,dw".to_meld().unwrap()],
                hanging_tiles: Vec::new() }].into());
        assert_eq!(compose_tiles(&("dw,dw,we,we,we").to_tiles().unwrap(), true, None, false), 
            vec![PartialHand {
                pairs: vec!["dw,dw".to_tiles().unwrap().make_pair().unwrap()],
                melds: vec!["we,we,we".to_meld().unwrap()],
                hanging_tiles: Vec::new() }].into());
        let mut tiles = "m1,m2,m3,p4,p5r,p3".to_tiles().unwrap(); tiles.sort();
        assert_eq!(compose_tiles(&tiles, true, None, false), 
            vec![PartialHand {
                melds: vec!["p4,p5r,p3".to_meld().unwrap(), "m1,m2,m3".to_meld().unwrap()],
                pairs: Vec::new(), hanging_tiles: Vec::new() }].into());
        tiles = "dw,dr,p4,dw,p5r,p3,dr,dw,m2,m2,m2".to_tiles().unwrap(); tiles.sort();
        assert_eq!(compose_tiles(&tiles, true, None, false), 
            vec![PartialHand {
                pairs: vec!["dr,dr".to_tiles().unwrap().make_pair().unwrap()],
                melds: vec!["dw,dw,dw".to_meld().unwrap(), "p3,p4,p5r".to_meld().unwrap(), "m2,m2,m2".to_meld().unwrap()],
                hanging_tiles: Vec::new() },].into());
        tiles = "m1,m1,m1,m2,m2,m2,m3,m3,m3".to_tiles().unwrap(); tiles.sort();
        assert_eq!(compose_tiles(&tiles, true, None, false).unwrap().iter().filter(|h| h.pairs.is_empty()).collect::<Vec<_>>(), 
            vec![&PartialHand {
                melds: vec!["m1,m2,m3".to_meld().unwrap(), "m1,m2,m3".to_meld().unwrap(), "m1,m2,m3".to_meld().unwrap()],
                pairs: Vec::new(), hanging_tiles: Vec::new() },
            &PartialHand {
                melds: vec!["m3,m3,m3".to_meld().unwrap(), "m2,m2,m2".to_meld().unwrap(), "m1,m1,m1".to_meld().unwrap()],
                pairs: Vec::new(), hanging_tiles: Vec::new() },]);
    }

    #[test]
    fn test_reading_kokushi(){
        // let hand: Hand = compose_hand(make_tiles_from_string("m1,m1,m9,p1,p9,s1,s9,dw,dr,dg,we,ws,wn,ww").unwrap(), None,
        //     Tile::Number{ suit: Suit::Man, number: 1, red: false }, WinType::Tsumo, Wind::East, Wind::South, None, None, RiichiRuleset::Default).unwrap();
        // assert!(matches!(hand, Hand::Kokushi {..}));

        // assert_eq!(hand, Hand::Kokushi{full_hand: make_tiles_from_string("m1,m1,m9,p1,p9,s1,s9,dw,dr,dg,we,ws,wn,ww").unwrap().try_into().unwrap(),
        //     winning_tile: Tile::Number{ suit: Suit::Man, number: 1, red: false }, yaku: vec![Yaku::Kokushi, Yaku::SpecialWait], han: 13, fu: 20});

        // let hand: Hand = compose_hand(crate::tiles::make_tiles_from_string("m1,m1,m9,p1,p9,s1,s9,dw,dr,dg,we,ws,wn,ww").unwrap(), None,
        //     Tile::Number{ suit: Suit::Man, number: 9, red: false }, WinType::Tsumo, Wind::East, Wind::South, None, None, RiichiRuleset::Default).unwrap();
        // assert_eq!(hand, Hand::Kokushi{full_hand: make_tiles_from_string("m1,m1,m9,p1,p9,s1,s9,dw,dr,dg,we,ws,wn,ww").unwrap().try_into().unwrap(),
        //     winning_tile: Tile::Number{ suit: Suit::Man, number: 9, red: false }, yaku: vec![Yaku::Kokushi], han: 13, fu: 20});
    }

    #[test]
    fn test_reading_chiitoi(){         
        // assert!(matches!(compose_hand(make_tiles_from_string("m1,m1,m2,m2,m4,m4,dw,dw,p6,p6,we,we,s5,s5").unwrap().try_into().unwrap(),
        //     None, Tile::Number{ suit: Suit::Man, number: 1, red: false }, WinType::Ron, Wind::East, Wind::South, None, None, RiichiRuleset::Default).unwrap(), Hand::Chiitoi {..}));
        
        // let chiitoi_yaku: Hand = compose_hand(make_tiles_from_string("m2,m2,m3,m3,m4,m4,s2,s2,s5,s5,p3,p3,p6,p6").unwrap().try_into().unwrap(),
        //     None, Tile::Number{ suit: Suit::Man, number: 2, red: false }, WinType::Ron, Wind::East, Wind::South, None, None, RiichiRuleset::Default).unwrap();
        // assert_eq!(chiitoi_yaku.get_yaku(), vec![Yaku::Chiitoi, Yaku::Tanyao]);

        // let chiitoi_yaku: Hand = compose_hand(make_tiles_from_string("m1,m1,m9,m9,p1,p1,we,we,ww,ww,dw,dw,dr,dr").unwrap().try_into().unwrap(),
        //     None, Tile::Number{ suit: Suit::Man, number: 1, red: false }, WinType::Ron, Wind::East, Wind::South, None, None, RiichiRuleset::Default).unwrap();
        // assert_eq!(chiitoi_yaku.get_yaku(), vec![Yaku::Chiitoi, Yaku::Honro]);

        // let chiitoi_yaku: Hand = compose_hand(make_tiles_from_string("dw,dw,dr,dr,dg,dg,we,we,ww,ww,ws,ws,wn,wn").unwrap().try_into().unwrap(),
        //     None, Tile::Number{ suit: Suit::Man, number: 2, red: false }, WinType::Ron, Wind::East, Wind::South, None, None, RiichiRuleset::Default).unwrap();
        // assert_eq!(chiitoi_yaku.get_yaku(), vec![Yaku::Daichiishin]);         
    }
}