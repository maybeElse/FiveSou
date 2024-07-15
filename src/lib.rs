#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod tiles;
pub mod yaku;
mod errors;
pub mod scoring;
pub mod hand;
pub mod rulesets;

use crate::tiles::{Tile, Wind, MakeTile};
use crate::hand::{Meld, Hand, HandTools};
use crate::yaku::{WinType, YakuHelpers, Yaku};
use crate::scoring::{Payment};
use crate::errors::errors::{ScoringError};
use crate::rulesets::{RiichiRuleset, RuleVariations, FromString};
use std::io;

pub fn score_hand_from_str( // note: this depends on specificly formatted input, so it may be a bit fragile
                        // score_hand_from_structs() should be preferred whenever possible,
                        // but this massively simplifies writing unit tests
    closed_tiles: &str,     // comma-separated tiles, ie "p1,p2,p3"
    called_tiles: &str,     // comma *and* pipe separated, with closed kans additionally enclosed in '()', ie "dw,dw,dw|m1,m2,m3|(p5,p5,p5,p5r)"
    winning_tile: &str,     // single tile, ie "m5r"
    seat_wind: char,        // single char, ie 'e' = east
    round_wind: char,
    win_type: char,         // 'r'on or 't'sumo
    dora_markers: &str,     // comma-separated tiles, ie "dw,wn"
    special_yaku: &str,     // comma-separated special yaku names, ie "riichi,ippatsu"
    repeat_counts: usize,   // number of repeat counters on the table
    ruleset: &str           // which ruleset to use
) -> Result<Payment, ScoringError> {
    let dora: Vec<Tile> = tiles::make_tiles_from_string(dora_markers).unwrap_or_default();
    let sp_yaku: Vec<Yaku> = Vec::<Yaku>::from_string(special_yaku).unwrap_or_default();

    score_hand_from_structs(
        tiles::make_tiles_from_string(&[closed_tiles, winning_tile].join(",")).unwrap(),
        hand::make_melds_from_string(called_tiles, true),
        Tile::from_string(winning_tile).unwrap(),
        Wind::from_char(seat_wind).unwrap(),
        Wind::from_char(round_wind).unwrap(),
        { match win_type {
            'r' => WinType::Ron,
            't' => WinType::Tsumo,
            _ => panic!() }},
        if dora.len() > 0 { Some(dora) } else { None },
        if sp_yaku.len() > 0 { Some(sp_yaku) } else { None },
        repeat_counts as i8,
        RiichiRuleset::from_string(ruleset)
    )
}

pub fn score_hand_from_structs(
    closed_tiles: Vec<Tile>,
    called_tiles: Option<Vec<Meld>>,
    winning_tile: Tile,
    seat_wind: Wind,
    round_wind: Wind,
    win_type: WinType,
    dora_markers: Option<Vec<Tile>>,
    special_yaku: Option<Vec<Yaku>>,
    repeats: i8,
    ruleset: RiichiRuleset
) -> Result<Payment, ScoringError> {
    let hand: Hand = hand::compose_hand(
        closed_tiles, called_tiles, winning_tile, win_type, seat_wind, round_wind, 
        special_yaku.clone(), dora_markers, ruleset
    )?;
    if special_yaku.unwrap_or_default().contains(&Yaku::NagashiMangan) && ruleset.allows_nagashi_mangan() {
        scoring::calc_player_split(2000, seat_wind == Wind::East, WinType::Tsumo, repeats)
    } else {
        scoring::calc_player_split(
            scoring::calc_base_points(
                hand.get_han(),
                hand.get_fu(),
                &hand.get_yaku(),
                ruleset
            )?, seat_wind == Wind::East, win_type, repeats
        )
    }
}

// fn human_readable_scoring(
//     closed_tiles: Vec<Tile>,
//     called_tiles: Option<Vec<Meld>>,
//     winning_tile: Tile,
//     seat_wind: Wind,
//     round_wind: Wind,
//     win_type: WinType,
//     dora_markers: Vec<Tile>,
//     special_yaku: Vec<YakuSpecial>,
//     repeats: i8
// ) -> String {

// }

#[cfg(test)]
mod tests {
    use super::*;
}