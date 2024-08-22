use crate::state::{Win, TileType, WinType, GameState, SeatState};
use crate::errors::errors::{HandError};
use crate::yaku::{Yaku, YAKUMAN, YakuHelpers};
use crate::tiles::{Tile, Dragon, Wind, Suit, TileIs, TileRelations};
use crate::hand::{Hand, HandShape, Meld, Pair, MeldHas, MeldIs, PairTrait, MeldVecHas};
use crate::rulesets::{RiichiRuleset, RuleVariations};

///////////////////////
// structs and enums //
///////////////////////

#[derive(Debug, PartialEq, Clone)]
pub enum Payment{
    DealerTsumo(u32),
    Tsumo {
        dealer: u32,
        non_dealer: u32
    },
    Ron(u32)
}

////////////
// traits //
////////////

pub trait HandScore {
    fn base_points(&self) -> Result<u32, HandError>;
    fn payment_split(&self) -> Result<Payment, HandError>;
}

pub trait CountFu {
    fn fu(&self, game_state: &GameState, seat_state: &SeatState, yaku: &Vec<Yaku>) -> Result<u8, HandError>;
}

pub trait CountHan {
    fn han(&self, is_open: bool, ruleset: RiichiRuleset) -> u8;
}

/////////////////////
// implementations //
/////////////////////

impl CountFu for HandShape {
    fn fu(&self, game_state: &GameState, seat_state: &SeatState, yaku: &Vec<Yaku>) -> Result<u8, HandError> {
        match self {
            HandShape::Standard {melds, pair} => {
                let mut fu: u8 = 20;    // 20 fu for winning
                let winning_tile: Tile = seat_state.latest_tile.unwrap();
    
                if let Some(tile_type) = seat_state.latest_type {
                    match tile_type {
                        TileType::Call if yaku.contains(&Yaku::Pinfu) => return Ok(30), // 30 fu total for open pinfu
                        TileType::Call if melds.iter().all(|m| !m.is_open ) => fu += 10,// 10 fu for a ron with a closed han
                        TileType::Draw if yaku.contains(&Yaku::Pinfu) => return Ok(20), // 20 fu for closed pinfu
                        TileType::Draw => fu += 2, // closed pinfu is already considered, so a tsumo gains 2 fu.
                        TileType::Kan if game_state.ruleset.is_rinshan_tsumo() => fu += 2, // rinshan fu rule
                        _ => (),
                    }
                } else { panic!("latest_type set to none during win?") }
    
                if pair.is_dragon() { fu += 2 } // 2 fu if the pair is a dragon or the round/seat wind
                else if let Tile::Wind(wind) = pair.tile() {
                    if wind == game_state.round_wind && wind == seat_state.seat_wind { fu += game_state.ruleset.double_wind_fu() }
                    else if wind == game_state.round_wind || wind == seat_state.seat_wind { fu += 2 }
                }
    
                if pair.tile() == winning_tile { fu += 2 } // 2 fu for a pair wait
    
                for meld in melds {
                    if meld.is_seq() && !meld.is_open && meld.contains(&winning_tile) {
                        // sequences only get fu for middle waits and single-sided waits (ie 1,2 waiting on 3)
                        if meld.tiles[1].is_some_and(|t| t == winning_tile) && pair.tile() != winning_tile { fu += 2 }
                        else if meld.has_terminal() && !winning_tile.is_terminal() { fu += 2 }
                    } else if meld.is_trip() {
                        if meld.is_open { fu += meld.base_fu() }
                        else if let Some(tile_type) = seat_state.latest_type { // check whether the meld was opened by the winning call
                            match tile_type {
                                TileType::Call => {
                                    if meld.contains(&winning_tile) { // check if the win could have opened it.
                                        if meld.is_quad() { fu += meld.base_fu() } // (I don't think this can happen, but if it did)
                                        else { // check if there are any sequences which could have been the wait instead.
                                            if melds.iter().filter(|m| m.is_seq()).any(|m| !m.is_open && m.contains(&winning_tile)) {
                                                fu += meld.base_fu() * 2    // if so, it should have stayed closed
                                                // TODO: are there any yaku we need to check for?
                                            } else { fu += meld.base_fu() } // otherwise, it was opened.
                                        }
                                    }
                                    else { fu += meld.base_fu() * 2 }
                                }
                                _ => fu += meld.base_fu() * 2,
                            }
                        }
                    } else if meld.is_quad() {
                        if meld.is_open { fu += meld.base_fu() }
                        else { fu += meld.base_fu() * 2 }
                    }
                }
    
                Ok( fu.round_to_tens() ) // round up to nearest 10
            },
            HandShape::Chiitoi {pairs} => return Ok(25),
            HandShape::Kokushi(_) => return Ok(20), // Kokushi doesn't have fu, so this doesn't matter.
            _ => return Err(HandError::NotAgari) // won't calculate overall fu for an incomplete hand.
        }   
    }
}

impl CountHan for Vec<Yaku> {
    fn han(&self, is_open: bool, ruleset: RiichiRuleset) -> u8 {
        let mut has_yakuman: bool = false;
        self.iter().fold(0, |han_count, y|{
            if !has_yakuman || YAKUMAN.contains(y) {
                match y {
                    // special criteria
                    Yaku::Chiitoi => han_count + 2,
    
                    // based on luck
                    Yaku::ClosedTsumo => han_count + 1,
    
                    // based on sequences
                    Yaku::Pinfu => if !is_open || ruleset.allows_open_tanyao() { han_count + 1 } else { han_count },
                    Yaku::Ipeiko => han_count + 1,
                    Yaku::SanshokuDoujun | Yaku::Ittsuu => han_count + if is_open { 1 } else { 2 },
                    Yaku::Ryanpeiko => han_count + 3,
    
                    // based on triplets/quads
                    Yaku::Toitoi | Yaku::Sananko | Yaku::SanshokuDouko | Yaku::Sankantsu => han_count + 2,
    
                    // based on terminal/honor
                    Yaku::Tanyao => han_count + 1,
                    Yaku::Yakuhai(count) => han_count + count,
                    Yaku::Chanta => han_count + if is_open { 1 } else { 2 },
                    Yaku::Junchan => han_count + if is_open { 2 } else { 3 },
                    Yaku::Honro | Yaku::Shosangen => han_count + 2,
    
                    // based on suits
                    Yaku::Honitsu => han_count + if is_open { 2 } else { 3 },
                    Yaku::Chinitsu => han_count + if is_open { 5 } else { 6 },
    
                    // special yaku
                    Yaku::Riichi | Yaku::UnderRiver | Yaku::UnderSea | Yaku::AfterKan | Yaku::RobbedKan => han_count + 1,
                    Yaku::Ippatsu => if ruleset.allows_ippatsu() { han_count + 1 } else { han_count },
                    Yaku::DoubleRiichi => han_count + if ruleset.allows_double_riichi() { 2 } else { 1 },
                    Yaku::NagashiMangan => han_count, // this should only happen if nagashi mangan isn't a valid yaku in the active ruleset.
    
                    // yakuman hands
                    Yaku::Daisushi | Yaku::Daichiishin | Yaku::SuuankouTanki if ruleset.has_double_yakuman() => { // double yakuman
                        if has_yakuman { han_count + 26 } else { has_yakuman = true; 26 } }
                    _ if YAKUMAN.contains(&y) => { // yakuman
                        if has_yakuman && ruleset.has_yakuman_stacking() { han_count + 13
                        } else { has_yakuman = true; 13 } }
                    _ => panic!(),
                }
            } else { han_count  }
        })
    }
}

///////////////
// functions //
///////////////

pub fn calc_base_points( han: u8, fu: u8, yaku: &Vec<Yaku>, ruleset: RiichiRuleset ) -> Result<u32, HandError> {
    if han < 0 || fu < 20 {
        return Err(HandError::ValueError)
    } else {
        match han {
            0 => return Err(HandError::NoYaku),
            1 ..= 4 => {
                let bp: u32 = (fu as u32) * (2_u32.pow(2 + (han as u32)));
                if bp > 2000 { Ok(2000)
                } else if bp == 1920 && ruleset.has_kiriage_mangan(){ Ok(2000)
                } else { Ok(bp) } },
            5 => Ok(2000),          // Mangan
            6 | 7 => Ok(3000),      // Haneman
            8 | 9 | 10 => Ok(4000), // Baiman
            11 | 12 => Ok(6000),    // Sanbaiman
            _ if han > 12 => {      // Yakuman and Kazoe Yakuman
                if yaku.contains_any(&YAKUMAN.to_vec()) { Ok(8000 * u32::from(han / 13))
                } else { Ok(ruleset.kazoe_yakuman_score()) }},
            _ => panic!("negative han???"),
} } }

pub fn calc_player_split(
    base: u32,
    is_dealer: bool,
    win_type: WinType,
    repeats: u8
) -> Result<Payment, HandError> {
    match win_type {
        WinType::Tsumo => {
            if is_dealer { Ok(Payment::DealerTsumo( (base * 2).round_to_hundreds() )) }
            else { Ok(Payment::Tsumo{dealer: (base * 2).round_to_hundreds(), non_dealer: base.round_to_hundreds() }) }
        }
        WinType::Ron => Ok(Payment::Ron( (base * {if is_dealer {6} else {4}}).round_to_hundreds() ))
    }
}

pub trait ScoreRounding {
    fn round_to_tens(&self) -> Self;
    fn round_to_hundreds(&self) -> Self;
}

macro_rules! impl_ScoreRounding {
    (for $($t:ty),+) => {
        $(impl ScoreRounding for $t {
            fn round_to_tens(&self) -> $t {
                self + self % 10
            }
            fn round_to_hundreds(&self) -> $t {
                self + self % 100
            }
        })*
    }
}

impl_ScoreRounding!(for u8, u32);

///////////
// tests //
///////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn han_counts(){
        assert_eq!(vec![Yaku::Chiitoi, Yaku::Riichi].han(true, RiichiRuleset::MajSoul), 3);
        assert_eq!(vec![Yaku::Chinroto, Yaku::Riichi].han(true, RiichiRuleset::MajSoul), 13);
        assert_eq!(vec![Yaku::Chinroto, Yaku::Honitsu, Yaku::Daisangen, Yaku::Riichi].han(true, RiichiRuleset::MajSoul), 26);
        assert_eq!(vec![Yaku::Daisushi, Yaku::Riichi].han(true, RiichiRuleset::MajSoul), 26);
    }

    #[test]
    fn base_point_calc(){
        assert_eq!(calc_base_points(1, 50, &Vec::new(), RiichiRuleset::Default).unwrap(), 400);
        assert_eq!(calc_base_points(2, 40, &Vec::new(), RiichiRuleset::Default).unwrap(), 640);
        assert_eq!(calc_base_points(3, 70, &Vec::new(), RiichiRuleset::Default).unwrap(), 2000);
        assert_eq!(calc_base_points(4, 40, &Vec::new(), RiichiRuleset::Default).unwrap(), 2000);
        assert_eq!(calc_base_points(7, 30, &Vec::new(), RiichiRuleset::Default).unwrap(), 3000);
        assert_eq!(calc_base_points(9, 50, &Vec::new(), RiichiRuleset::Default).unwrap(), 4000);
        assert_eq!(calc_base_points(11, 40, &Vec::new(), RiichiRuleset::Default).unwrap(), 6000);
        assert_eq!(calc_base_points(13, 50, &Vec::new(), RiichiRuleset::Default).unwrap(), 6000);
        assert_eq!(calc_base_points(13, 50, &Vec::new(), RiichiRuleset::MajSoul).unwrap(), 8000);
        
        assert_eq!(calc_base_points(0, 50, &Vec::new(), RiichiRuleset::Default), Err(HandError::NoYaku));
        assert_eq!(calc_base_points(0, 10, &Vec::new(), RiichiRuleset::Default), Err(HandError::ValueError));
    }

    #[test]
    fn bp_and_split_calc(){
        assert_eq!(calc_player_split(calc_base_points(4, 40, &Vec::new(), RiichiRuleset::Default).unwrap(), false, WinType::Tsumo, 0).unwrap(),
                    Payment::Tsumo{dealer: 4000, non_dealer: 2000});
        assert_eq!(calc_player_split(calc_base_points(2, 50, &Vec::new(), RiichiRuleset::Default).unwrap(), true, WinType::Tsumo, 0).unwrap(),
                    Payment::DealerTsumo(1600));
        assert_eq!(calc_player_split(calc_base_points(3, 70, &Vec::new(), RiichiRuleset::Default).unwrap(), true, WinType::Ron, 0).unwrap(),
                    Payment::Ron(12000));
    }
}