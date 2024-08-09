use crate::errors::errors::{HandError, ParsingError};
use crate::rulesets::{RiichiRuleset, RuleVariations};
use core::fmt;
use core::cmp::Ordering;

///////////////////////
// structs and enums //
///////////////////////

#[derive(Debug, Clone, Copy, Hash)]
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

pub fn make_tiles_from_string(str: &str) -> Result<Vec<Tile>, HandError> {
    let mut tiles: Vec<Tile> = Vec::new();
    let input: Vec<&str> = str.split(',').collect();
    for tile in input {
        tiles.push(Tile::from_string(tile)?);
    }
    tiles.sort();
    Ok(tiles)   
}

////////////
// traits //
////////////


pub trait MakeTile {
    fn from_char(char: char) -> Result<Self, HandError> where Self: Sized {panic!()}
    fn from_string(str: &str) -> Result<Self, HandError> where Self: Sized {panic!()} }
pub trait TileHelpers {
    fn is_numbered(&self) -> bool;
    fn is_terminal(&self) -> bool;
    fn is_honor(&self) -> bool;
    fn is_wind(&self) -> bool;
    fn is_dragon(&self) -> bool;
    fn is_pure_green(&self, ruleset: RiichiRuleset) -> bool;
    fn adjacent_all(&self) -> Vec<[Tile; 2]>;
    fn adjacent_up(&self) -> Option<[Tile; 2]>;
    fn adjacent_down(&self) -> Option<[Tile; 2]>;
    fn adjacent_around(&self) -> Option<[Tile; 2]>;
    fn adjacent_aside(&self) -> [Tile; 2];
    fn adjacent(suit: Suit, number: i8, one: i8, two: i8) -> [Tile; 2];
    fn get_number(&self) -> Option<u8>;
    fn get_suit(&self) -> Option<Suit>;
    fn dora(self: &Self) -> Tile; }

impl MakeTile for Tile {
    fn from_string(str: &str) -> Result<Self, HandError> {
        let v: Vec<char> = str.chars().collect();
        match v.len() {
            3 => {
                match v[0] {
                    'p' | 'm' | 's' => {
                        { Ok(Tile::Number{
                            suit: Suit::from_char(v[0])?,
                            number: (v[1] as i8) - 48,
                            red: {if v.get(2) == Some(&'r') { true } else { false }}
                        })}
                    },
                    _ => Err(HandError::ParseError(ParsingError::BadString)),
                }},
            2 => {
                match v[0] {
                    'd' => { Ok(Tile::Dragon(Dragon::from_char(v[1]).unwrap())) }
                    'w' => { Ok(Tile::Wind(Wind::from_char(v[1]).unwrap())) },
                    'p' | 'm' | 's' => {
                        { Ok(Tile::Number{
                            suit: Suit::from_char(v[0])?,
                            number: (v[1] as i8) - 48,
                            red: false
                        })}
                    },
                    _ => Err(HandError::ParseError(ParsingError::BadChar)),
                }},
            _ => Err(HandError::ParseError(ParsingError::BadString)),
        }
} }

impl MakeTile for Dragon {
    fn from_char(char: char) -> Result<Self, HandError> {
        match char {
            'r' => Ok(Dragon::Red),
            'w' => Ok(Dragon::White),
            'g' => Ok(Dragon::Green),
            _ => Err(HandError::ParseError(ParsingError::BadChar)),
        }
    }
}

impl MakeTile for Wind {
    fn from_char(char: char) -> Result<Self, HandError> {
        match char {
            'e' => Ok(Wind::East),
            's' => Ok(Wind::South),
            'w' => Ok(Wind::West),
            'n' => Ok(Wind::North),
            _ => Err(HandError::ParseError(ParsingError::BadChar)),
} } }

impl MakeTile for Suit {
    fn from_char(char: char) -> Result<Self, HandError> {
        match char {
            'p' => Ok(Suit::Pin),
            'm' => Ok(Suit::Man),
            's' => Ok(Suit::Sou),
            _ => Err(HandError::ParseError(ParsingError::BadChar)),
} } }

impl TileHelpers for Tile {
    fn is_numbered(&self) -> bool {
        if let Tile::Number {..} = self { true } else { false}
    }
    fn is_terminal(&self) -> bool {
        if let Tile::Number {number, ..} = self {
                if *number == 1 || *number == 9 { true } else { false }
        } else { false }
    }
    fn is_honor(&self) -> bool {
        if let Tile::Number {..} = self { false } else { true }
    }
    fn is_wind(&self) -> bool{
        if let Tile::Wind {..} = self { true } else { false }
    }
    fn is_dragon(&self) -> bool {
        if let Tile::Dragon {..} = self { true } else { false }
    }
    fn is_pure_green(&self, ruleset: RiichiRuleset) -> bool {
        match self {
            Tile::Dragon(dragon) if ruleset.allows_all_green_hatsu() => if let Dragon::Green = dragon { true } else { false },
            Tile::Number {suit, number, ..} => if let Suit::Sou = suit { [2,3,4,6,8].contains(number) } else { false },
            _ => false,
        }
    }
    fn adjacent_all(&self)  -> Vec<[Tile; 2]> {
        let arr: [Option<[Tile; 2]>; 3] = [self.adjacent_up(), self.adjacent_around(), self.adjacent_down()];
        let mut vec: Vec<[Tile; 2]> = vec![self.adjacent_aside()];
        for element in arr.iter() {
            if let Some(e) = element { vec.push(*e) } }
        vec
    }
    fn adjacent_up(&self)  -> Option<[Tile; 2]> {
        match self {Tile::Number {suit, number, ..} => {
            match number {
                8 | 9 => None,
                _ => Some(Self::adjacent(*suit, *number, 1, 2))
            }},
            _ => None,}
    }
    fn adjacent_down(&self)  -> Option<[Tile; 2]> {
        match self {Tile::Number {suit, number, ..} => {
            match number {
                1 | 2 => None,
                _ => Some(Self::adjacent(*suit, *number, -2, -1))
            }},
            _ => None,}
    }
    fn adjacent_around(&self)  -> Option<[Tile; 2]> {
        match self {Tile::Number {suit, number, ..} => {
            match number {
                1 | 9 => None,
                _ => Some(Self::adjacent(*suit, *number, -1, 1))
            }},
            _ => None,}
    }
    fn adjacent_aside(&self) -> [Tile; 2] {
        [*self, *self] }
    fn adjacent(suit: Suit, number: i8, one: i8, two: i8) -> [Tile; 2] {
        let adj: [Tile; 2] = [
            Tile::Number{suit: suit, number: number + one, red: false},
            Tile::Number{suit: suit, number: number + two, red: false}
        ];
        adj
    }
    fn get_number(&self) -> Option<u8> {
        if let Tile::Number {number, ..} = self { Some(*number as u8) } else { None }
    }
    fn get_suit(&self) -> Option<Suit> {
        if let Tile::Number {suit, ..} = self { Some(*suit) } else { None }
    }
    fn dora (self: &Self) -> Tile {
        match self {
            Tile::Number {suit, number, ..} => {
                if *number == 9 as i8 {
                    Tile::Number{suit: *suit, number: 1, red: false}
                } else {
                    Tile::Number{suit: *suit, number: number + 1, red: false}
                }
            },
            Tile::Dragon(dragon) => {
                match dragon {
                    Dragon::White => Tile::Dragon(Dragon::Green),
                    Dragon::Green => Tile::Dragon(Dragon::Red),
                    Dragon::Red => Tile::Dragon(Dragon::White),
                    
                }
            },
            Tile::Wind(wind) => {
                match wind {
                    Wind::East => Tile::Wind(Wind::South),
                    Wind::South => Tile::Wind(Wind::West),
                    Wind::West => Tile::Wind(Wind::North),
                    Wind::North => Tile::Wind(Wind::East),
                }
            }
        }
    }
}

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
                    Tile::Wind(val2) => val1.cmp(&val2),
                    _ => Ordering::Greater } },
            Tile::Dragon(val1) => {
                match other {
                    Tile::Dragon(val2) => val1.cmp(&val2),
                    Tile::Wind(_) => Ordering::Less,
                    _ => Ordering::Greater } },
            Tile::Number {suit: s1, number: n1, ..} => {
                if let Tile::Number {suit, number, ..} = other { 
                    if s1 == suit { n1.cmp(&number) }
                    else { s1.cmp(&suit) }
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

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Wind(wind) => { write!(f, "w{}", wind) },
            Tile::Dragon(dragon) => { write!(f, "d{}", dragon) },
            Tile::Number {suit, number, red} => { write!(f, "{}{}{}", suit, number, {if *red { "r" } else { "" }}) },
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

///////////
// tests //
///////////

mod tests {
    use super::*;

    #[test]
    fn string_to_tiles(){
        assert_eq!(make_tiles_from_string("m1").unwrap(), vec![Tile::Number{suit: Suit::Man, number: 1, red: false}]);
        assert_eq!(make_tiles_from_string("m3,m2,m1").unwrap(), 
            vec![Tile::Number{suit: Suit::Man, number: 1, red: false},
                Tile::Number{suit: Suit::Man, number: 2, red: false},
                Tile::Number{suit: Suit::Man, number: 3, red: false}]);
        assert_eq!(make_tiles_from_string("we,dw,s2").unwrap(), 
            vec![Tile::Number{suit: Suit::Sou, number: 2, red: false},
                Tile::Dragon(Dragon::White),
                Tile::Wind(Wind::East),]);
        assert_eq!(make_tiles_from_string("s5,s5r,s5").unwrap(), 
            vec![Tile::Number{suit: Suit::Sou, number: 5, red: false},
                Tile::Number{suit: Suit::Sou, number: 5, red: false},
                Tile::Number{suit: Suit::Sou, number: 5, red: true}]);
        assert_eq!(make_tiles_from_string("p1,p3,p2,p5r,p4,p6,p7,p8,p9,we,we,we,dr").unwrap(),
            vec![
                Tile::Number{suit: Suit::Pin, number: 1, red: false},
                Tile::Number{suit: Suit::Pin, number: 2, red: false},
                Tile::Number{suit: Suit::Pin, number: 3, red: false},
                Tile::Number{suit: Suit::Pin, number: 4, red: false},
                Tile::Number{suit: Suit::Pin, number: 5, red: true},
                Tile::Number{suit: Suit::Pin, number: 6, red: false},
                Tile::Number{suit: Suit::Pin, number: 7, red: false},
                Tile::Number{suit: Suit::Pin, number: 8, red: false},
                Tile::Number{suit: Suit::Pin, number: 9, red: false},
                Tile::Dragon(Dragon::Red),
                Tile::Wind(Wind::East),
                Tile::Wind(Wind::East),
                Tile::Wind(Wind::East),
            ]);
    }

    #[test]
    fn tile_equality(){
        // making sure that reddness isn't included in equality ...
        assert_eq!(Tile::Number{suit: Suit::Sou, number: 5, red: true}, Tile::Number{suit: Suit::Sou, number: 5, red: false});
        assert_eq!(Tile::Number{suit: Suit::Sou, number: 5, red: true} > Tile::Number{suit: Suit::Sou, number: 5, red: false}, false);
    }

    #[test]
    fn test_dora(){
        assert_eq!(Tile::Dragon(Dragon::Green).dora(), Tile::Dragon(Dragon::Red));
        assert_eq!(Tile::Wind(Wind::North).dora(), Tile::Wind(Wind::East));
        assert_eq!(Tile::Number{suit: Suit::Sou, number: 1, red: false}.dora(), Tile::Number{suit: Suit::Sou, number: 2, red: false});
        assert_eq!(Tile::Number{suit: Suit::Sou, number: 9, red: false}.dora(), Tile::Number{suit: Suit::Sou, number: 1, red: false});
    }

    #[test]
    fn test_adjacent(){
        assert_eq!(Tile::Number{suit: Suit::Sou, number: 1, red: false}.adjacent_all(), 
            vec![[Tile::from_string("s1").unwrap(), Tile::from_string("s1").unwrap()],
                 [Tile::from_string("s2").unwrap(), Tile::from_string("s3").unwrap()]
            ]);
        assert_eq!(Tile::Number{suit: Suit::Sou, number: 2, red: false}.adjacent_all(), 
            vec![[Tile::from_string("s2").unwrap(), Tile::from_string("s2").unwrap()],
                 [Tile::from_string("s3").unwrap(), Tile::from_string("s4").unwrap()],
                 [Tile::from_string("s1").unwrap(), Tile::from_string("s3").unwrap()],
            ]);
        assert_eq!(Tile::Number{suit: Suit::Sou, number: 5, red: false}.adjacent_all(), 
            vec![[Tile::from_string("s5").unwrap(), Tile::from_string("s5").unwrap()],
                 [Tile::from_string("s6").unwrap(), Tile::from_string("s7").unwrap()],
                 [Tile::from_string("s4").unwrap(), Tile::from_string("s6").unwrap()],
                 [Tile::from_string("s3").unwrap(), Tile::from_string("s4").unwrap()]
            ]);
        assert_eq!(Tile::Number{suit: Suit::Sou, number: 8, red: false}.adjacent_all(), 
            vec![[Tile::from_string("s8").unwrap(), Tile::from_string("s8").unwrap()],
                 [Tile::from_string("s7").unwrap(), Tile::from_string("s9").unwrap()],
                 [Tile::from_string("s6").unwrap(), Tile::from_string("s7").unwrap()]
            ]);
        assert_eq!(Tile::Number{suit: Suit::Sou, number: 9, red: false}.adjacent_all(), 
            vec![[Tile::from_string("s9").unwrap(), Tile::from_string("s9").unwrap()],
                 [Tile::from_string("s7").unwrap(), Tile::from_string("s8").unwrap()]
            ]);
        assert_eq!(Tile::Wind(Wind::East).adjacent_all(),
            vec![[Tile::from_string("we").unwrap(), Tile::from_string("we").unwrap()]]);
        assert_eq!(Tile::Dragon(Dragon::Red).adjacent_all(),
            vec![[Tile::from_string("dr").unwrap(), Tile::from_string("dr").unwrap()]]);
    }
}