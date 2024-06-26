use crate::errors::errors::{ScoringError, ParsingError};
    
trait FromString {
    fn from_string(str: &str) -> Result<Self, ScoringError> where Self: Sized;
    
}
trait FromChar {fn from_char(char: char) -> Result<Self, ScoringError> where Self: Sized;}
trait AsTiles {fn from_string(str: &str) -> Result<Self, ScoringError> where Self: Sized;}

pub fn make_tiles_from_string(str: &str) -> Result<Vec<Tile>, ScoringError> {
    let mut vec: Vec<Tile> = Vec::new();
    let mut hand: Vec<&str> = str.split(',').collect::<Vec<&str>>();
    hand.sort();
    for tile in hand {
        vec.push(Tile::from_string(tile)?);
    }
    Ok(vec)   
}

impl FromString for Tile {
    fn from_string(str: &str) -> Result<Self, ScoringError> {
        let v: Vec<char> = str.chars().collect();
        match v.len() {
            3 => {
                match v[0] {
                    'p' | 'm' | 's' => Ok(Tile::Simple(SimpleTile::from_string(str).unwrap())),
                    _ => Err(ScoringError::ParseError(ParsingError::BadString)),
                }},
            2 => {
                match v[0] {
                    'd' | 'w' => Ok(Tile::Honor(HonorTile::from_string(str).unwrap())),
                    'p' | 'm' | 's' => Ok(Tile::Simple(SimpleTile::from_string(str).unwrap())),
                    _ => Err(ScoringError::ParseError(ParsingError::BadChar)),
                }},
            _ => Err(ScoringError::ParseError(ParsingError::BadString)),
        }
    }
}

impl FromString for SimpleTile {
    fn from_string(str: &str) -> Result<Self, ScoringError> {
        let v: Vec<char> = str.chars().collect();
        let i: i8 = (v[1] as i8) - 48;
        if !(i > 0 && i <= 9) {
            Err(ScoringError::ParseError(ParsingError::BadInteger))
        } else {
            Ok(SimpleTile{
                suit: Suit::from_char(v[0])?,
                number: i,
                red: {if v.get(2) == Some(&'r') { true } else { false }},
            })
        }
    }
}

impl FromString for HonorTile {
    fn from_string(str: &str) -> Result<Self, ScoringError> {
        let v: Vec<char> = str.chars().collect();
        match v[0] {
            'd' => Ok(HonorTile::Dragon(DragonTile::from_char(v[1]).unwrap())),
            'w' => Ok(HonorTile::Wind(WindTile::from_char(v[1]).unwrap())),
            _ => Err(ScoringError::ParseError(ParsingError::BadChar)),
        }
    }
}

impl FromChar for DragonTile {
    fn from_char(char: char) -> Result<Self, ScoringError> {
        match char {
            'r' => Ok(DragonTile::Red),
            'w' => Ok(DragonTile::White),
            'g' => Ok(DragonTile::Green),
            _ => Err(ScoringError::ParseError(ParsingError::BadChar)),
        }
    }
}

impl FromChar for WindTile {
    fn from_char(char: char) -> Result<Self, ScoringError> {
        match char {
            'e' => Ok(WindTile::East),
            's' => Ok(WindTile::East),
            'w' => Ok(WindTile::East),
            'n' => Ok(WindTile::East),
            _ => Err(ScoringError::ParseError(ParsingError::BadChar)),
        }
    }
}

impl FromChar for Suit {
    fn from_char(char: char) -> Result<Self, ScoringError> {
        match char {
            'p' => Ok(Suit::Pin),
            'm' => Ok(Suit::Man),
            's' => Ok(Suit::Sou),
            _ => Err(ScoringError::ParseError(ParsingError::BadChar)),
        }
    }
}

trait TileHelpers {
    fn is_terminal(self: &Self) -> bool;
    fn adjacent_up(self: &Self) -> Option<[Tile; 2]>;
    fn adjacent_down(self: &Self) -> Option<[Tile; 2]>;
    fn adjacent_around(self: &Self) -> Option<[Tile; 2]>;
}

impl TileHelpers for SimpleTile {

    fn is_terminal(self: &Self) -> bool {
        match self.number {
            1 | 9 => true,
            _ => false,
        }
    }

    fn adjacent_up(self: &Self) -> Option<[Tile; 2]> {
        match self.number {
            8 | 9 => None,
            _ => {
                let adj: [Tile; 2] = [
                    Tile::Simple(SimpleTile{suit: self.suit, number: self.number + 1, red: false}),
                    Tile::Simple(SimpleTile{suit: self.suit, number: self.number + 2, red: false})
                ];
                Some(adj)
            }
        }
    }

    fn adjacent_down(self: &Self) -> Option<[Tile; 2]> {
        match self.number {
            1 | 2 => None,
            _ => {
                let adj: [Tile; 2] = [
                    Tile::Simple(SimpleTile{suit: self.suit, number: self.number - 1, red: false}),
                    Tile::Simple(SimpleTile{suit: self.suit, number: self.number - 2, red: false})
                ];
                Some(adj)
            }
        }
    }

    fn adjacent_around(self: &Self) -> Option<[Tile; 2]> {
        match self.number {
            1 | 9 => None,
            _ => {
                let adj: [Tile; 2] = [
                    Tile::Simple(SimpleTile{suit: self.suit, number: self.number + 1, red: false}),
                    Tile::Simple(SimpleTile{suit: self.suit, number: self.number - 1, red: false})
                ];
                Some(adj)
            }
        }
    }
}

trait DoraOf {
    fn dora_of (self: &Self) -> Tile;
}

impl DoraOf for Tile {
    fn dora_of (self: &Self) -> Tile {
        match self {
            Tile::Simple(simp) => {
                if simp.number == 9 {
                    Tile::Simple(SimpleTile{suit: simp.suit, number: 1, red: false})
                } else {
                    Tile::Simple(SimpleTile{suit: simp.suit, number: simp.number + 1, red: false})
                }
            },
            Tile::Honor(hon) => {
                match hon { // there should be a better way to do this
                    HonorTile::Dragon(DragonTile::White) => Tile::Honor(HonorTile::Dragon(DragonTile::Green)),
                    HonorTile::Dragon(DragonTile::Green) => Tile::Honor(HonorTile::Dragon(DragonTile::Red)),
                    HonorTile::Dragon(DragonTile::Red) => Tile::Honor(HonorTile::Dragon(DragonTile::White)),
                    HonorTile::Wind(WindTile::East) => Tile::Honor(HonorTile::Wind(WindTile::South)),
                    HonorTile::Wind(WindTile::South) => Tile::Honor(HonorTile::Wind(WindTile::West)),
                    HonorTile::Wind(WindTile::West) => Tile::Honor(HonorTile::Wind(WindTile::North)),
                    HonorTile::Wind(WindTile::North) => Tile::Honor(HonorTile::Wind(WindTile::East)),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Meld {
    Triplet(Triplet),
    Sequence(Sequence),
    Kan(Kan),
    Pair(Pair)
}

#[derive(Debug, PartialEq)]
pub struct Triplet {
    pub open: bool,
    pub tile: Tile
}

#[derive(Debug, PartialEq)]
pub struct Sequence {
    pub open: bool,
    pub tiles: [Tile; 3]
}

#[derive(Debug, PartialEq)]
pub struct Kan {
    pub open: bool,
    pub tile: Tile
}

#[derive(Debug, PartialEq)]
pub struct Pair {
    pub open: bool,
    pub tile: Tile
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tile {
    Simple(SimpleTile),
    Honor(HonorTile),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SimpleTile {
    pub suit: Suit,
    pub number: i8,
    pub red: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HonorTile {
    Dragon(DragonTile),
    Wind(WindTile)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {Man, Sou, Pin,}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DragonTile {White, Green, Red,}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WindTile {East, South, West, North,}

mod tests {
    use super::*;

    #[test]
    fn string_to_tiles(){
        assert_eq!(make_tiles_from_string("m1").unwrap(), vec![Tile::Simple(SimpleTile{suit: Suit::Man, number: 1, red: false})]);
        assert_eq!(make_tiles_from_string("m3,m2,m1").unwrap(), 
            vec![Tile::Simple(SimpleTile{suit: Suit::Man, number: 1, red: false}),
                Tile::Simple(SimpleTile{suit: Suit::Man, number: 2, red: false}),
                Tile::Simple(SimpleTile{suit: Suit::Man, number: 3, red: false})]);
        assert_eq!(make_tiles_from_string("we,dw,s2").unwrap(), 
            vec![Tile::Honor(HonorTile::Dragon(DragonTile::White)),
                Tile::Simple(SimpleTile{suit: Suit::Sou, number: 2, red: false}),
                Tile::Honor(HonorTile::Wind(WindTile::East)),]);
        assert_eq!(make_tiles_from_string("s5,s5r,s5").unwrap(), 
            vec![Tile::Simple(SimpleTile{suit: Suit::Sou, number: 5, red: false}),
                Tile::Simple(SimpleTile{suit: Suit::Sou, number: 5, red: false}),
                Tile::Simple(SimpleTile{suit: Suit::Sou, number: 5, red: true})]);
        assert_eq!(make_tiles_from_string("p1,p3,p2,p5r,p4,p6,p7,p8,p9,we,we,we,dr").unwrap(),
            vec![Tile::Honor(HonorTile::Dragon(DragonTile::Red)),
                Tile::Simple(SimpleTile{suit: Suit::Pin, number: 1, red: false}),
                Tile::Simple(SimpleTile{suit: Suit::Pin, number: 2, red: false}),
                Tile::Simple(SimpleTile{suit: Suit::Pin, number: 3, red: false}),
                Tile::Simple(SimpleTile{suit: Suit::Pin, number: 4, red: false}),
                Tile::Simple(SimpleTile{suit: Suit::Pin, number: 5, red: true}),
                Tile::Simple(SimpleTile{suit: Suit::Pin, number: 6, red: false}),
                Tile::Simple(SimpleTile{suit: Suit::Pin, number: 7, red: false}),
                Tile::Simple(SimpleTile{suit: Suit::Pin, number: 8, red: false}),
                Tile::Simple(SimpleTile{suit: Suit::Pin, number: 9, red: false}),
                Tile::Honor(HonorTile::Wind(WindTile::East)),
                Tile::Honor(HonorTile::Wind(WindTile::East)),
                Tile::Honor(HonorTile::Wind(WindTile::East)),
            ]);
    }
}