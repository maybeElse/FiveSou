#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod tiles;
pub mod yaku;
mod errors;
pub mod scoring;
pub mod hand;
pub mod rulesets;
pub mod state;
pub mod conversions;

use crate::tiles::{Tile, Wind};
use crate::conversions::{StringConversions, CharConversions};
use crate::hand::{Meld, Hand, HandTrait};
use crate::state::{GameState, SeatState, InferWin};
use crate::yaku::{Yaku};
use crate::scoring::{Payment, calc_base_points, calc_player_split};
use crate::errors::errors::{HandError};
use crate::rulesets::{RiichiRuleset, RuleVariations};
use std::io;

pub fn score_hand_from_str( // note: this depends on specificly formatted input, so it may be a bit fragile
                        // hand::new() etc should be preferred whenever possible,
                        // but this massively simplifies writing unit tests
    closed_tiles: &str,     // comma-separated tiles, ie "p1,p2,p3"
    called_tiles: &str,     // comma *and* pipe separated, with closed kans additionally enclosed in '()', ie "dw,dw,dw|m1,m2,m3|(p5,p5,p5,p5r)"
    latest_tile: &str,      // single tile, ie "m5r"
    seat_wind: char,        // single char, ie 'e' = east
    round_wind: char,
    latest_type: char,      // 'd'raw, 'c'all, or 'k'an
    dora_markers: &str,     // comma-separated tiles, ie "dw,wn"
    ura_markers: &str,
    special_yaku: &str,     // comma-separated special yaku names, ie "riichi,ippatsu"
    repeat_counts: u8,      // number of repeat counters on the table
    ruleset: &str           // which ruleset to use
) -> Result<Payment, HandError> {
    let game_state: GameState = GameState {
        ruleset: ruleset.to_ruleset().unwrap(),
        round_wind: round_wind.to_wind().unwrap(),
        dora_markers: dora_markers.to_tiles().ok(),
        ura_dora_markers: None,
        repeats: repeat_counts,
    };
    let seat_state: SeatState = SeatState {
        closed_tiles: closed_tiles.to_tiles().unwrap(),
        called_melds: called_tiles.to_calls().ok(),
        seat_wind: seat_wind.to_wind().unwrap(),
        latest_tile: latest_tile.to_tile().ok(),
        latest_type: latest_type.to_tile_type().ok(),
        special_yaku: special_yaku.to_yaku_vec().ok(),
    };

    let hand: Hand = Hand::new(game_state.clone(), seat_state.clone());

    calc_player_split(
        calc_base_points(hand.han() + hand.dora(), hand.fu(), hand.yaku(), game_state.ruleset).expect(""),
        hand.is_dealer(), seat_state.latest_type.expect("").as_win(), game_state.repeats
    )
}

// pub fn score_hand_from_structs(
//     closed_tiles: Vec<Tile>,
//     called_tiles: Option<Vec<Meld>>,
//     winning_tile: Tile,
//     seat_wind: Wind,
//     round_wind: Wind,
//     win_type: WinType,
//     dora_markers: Option<Vec<Tile>>,
//     special_yaku: Option<Vec<Yaku>>,
//     repeats: i8,
//     ruleset: RiichiRuleset
// ) -> Result<Payment, HandError> {
//     let hand: Hand = hand::compose_hand(
//         closed_tiles, called_tiles, winning_tile, win_type, seat_wind, round_wind, 
//         special_yaku.clone(), dora_markers, ruleset
//     )?;
//     if special_yaku.unwrap_or_default().contains(&Yaku::NagashiMangan) && ruleset.allows_nagashi_mangan() {
//         scoring::calc_player_split(2000, seat_wind == Wind::East, WinType::Tsumo, repeats)
//     } else {
//         scoring::calc_player_split(
//             scoring::calc_base_points(
//                 hand.get_han(),
//                 hand.get_fu(),
//                 &hand.get_yaku(),
//                 ruleset
//             )?, seat_wind == Wind::East, win_type, repeats
//         )
//     }
// }

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