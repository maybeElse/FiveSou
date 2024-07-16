use crate::errors::errors::{ScoringError, ParsingError, ValueError};
use crate::yaku::{Yaku, WinType, YAKUMAN, YakuHelpers};
use crate::tiles::{Tile, Dragon, Wind, Suit, TileHelpers};
use crate::hand::{Hand, FullHand, HandHelpers, Meld, MeldHelpers, HandTools, MeldOrPair, SequenceHelpers};
use crate::rulesets::{RiichiRuleset, RuleVariations};

#[derive(Debug, PartialEq)]
pub enum Payment{
    DealerTsumo(i32),
    Tsumo {
        dealer: i32,
        non_dealer: i32
    },
    Ron(i32)
}

pub fn count_han(
    yaku_vec: &Vec<Yaku>,
    dora: i8,
    closed: bool,
    ruleset: RiichiRuleset
) -> i8 {
    let mut han_count: i8 = dora;
    let mut has_yakuman: bool = false;

    for yaku in yaku_vec {
        // safety rails for han counting around yakuman
        if !has_yakuman || YAKUMAN.contains(yaku) {
            match yaku {
                // special criteria
                Yaku::Chiitoi => han_count += 2,

                // based on luck
                Yaku::ClosedTsumo => han_count += 1,

                // based on sequences
                Yaku::Pinfu => han_count += if closed || ruleset.allows_open_tanyao() { 1 } else { 0 },
                Yaku::Ipeiko => han_count += 1,
                Yaku::Sanshoku | Yaku::Ittsuu => han_count += if closed { 2 } else { 1 },
                Yaku::Ryanpeiko => han_count += 3,

                // based on triplets/quads
                Yaku::Toitoi | Yaku::Sananko | Yaku::SanshokuDouko | Yaku::Sankantsu
                    => han_count += 2,

                // based on terminal/honor
                Yaku::Tanyao => han_count += 1,
                Yaku::Yakuhai(count) => han_count += count,
                Yaku::Chanta => han_count += if closed { 2 } else { 1 },
                Yaku::Junchan => han_count += if closed { 3 } else { 2 },
                Yaku::Honro | Yaku::Shosangen => han_count += 2,

                // based on suits
                Yaku::Honitsu => han_count += if closed { 3 } else { 2 },
                Yaku::Chinitsu => han_count +=  if closed { 6 } else { 5 },

                // special yaku
                Yaku::Riichi | Yaku::UnderRiver | Yaku::UnderSea | Yaku::AfterKan | Yaku::RobbedKan => han_count += 1,
                Yaku::Ippatsu => han_count += if ruleset.allows_ippatsu() { 1 } else { 0 },
                Yaku::DoubleRiichi => han_count += if ruleset.allows_double_riichi() { 2 } else { 1 },
                Yaku::NagashiMangan => (), // this should only happen if nagashi mangan isn't a valid yaku in the active ruleset.

                // yakuman hands
                Yaku::Daisushi | Yaku::Daichiishin | Yaku::SuuankouTanki if ruleset.has_double_yakuman() => { // double yakuman
                    if has_yakuman { han_count += 26; } else { has_yakuman = true; han_count = 26 } }
                _ if YAKUMAN.contains(&yaku) => { // yakuman
                    if has_yakuman && ruleset.has_yakuman_stacking() { han_count += 13;
                    } else { has_yakuman = true; han_count = 13 } }
                _ => panic!(),
    } } }
    han_count
}

pub fn count_fu(
    full_hand: &FullHand, winning_tile: &Tile, open: bool, yaku: &Vec<Yaku>,
    win_type: WinType, round_wind: Wind, seat_wind: Wind, ruleset: RiichiRuleset
) -> Result<i8, ScoringError> {
    let mut fu: i8 = 20;                                // 20 fu for winning

    if let WinType::Ron = win_type { 
        fu += if !open { 10 }                           // 10 fu for a ron with a closed hand
            else if yaku.contains(&Yaku::Pinfu) { return Ok(30) } // 30 fu total for open pinfu
            else { 0 }
    } else if !yaku.contains(&Yaku::Pinfu) && (!yaku.contains(&Yaku::AfterKan) || ruleset.is_rinshan_tsumo())
            { fu += 2 }                                 // 2 fu for tsumo, but not if it's pinfu; check ruleset for after a kan

    if full_hand.pair.is_dragon() { fu += 2             // 2 fu if the pair is a dragon or the round/seat wind
    } else if let Tile::Wind(w) = full_hand.pair.tile { fu += if w == round_wind && w == seat_wind { ruleset.double_wind_fu()} 
        else if w == round_wind || w == seat_wind { 2 } else { 0 } }

    for meld in full_hand.melds {
        match meld {
            Meld::Triplet {open, tile} => {
                if meld.has_honor() {                   // open honor triplets get 4, closed get 8
                    fu += if open || &tile == winning_tile { 4 } else { 8 }  // calling a tile to finish the meld opens it
                } else if &tile == winning_tile && !open && !full_hand.only_sequences().iter().any(  // this complicates numbered tiles
                    |&x| x.contains_tile(&tile) && x.is_closed()){       // if there's no case which would justify a different wait,
                    if let WinType::Ron = win_type { // (and a ron opened part of the hand
                        fu += if meld.has_terminal() { 4 } else { 2 }   // then treat the meld as if it's open.
                    } else {  fu += if meld.has_terminal() { 8 } else { 4 } }   // ... but a tsumo doesn't open the meld.
                } else {
                    fu += if meld.has_terminal() { if open { 4 } else { 8 } // open terminal triplets get 4, closed get 8
                        } else { if open { 2 } else { 4 } }                 // open simple triplets get 2, closed get 4
            } } 
            Meld::Kan {open, ..} => fu += if meld.has_honor() || meld.has_terminal() {
                if open { 16 } else { 32 }              // open honor/terminal kans get 16, closed get 32
            } else { if open { 8 } else { 16 } },       // open simple kans get 8, closed get 16
            Meld::Sequence {..} => (),                  // sequences get nothing
    } }

    for meld in full_hand.only_closed() {
        match meld {                                    // 2 fu for certain wait shapes:
            MeldOrPair::Pair(p) => if p.contains_tile(winning_tile) { fu += 2; break }, // pair wait
            MeldOrPair::Meld(m) => { match m {
                Meld::Sequence {tiles, ..} => { if m.contains_tile(winning_tile) { 
                    if m.is_middle(winning_tile) { fu += 2; break 
                    } else if m.has_terminal() && !winning_tile.is_terminal() { fu += 2; break }
                } }, // through-shot waits (ie 24 waiting on 3) and waits against a terminal (ie 12 waiting on 3)
                _ => (),
    } } } }
    Ok( round_tens(fu) ) // round up to nearest 10
}

pub fn calc_base_points( han: i8, fu: i8, yaku: &Vec<Yaku>, ruleset: RiichiRuleset ) -> Result<i32, ScoringError> {
    if han < 0 || fu < 20 {
        return Err(ScoringError::ValueError(ValueError::BadInput))
    } else {
        match han {
            0 => return Err(ScoringError::NoYaku),
            1 ..= 4 => {
                let bp: i32 = (fu as i32) * (2_i32.pow(2 + (han as u32)));
                if bp > 2000 { Ok(2000)
                } else if bp == 1920 && ruleset.has_kiriage_mangan(){ Ok(2000)
                } else { Ok(bp) } },
            5 => Ok(2000),          // Mangan
            6 | 7 => Ok(3000),      // Haneman
            8 | 9 | 10 => Ok(4000), // Baiman
            11 | 12 => Ok(6000),    // Sanbaiman
            _ if han > 12 => {      // Yakuman and Kazoe Yakuman
                if yaku.contains_any(&YAKUMAN.to_vec()) { Ok(8000 * (han as i32 / 13))
                } else { Ok(ruleset.kazoe_yakuman_score()) }},
            _ => panic!("negative han???"),
} } }

pub fn calc_player_split(
    base: i32,
    is_dealer: bool,
    win_type: WinType,
    repeats: i8
) -> Result<Payment, ScoringError> {
    match win_type {
        WinType::Tsumo => {
            if is_dealer { Ok(Payment::DealerTsumo( round_hundreds(base * 2) )) }
            else { Ok(Payment::Tsumo{dealer: round_hundreds(base * 2), non_dealer: round_hundreds(base) }) }
        }
        WinType::Ron => Ok(Payment::Ron(round_hundreds(base * {if is_dealer {6} else {4}})))
    }
}

fn round_tens(n: i8) -> i8 { round_nonzero(n as i32, 10) as i8 }

fn round_hundreds(n: i32) -> i32 { round_nonzero(n, 100) }

fn round_nonzero(n: i32, p: i32) -> i32 {
    n + {if (n % p) > 0 { p - (n % p) } else { 0 }}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hand::compose_hand;
    use crate::tiles::{Tile, Dragon, Wind, Suit, TileHelpers, make_tiles_from_string};

    #[test]
    fn han_counts(){
        assert_eq!(count_han(&vec![Yaku::Chiitoi, Yaku::Riichi], 0, true, RiichiRuleset::MajSoul), 3);
        assert_eq!(count_han(&vec![Yaku::Chinroto, Yaku::Riichi], 0, true, RiichiRuleset::MajSoul), 13);
        assert_eq!(count_han(&vec![Yaku::Chinroto, Yaku::Honitsu, Yaku::Daisangen, Yaku::Riichi], 0, true, RiichiRuleset::MajSoul), 26);
        assert_eq!(count_han(&vec![Yaku::Daisushi, Yaku::Riichi], 0, true, RiichiRuleset::MajSoul), 26);
    }

    #[test]
    fn base_point_calc(){
        assert_eq!(calc_base_points(1, 50, &vec![], RiichiRuleset::Default).unwrap(), 400);
        assert_eq!(calc_base_points(2, 40, &vec![], RiichiRuleset::Default).unwrap(), 640);
        assert_eq!(calc_base_points(3, 70, &vec![], RiichiRuleset::Default).unwrap(), 2000);
        assert_eq!(calc_base_points(4, 40, &vec![], RiichiRuleset::Default).unwrap(), 2000);
        assert_eq!(calc_base_points(7, 30, &vec![], RiichiRuleset::Default).unwrap(), 3000);
        assert_eq!(calc_base_points(9, 50, &vec![], RiichiRuleset::Default).unwrap(), 4000);
        assert_eq!(calc_base_points(11, 40, &vec![], RiichiRuleset::Default).unwrap(), 6000);
        assert_eq!(calc_base_points(13, 50, &vec![], RiichiRuleset::Default).unwrap(), 6000);
        assert_eq!(calc_base_points(13, 50, &vec![], RiichiRuleset::MajSoul).unwrap(), 8000);
        
        assert_eq!(calc_base_points(0, 50, &vec![], RiichiRuleset::Default), Err(ScoringError::NoYaku));
        assert_eq!(calc_base_points(0, 10, &vec![], RiichiRuleset::Default), Err(ScoringError::ValueError(ValueError::BadInput)));
    }

    #[test]
    fn bp_and_split_calc(){
        assert_eq!(calc_player_split(calc_base_points(4, 40, &vec![], RiichiRuleset::Default).unwrap(), false, WinType::Tsumo, 0).unwrap(),
                    Payment::Tsumo{dealer: 4000, non_dealer: 2000});
        assert_eq!(calc_player_split(calc_base_points(2, 50, &vec![], RiichiRuleset::Default).unwrap(), true, WinType::Tsumo, 0).unwrap(),
                    Payment::DealerTsumo(1600));
        assert_eq!(calc_player_split(calc_base_points(3, 70, &vec![], RiichiRuleset::Default).unwrap(), true, WinType::Ron, 0).unwrap(),
                    Payment::Ron(12000));
    }
}