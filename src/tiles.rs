use crate::errors::mahjong_errors::{HandError, ParsingError};
use crate::rulesets::{RiichiRuleset, RuleVariations};
use crate::hand::Meld;
use crate::conversions::{ConvertStrings, ConvertChars};
use core::fmt;
use core::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{DefaultHasher, Hash, Hasher};
use itertools::Itertools;

///////////////////////
// structs and enums //
///////////////////////

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Number{
        suit: Suit,
        number: i8,
        red: bool
    },
    Dragon(Dragon),
    Wind(Wind)
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum Suit {Man, Sou, Pin,}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum Dragon {White, Green, Red,}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum Wind {East, South, West, North,}

///////////////
// functions //
///////////////

pub fn make_tiles_from_string(str: &str) -> Result<Vec<Tile>, ParsingError> {
    let mut tiles: Vec<Tile> = str.split(',').map(|t| t.to_tile().expect("string should be a tile") ).collect();
    tiles.sort();
    Ok(tiles) 
}

////////////
// traits //
////////////

pub trait TileIs {
    fn is_numbered(&self) -> bool;
    fn is_terminal(&self) -> bool;
    fn is_simple(&self) -> bool;
    fn is_honor(&self) -> bool;
    fn is_wind(&self) -> bool;
    fn is_dragon(&self) -> bool;
    fn is_pure_green(&self, ruleset: &RiichiRuleset) -> bool;
    fn suit(&self) -> Option<Suit>;
    fn number(&self) -> Option<i8>;
    fn wind(&self) -> Option<Wind>;
    fn dragon(&self) -> Option<Dragon>;
}

pub trait TileRelations {
    fn adjacent_all(&self) -> Vec<[Tile; 2]>;       // returns all possible adjacent tiles
    fn adjacent_up(&self) -> Option<[Tile; 2]>;     // for numbered tiles, returns n+1 and n+2 (if possible)
    fn adjacent_down(&self) -> Option<[Tile; 2]>;   // for numbered tiles, returns n-1 and n-2 (if possible)
    fn adjacent_around(&self) -> Option<[Tile; 2]>; // 
    fn adjacent_aside(&self) -> Option<[Tile; 2]>;  // returns itself, twice.
    fn adjacent(&self, one: i8, two: i8) -> Option<[Tile; 2]>; // helper for the others
}

pub trait DoraTrait {
    fn dora(&self) -> Self where Self: Sized;
}

pub trait TileVecTrait {
    fn count_occurrences(&self, tile: &Tile) -> usize;
    fn remove_occurences(&mut self, tile: &Tile, count: usize);
    fn count_suits(&self) -> usize;
    fn has_any_simple(&self) -> bool;
    fn has_any_honor(&self) -> bool;
    fn has_any_terminal(&self) -> bool;
    fn count_dora(&self, dora_markers: &Option<Vec<Tile>>) -> u8;
}

/////////////////////
// implementations //
/////////////////////

impl TileIs for Tile {
    fn is_numbered(&self) -> bool {
        matches!(self, Tile::Number{..})
    }
    fn is_terminal(&self) -> bool {
        if let Tile::Number{number, ..} = self {
            if *number == 1 || *number == 9 { return true }
        }
        false
    }
    fn is_simple(&self) -> bool {
        if let Tile::Number{number, ..} = self {
            if *number != 1 && *number != 9 { return true }
        }
        false
    }
    fn is_honor(&self) -> bool {
        !matches!(self, Tile::Number{..})
    }
    fn is_wind(&self) -> bool {
        matches!(self, Tile::Wind(_))
    }
    fn is_dragon(&self) -> bool {
        matches!(self, Tile::Dragon(_))
    }
    fn is_pure_green(&self, ruleset: &RiichiRuleset) -> bool {
        match self {
            Tile::Dragon(dragon) if ruleset.allows_all_green_hatsu() => matches!(dragon, Dragon::Green),
            Tile::Number {suit:Suit::Sou, number, ..} => [2,3,4,6,8].contains(number),
            _ => false,
    } }
    fn number(&self) -> Option<i8> {
        if let Tile::Number{number, ..} = self { return Some(*number) }
        None
    }
    fn suit(&self) -> Option<Suit> {
        if let Tile::Number{suit, ..} = self { return Some(*suit) }
        None
    }
    fn wind(&self) -> Option<Wind> {
        if let Tile::Wind(wind) = self { return Some(*wind) }
        None
    }
    fn dragon(&self) -> Option<Dragon> {
        if let Tile::Dragon(dragon) = self { return Some(*dragon) }
        None
    }
}

impl TileRelations for Tile {
    fn adjacent_all(&self) -> Vec<[Tile; 2]> { panic!() }
    fn adjacent_up(&self) -> Option<[Tile; 2]> { self.adjacent(1,2) }
    fn adjacent_down(&self) -> Option<[Tile; 2]> { self.adjacent(-2,1) }
    fn adjacent_around(&self) -> Option<[Tile; 2]> { self.adjacent(-1,1) }
    fn adjacent_aside(&self) -> Option<[Tile; 2]> { Some([*self, *self]) }
    fn adjacent(&self, one: i8, two: i8) -> Option<[Tile; 2]> {
        if let Tile::Number {suit, number, ..} = self {
            match number {
                8 | 9 => None,
                _ => Some([
                    Tile::Number{suit: *suit, number: number + one, red: false},
                    Tile::Number{suit: *suit, number: number + two, red: false} ])
        } } else { None }
    }
}

impl DoraTrait for Tile {
    fn dora(&self) -> Tile {
        match self {
            Tile::Number {suit, number, ..} => {
                if *number == 9 { Tile::Number{suit: *suit, number: 1, red: false} }
                else { Tile::Number{suit: *suit, number: number + 1, red: false} } },
            Tile::Dragon(dragon) => Tile::Dragon(dragon.dora()),
            Tile::Wind(wind) => Tile::Wind(wind.dora()),
    } }
}

impl DoraTrait for Wind {
    fn dora(&self) -> Self where Self: Sized {
        match self {
            Wind::East => Wind::South,
            Wind::South => Wind::West,
            Wind::West => Wind::North,
            Wind::North => Wind::East,
        }
    }
}

impl DoraTrait for Dragon {
    fn dora(&self) -> Self where Self: Sized {
        match self {
            Dragon::White => Dragon::Green,
            Dragon::Green => Dragon::Red,
            Dragon::Red => Dragon::White,
        }
    }
}

macro_rules! impl_TileVecTrait {
    (for $($t:ty),+) => {
        $(impl TileVecTrait for $t {
            fn count_occurrences(&self, tile: &Tile) -> usize {
                self.iter().fold(0, |acc, value| {if tile == value { acc + 1 } else { acc }})
            }
            fn remove_occurences(&mut self, tile: &Tile, count: usize) { panic!() }
            fn count_suits(&self) -> usize {
                let mut s = [false; 3];
                let mut i = self.iter();

                while let Some(t) = i.next() {
                    match t.suit() {
                        Some(Suit::Sou) => s[0] = true,
                        Some(Suit::Pin) => s[1] = true,
                        Some(Suit::Man) => s[2] = true,
                        None => (),
                    }
                    if s[0] && s[1] && s[2] { break }
                }

                s.iter().fold(0, |acc, v| if *v { acc + 1 } else { acc })

                //self.iter().map(|x| x.suit()).flatten().unique().count()
            }
            fn has_any_simple(&self) -> bool {
                self.iter().any(|t| t.is_numbered() && !t.is_terminal())
            }
            fn has_any_honor(&self) -> bool {
                self.iter().any(|t| t.is_honor())
            }
            fn has_any_terminal(&self) -> bool {
                self.iter().any(|t| t.is_terminal())
            }
            fn count_dora(&self, dora_markers: &Option<Vec<Tile>>) -> u8 {
                if let Some(markers) = dora_markers {
                    let mut dora_tiles: Vec<_> = markers.iter().map(|t| t.dora()).collect();
                    dora_tiles.sort();
                    self.iter().fold(0, |acc, t| {
                        if let Ok(_) = dora_tiles.binary_search(&t) { acc + 1 }
                        else { acc }
                    })
                } else { 0 }
            }
        })*
    }
}

impl_TileVecTrait!(for Vec<Tile>, [Tile]);

/////////////////////////////
/// equality & ordinality ///
/////////////////////////////

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Tile::Wind(val1) => {
                match other {
                    Tile::Wind(val2) => val1 == val2,
                    _ => false,
                } },
            Tile::Dragon(val1) => {
                if let Tile::Dragon(val2) = other { val1 == val2 } else { false }
                },
            Tile::Number {suit: s1, number: n1, ..} => {
                if let Tile::Number {suit, number, ..} = other { s1 == suit && n1 == number } else { false }
            }
        }
    }
}
impl Eq for Tile {}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Tile::Wind(val1) => {
                match other {
                    Tile::Wind(val2) => val1.cmp(val2),
                    _ => Ordering::Greater } },
            Tile::Dragon(val1) => {
                match other {
                    Tile::Dragon(val2) => val1.cmp(val2),
                    Tile::Wind(_) => Ordering::Less,
                    Tile::Number{..} => Ordering::Greater } },
            Tile::Number {suit: s1, number: n1, ..} => {
                if let Tile::Number {suit, number, ..} = other { 
                    if s1 == suit { n1.cmp(number) }
                    else { s1.cmp(suit) }
                } else {Ordering::Less }
            }
        }
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Tile::Number { suit, number, red } => {
                state.write_u8(1);
                suit.hash(state);
                number.hash(state);
            },
            Tile::Dragon(inside) => {
                state.write_u8(2);
                inside.hash(state);
            },
            Tile::Wind(inside) => {
                state.write_u8(3);
                inside.hash(state);
            },
        }
    }
}

//////////////////
/// formatting ///
//////////////////

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Wind(wind) => { write!(f, "w{wind}") },
            Tile::Dragon(dragon) => { write!(f, "d{dragon}") },
            Tile::Number {suit, number, red} => { write!(f, "{suit}{number}{}", {if *red { "r" } else { "" }}) },
        }
    }
}

impl fmt::Display for Dragon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dragon::Red => write!(f, "r",),
            Dragon::White => write!(f, "w",),
            Dragon::Green => write!(f, "g",),
        }
    }
}

impl fmt::Display for Wind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Wind::East => write!(f, "e",),
            Wind::South => write!(f, "s",),
            Wind::West => write!(f, "w",),
            Wind::North => write!(f, "n",),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Pin => write!(f, "p",),
            Suit::Man => write!(f, "m",),
            Suit::Sou => write!(f, "s",),
        }
    }
}