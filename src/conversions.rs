use crate::errors::errors::{HandError, ParsingError};
use crate::tiles::{Tile, Suit, Dragon, Wind, TileIs, TileRelations, TileVecTrait};
use crate::hand::{Meld, Pair};
use crate::yaku::{Yaku, YakuHelpers};

////////////
// traits //
////////////

pub trait StringConversions {
    fn to_tile(&self) -> Result<Tile, ParsingError>;
    fn to_tiles(&self) -> Result<Vec<Tile>, ParsingError>;
    fn to_meld(&self) -> Result<Meld, ParsingError>;
    fn to_calls(&self) -> Result<Vec<Meld>, ParsingError>;
    fn to_yaku(&self) -> Result<Yaku,ParsingError>;
    fn to_yaku_vec(&self) -> Result<Vec<Yaku>, ParsingError>;
}

pub trait CharConversions {
    fn to_dragon(&self) -> Result<Dragon, ParsingError>;
    fn to_wind(&self) -> Result<Wind, ParsingError>;
    fn to_suit(&self) -> Result<Suit, ParsingError>;
}

pub trait TileConversions {
    fn make_meld(&self, open: bool) -> Option<Meld>;
    fn make_pair(&self) -> Option<Pair>;
}

/////////////////////
// implementations //
/////////////////////

impl StringConversions for str {
    fn to_tile(&self) -> Result<Tile, ParsingError> {
        match self.chars().nth(0).expect("string should not be empty") {
            'p' | 'm' | 's' if self.chars().nth(1).is_some_and(|c| c.is_digit(10)) && ( self.len() == 2 ||  self.len() == 3) => {
                Ok(Tile::Number{
                    suit: self.chars().nth(0).expect("first char should a suit").to_suit()?,
                    number: self.chars().nth(1).expect("second char should be a number").to_digit(10).ok_or(ParsingError::BadInteger)? as i8,
                    red: {if self.chars().nth(2) == Some('r') { true } else { false }}
                })
            },
            'd' if self.len() == 2 => {
                Ok(Tile::Dragon(self.chars().nth(1).expect("second char should be a dragon").to_dragon()?))
            },
            'w' if self.len() == 2 => {
                Ok(Tile::Wind(self.chars().nth(1).expect("second char should be a wind").to_wind()?))
            },
            _ => Err(ParsingError::BadString)
        }
    }
    fn to_tiles(&self) -> Result<Vec<Tile>, ParsingError> {
        if self.is_empty() { return Err(ParsingError::Empty) }
        else {
            Ok(self.split(',').map(|s| s.to_tile().expect("string should be a tile")).collect::<Vec<_>>())
        }
    }
    fn to_meld(&self) -> Result<Meld, ParsingError> {
        if self.is_empty() { return Err(ParsingError::Empty) }
        else {
            if self.chars().nth(0) == Some('!') { self[1..].to_tiles()?.make_meld(false).ok_or(ParsingError::BadMeld) }
            else { self.to_tiles()?.make_meld(true).ok_or(ParsingError::BadMeld) }
        }
    }
    fn to_calls(&self) -> Result<Vec<Meld>, ParsingError> {
        if self.is_empty() { return Ok(Vec::new()) }
        else { return Ok(self.split('|').map(|s| s.to_meld().expect("tiles should be a valid meld")).collect()) }
    }
    fn to_yaku(&self) -> Result<Yaku,ParsingError> {
        match self.to_lowercase().as_str() {
            "riichi" => Ok(Yaku::Riichi),
            "ippatsu" => Ok(Yaku::Ippatsu),
            "doubleriichi" => Ok(Yaku::DoubleRiichi),
            "undersea" | "underthesea" | "haiteiraoyue" | "haitei" => Ok(Yaku::UnderSea),
            "underriver" | "undertheriver" | "houteiraoyui" | "houtei" => Ok(Yaku::UnderRiver),
            "afterkan" | "rinshan" | "rinshankaiho" => Ok(Yaku::AfterKan),
            "robbedkan" | "robbingakan" | "chankan" => Ok(Yaku::RobbedKan),
            "nagashimangan" => Ok(Yaku::NagashiMangan),
            "tenho" | "blessingofheaven" => Ok(Yaku::Tenho),
            "chiho" | "blessingofearth" => Ok(Yaku::Chiho),
            _ => return Err(ParsingError::BadString)
        }
    }
    fn to_yaku_vec(&self) -> Result<Vec<Yaku>, ParsingError> {
        let mut yaku: Vec<Yaku> = Vec::new();
        yaku.append_checked(&(self.split(',').map(|s| s.to_yaku().unwrap()).collect()));
        Ok(yaku)
    }
}

impl CharConversions for char {
    fn to_dragon(&self) -> Result<Dragon, ParsingError> {
        match self {
            'r' => Ok(Dragon::Red),
            'w' => Ok(Dragon::White),
            'g' => Ok(Dragon::Green),
            _ => Err(ParsingError::BadChar),
    } }
    fn to_wind(&self) -> Result<Wind, ParsingError> {
        match self {
            'e' => Ok(Wind::East),
            's' => Ok(Wind::South),
            'w' => Ok(Wind::West),
            'n' => Ok(Wind::North),
            _ => Err(ParsingError::BadChar),
    } }
    fn to_suit(&self) -> Result<Suit, ParsingError> {
        match self {
            'p' => Ok(Suit::Pin),
            'm' => Ok(Suit::Man),
            's' => Ok(Suit::Sou),
            _ => Err(ParsingError::BadChar),
    } }
}

impl TileConversions for Vec<Tile> {
    fn make_meld(&self, open: bool) -> Option<Meld> {
        fn pad_to_length(tiles: &Vec<Tile>) -> [Option<Tile>; 4] {
            let mut array: [Option<Tile>; 4] = [None; 4];
            for t in 0..tiles.len() { array[t] = Some(tiles[t]) }
            array
        }
        
        if self.len() == 3 {
            if self.count_occurrences(&self[0]) == 3 {
                return Some(Meld{
                    tiles: pad_to_length(self),
                    is_open: open
                })
            } else {
                let mut tiles = self.clone();
                tiles.sort();

                if let Some(adj) = tiles[0].adjacent_up() {
                    if adj.contains(&tiles[1]) && adj.contains(&tiles[2]) && tiles[1] != tiles[2] {
                        return Some(Meld{
                            tiles: pad_to_length(&tiles),
                            is_open: open
                        })
                    }
                }
            }
        } else if self.len() == 4 && self.count_occurrences(&self[0]) == 4 {
            return Some(Meld{
                tiles: pad_to_length(self),
                is_open: open
            })
        } else { panic!("bad meld length!") }
        return None
    }
    fn make_pair(&self) -> Option<Pair> {
        if self.len() == 2 && self.get(0) == self.get(1) {
            return Some(Pair{tiles: [self[0], self[1]]})
        } None
    }
}

impl TileConversions for [Tile] { // TODO: idk
    fn make_meld(&self, open: bool) -> Option<Meld> {
        self.to_vec().make_meld(open)
    }
    fn make_pair(&self) -> Option<Pair> {
        self.to_vec().make_pair()
    }
}

///////////
// tests //
///////////

mod tests {
    use super::*;
    
    #[test]
    fn test_char_conversions(){
        assert_eq!('r'.to_dragon(), Ok(Dragon::Red));
        assert_eq!('e'.to_wind(), Ok(Wind::East));
        assert_eq!('p'.to_suit(), Ok(Suit::Pin));

        assert_eq!('r'.to_suit(), Err(ParsingError::BadChar));
        assert_eq!('e'.to_dragon(), Err(ParsingError::BadChar));
        assert_eq!('r'.to_wind(), Err(ParsingError::BadChar));
    }

    #[test]
    fn test_string_conversions(){
        assert_eq!("dr".to_tile(), Ok(Tile::Dragon(Dragon::Red)));
        assert_eq!("we".to_tile(), Ok(Tile::Wind(Wind::East)));
        assert_eq!("m7".to_tile(), Ok(Tile::Number{suit: Suit::Man, number: 7, red: false}));

        // redness isn't included in tile equality
        assert!("p5r".to_tile().is_ok_and(|t| if let Tile::Number {red, ..} = t { red } else { false } ));
        assert!("p5".to_tile().is_ok_and(|t| if let Tile::Number {red, ..} = t { !red } else { false } ));

        assert_eq!("we,we".to_tiles(), Ok(vec!["we".to_tile().unwrap(); 2]));
        assert_eq!("we,we,we,we".to_tiles(), Ok(vec!["we".to_tile().unwrap(); 4]));
        assert_eq!("dr,dr,dr".to_tiles(), Ok(vec!["dr".to_tile().unwrap(); 3]));

        assert_eq!("we,we,we,we".to_meld().ok(), Some(Meld{is_open: true, tiles: [Some(Tile::Wind(Wind::East)); 4]}));
        assert_eq!("!we,we,we,we".to_meld().ok(), Some(Meld{is_open: false, tiles: [Some(Tile::Wind(Wind::East)); 4]}));
        assert_eq!("we,we,we".to_meld().ok(), Some(Meld{is_open: true, tiles: ["we".to_tile().ok(), "we".to_tile().ok(), "we".to_tile().ok(), None]}));
        assert_eq!("p1,p2,p3".to_meld().ok(), Some(Meld{is_open: true, tiles: ["p1".to_tile().ok(), "p2".to_tile().ok(), "p3".to_tile().ok(), None]}));

        assert_eq!("we,we,we".to_calls().ok(), Some(vec!["we,we,we".to_meld().unwrap()]));
        assert_eq!("we,we,we|dr,dr,dr".to_calls().ok(), Some(vec!["we,we,we".to_meld().unwrap(), "dr,dr,dr".to_meld().unwrap()]));
    }

    #[test]
    fn test_tile_conversions(){
        assert_eq!("we,we,we,we".to_tiles().unwrap().make_pair(), None);
        assert_eq!("we,wn".to_tiles().unwrap().make_pair(), None);
        assert_eq!("we".to_tiles().unwrap().make_pair(), None);

        assert_eq!("we,we".to_tiles().unwrap().make_pair(), Some(Pair{tiles: ["we".to_tile().unwrap(); 2]}));
        assert_eq!("p5,p5".to_tiles().unwrap().make_pair(), Some(Pair{tiles: ["p5".to_tile().unwrap(); 2]}));

        assert_eq!("we,we,we,we".to_tiles().unwrap().make_meld(true), Some(Meld{tiles: ["we".to_tile().ok(); 4], is_open: true}));
        assert_eq!("we,we,we".to_tiles().unwrap().make_meld(false), Some(Meld{tiles: ["we".to_tile().ok(), "we".to_tile().ok(), "we".to_tile().ok(), None], is_open: false}));
        assert_eq!("p2,p3,p4".to_tiles().unwrap().make_meld(false), Some(Meld{tiles: ["p2".to_tile().ok(), "p3".to_tile().ok(), "p4".to_tile().ok(), None], is_open: false}));
        assert_eq!("p3,p2,p4".to_tiles().unwrap().make_meld(false), Some(Meld{tiles: ["p2".to_tile().ok(), "p3".to_tile().ok(), "p4".to_tile().ok(), None], is_open: false}));   
    }

    #[test]
    fn yaku_from_strings(){
        assert_eq!("riichi".to_yaku().ok(), Some(Yaku::Riichi));
        assert_eq!("riichi,ippatsu".to_yaku_vec().ok(), Some(vec![Yaku::Riichi, Yaku::Ippatsu]));

        // should check for mutually exclusive yaku
        assert_eq!("riichi,ippatsu,nagashimangan".to_yaku_vec().ok(), Some(vec![Yaku::NagashiMangan]));
        assert_eq!("chiho,robbedkan".to_yaku_vec().ok(), Some(vec![Yaku::Chiho]));
        assert_eq!("robbedkan,chiho".to_yaku_vec().ok(), Some(vec![Yaku::Chiho]));
    }
}