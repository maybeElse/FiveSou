use crate::errors::errors::{ScoringError, ParsingError};
use core::fmt;
    

///////////////////////
// structs and enums //
///////////////////////

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum Tile {
    Number{
        suit: Suit,
        number: i8,
        red: bool
    },
    Dragon(Dragon),
    Wind(Wind)
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum Suit {Man, Sou, Pin,}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum Dragon {White, Green, Red,}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum Wind {East, South, West, North,}

///////////////
// functions //
///////////////

pub fn make_tiles_from_string(str: &str) -> Result<Vec<Tile>, ScoringError> {
    let mut tiles: Vec<Tile> = vec![];
    let mut input: Vec<&str> = str.split(',').collect();
    input.sort();
    for tile in input {
        tiles.push(Tile::from_string(tile)?);
    }
    Ok(tiles)   
}

////////////
// traits //
////////////

pub trait FromString {fn from_string(str: &str) -> Result<Self, ScoringError> where Self: Sized;}
pub trait FromChar {fn from_char(char: char) -> Result<Self, ScoringError> where Self: Sized;}
pub trait AsTiles {fn from_string(str: &str) -> Result<Self, ScoringError> where Self: Sized;}
pub trait TileHelpers {
    fn is_numbered(&self) -> bool;
    fn is_terminal(&self) -> bool;
    fn is_honor(&self) -> bool;
    fn is_wind(&self) -> bool;
    fn is_dragon(&self) -> bool;
    fn is_pure_green(&self) -> bool;
    fn adjacent_all(&self) -> Vec<[Tile; 2]>;
    fn adjacent_up(&self) -> Option<[Tile; 2]>;
    fn adjacent_down(&self) -> Option<[Tile; 2]>;
    fn adjacent_around(&self) -> Option<[Tile; 2]>;
    fn adjacent(suit: Suit, number: i8, one: i8, two: i8) -> [Tile; 2];
    fn get_number(&self) -> Result<u8, ScoringError>;
    fn dora(self: &Self) -> Tile;
}

impl FromString for Tile {
    fn from_string(str: &str) -> Result<Self, ScoringError> {
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
                    _ => Err(ScoringError::ParseError(ParsingError::BadString)),
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
                    _ => Err(ScoringError::ParseError(ParsingError::BadChar)),
                }},
            _ => Err(ScoringError::ParseError(ParsingError::BadString)),
        }
} }

impl FromChar for Dragon {
    fn from_char(char: char) -> Result<Self, ScoringError> {
        match char {
            'r' => Ok(Dragon::Red),
            'w' => Ok(Dragon::White),
            'g' => Ok(Dragon::Green),
            _ => Err(ScoringError::ParseError(ParsingError::BadChar)),
        }
    }
}

impl FromChar for Wind {
    fn from_char(char: char) -> Result<Self, ScoringError> {
        match char {
            'e' => Ok(Wind::East),
            's' => Ok(Wind::South),
            'w' => Ok(Wind::West),
            'n' => Ok(Wind::North),
            _ => Err(ScoringError::ParseError(ParsingError::BadChar)),
} } }

impl FromChar for Suit {
    fn from_char(char: char) -> Result<Self, ScoringError> {
        match char {
            'p' => Ok(Suit::Pin),
            'm' => Ok(Suit::Man),
            's' => Ok(Suit::Sou),
            _ => Err(ScoringError::ParseError(ParsingError::BadChar)),
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
    fn is_pure_green(&self) -> bool {
        match self {
            Tile::Dragon(dragon) => if let Dragon::Green = dragon { true } else { false },
            Tile::Wind(_) => false,
            Tile::Number {suit, number, ..} => if let Suit::Sou = suit { [2,3,4,6,8].contains(number) } else { false }
        }
    }
    fn adjacent_all(&self)  -> Vec<[Tile; 2]> {
        let arr: [Option<[Tile; 2]>; 3] = [self.adjacent_up(), self.adjacent_around(), self.adjacent_down()];
        let mut vec: Vec<[Tile; 2]> = vec![];
        for element in arr.iter() {
            if element.is_some() { vec.push(element.unwrap()) }
        }
        vec
    }
    fn adjacent_up(&self)  -> Option<[Tile; 2]> {
        match self {Tile::Number {suit, number, ..} => {
            match number {
                8 | 9 => None,
                _ => Some(Self::adjacent(*suit, *number, 1, 2))
            }},
            _ => panic!("unreachable code in adjacent_up"),}
    }
    fn adjacent_down(&self)  -> Option<[Tile; 2]> {
        match self {Tile::Number {suit, number, ..} => {
            match number {
                1 | 2 => None,
                _ => Some(Self::adjacent(*suit, *number, -1, -2))
            }},
            _ => panic!("unreachable code in adjacent_down"),}
    }
    fn adjacent_around(&self)  -> Option<[Tile; 2]> {
        match self {Tile::Number {suit, number, ..} => {
            match number {
                1 | 9 => None,
                _ => Some(Self::adjacent(*suit, *number, -1, 1))
            }},
            _ => panic!("unreachable code in adjacent_around"),}
    }
    fn adjacent(suit: Suit, number: i8, one: i8, two: i8) -> [Tile; 2] {
        let adj: [Tile; 2] = [
            Tile::Number{suit: suit, number: number + one, red: false},
            Tile::Number{suit: suit, number: number + two, red: false}
        ];
        adj
    }
    fn get_number(&self) -> Result<u8, ScoringError> {
        if let Tile::Number {number, ..} = self { Ok(*number as u8) } else { Err(ScoringError::TileError) }
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
                }},
            Tile::Dragon(val1) => {
                if let Tile::Dragon(val2) = other { val1 == val2 } else { false }
                },
            Tile::Number {suit, number, red} => {
                let s1 = suit;
                let n1 = number;
                if let Tile::Number {suit, number, ..} = other { s1 == suit && n1 == number } else { false }
            }
        }
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
            vec![Tile::Dragon(Dragon::White),
                Tile::Number{suit: Suit::Sou, number: 2, red: false},
                Tile::Wind(Wind::East),]);
        assert_eq!(make_tiles_from_string("s5,s5r,s5").unwrap(), 
            vec![Tile::Number{suit: Suit::Sou, number: 5, red: false},
                Tile::Number{suit: Suit::Sou, number: 5, red: false},
                Tile::Number{suit: Suit::Sou, number: 5, red: true}]);
        assert_eq!(make_tiles_from_string("p1,p3,p2,p5r,p4,p6,p7,p8,p9,we,we,we,dr").unwrap(),
            vec![Tile::Dragon(Dragon::Red),
                Tile::Number{suit: Suit::Pin, number: 1, red: false},
                Tile::Number{suit: Suit::Pin, number: 2, red: false},
                Tile::Number{suit: Suit::Pin, number: 3, red: false},
                Tile::Number{suit: Suit::Pin, number: 4, red: false},
                Tile::Number{suit: Suit::Pin, number: 5, red: true},
                Tile::Number{suit: Suit::Pin, number: 6, red: false},
                Tile::Number{suit: Suit::Pin, number: 7, red: false},
                Tile::Number{suit: Suit::Pin, number: 8, red: false},
                Tile::Number{suit: Suit::Pin, number: 9, red: false},
                Tile::Wind(Wind::East),
                Tile::Wind(Wind::East),
                Tile::Wind(Wind::East),
            ]);
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
            vec![[Tile::from_string("s2").unwrap(), Tile::from_string("s3").unwrap()]]);
        assert_eq!(Tile::Number{suit: Suit::Sou, number: 2, red: false}.adjacent_all(), 
            vec![[Tile::from_string("s3").unwrap(), Tile::from_string("s4").unwrap()],
                 [Tile::from_string("s1").unwrap(), Tile::from_string("s3").unwrap()]
            ]);
        assert_eq!(Tile::Number{suit: Suit::Sou, number: 5, red: false}.adjacent_all(), 
            vec![[Tile::from_string("s6").unwrap(), Tile::from_string("s7").unwrap()],
                 [Tile::from_string("s4").unwrap(), Tile::from_string("s6").unwrap()],
                 [Tile::from_string("s4").unwrap(), Tile::from_string("s3").unwrap()]
            ]);
        assert_eq!(Tile::Number{suit: Suit::Sou, number: 8, red: false}.adjacent_all(), 
            vec![[Tile::from_string("s7").unwrap(), Tile::from_string("s9").unwrap()],
                 [Tile::from_string("s7").unwrap(), Tile::from_string("s6").unwrap()]
            ]);
        assert_eq!(Tile::Number{suit: Suit::Sou, number: 9, red: false}.adjacent_all(), 
            vec![[Tile::from_string("s8").unwrap(), Tile::from_string("s7").unwrap()]]);
    }
}