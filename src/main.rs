#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod tiles;
mod yaku;
mod errors;
mod scoring;
mod hand;
mod rulesets;

use crate::tiles::{Tile, Wind, FromString, FromChar};
use crate::hand::{Meld, Hand, HandTools};
use crate::yaku::{WinType, YakuHelpers, Yaku};
use crate::scoring::{Payment};
use crate::errors::errors::{ScoringError};
use std::io;

fn main() {
    println!("\
\t+----------------------------------------+
\t| Hello [generic riichi mahjong player]! |
\t+----------------------------------------+");
}

fn score_hand_from_str( // note: this depends on specificly formatted input, so it may be a bit fragile
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
        repeat_counts as i8
    )
}

fn score_hand_from_structs(
    closed_tiles: Vec<Tile>,
    called_tiles: Option<Vec<Meld>>,
    winning_tile: Tile,
    seat_wind: Wind,
    round_wind: Wind,
    win_type: WinType,
    dora_markers: Option<Vec<Tile>>,
    special_yaku: Option<Vec<Yaku>>,
    repeats: i8
) -> Result<Payment, ScoringError> {
    let hand: Hand = hand::compose_hand(
        closed_tiles, called_tiles, winning_tile, win_type, seat_wind, round_wind, 
        special_yaku.clone(), dora_markers
    )?;
    if special_yaku.unwrap_or_default().contains(&Yaku::NagashiMangan) {
        scoring::calc_player_split(2000, seat_wind == Wind::East, WinType::Tsumo, repeats)
    } else {
        scoring::calc_player_split(
            scoring::calc_base_points(
                hand.get_han(),
                hand.get_fu()
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
    use crate::scoring::Payment;

    #[test]
    fn jpml_pro_test_2022(){
        // test cases sourced from the 2022 JPML pro test:
        // https://cloudymahjong.com/wp-content/uploads/2023/12/JPML-Pro-Test-Questions-2022.pdf
        // https://cloudymahjong.com/wp-content/uploads/2023/12/JPML-Pro-Test-Answers-2022.pdf
        // while it doesn't hit everything that needs to be tested, it's full of tricky edge cases

        // #1
        assert_eq!(score_hand_from_str("p1,p2,p3,p4,p4,p4,p5,p6,p7,p8,s2,s3,s4", "", "p9", 'e', 'e', 't', "", "", 0), Ok(Payment::DealerTsumo(2600)));
        assert_eq!(score_hand_from_str("p1,p2,p3,p4,p4,p4,p5,p6,p7,p8,s2,s3,s4", "", "p9", 'e', 'e', 'r', "", "", 0), Ok(Payment::Ron(5800)));
        assert_eq!(score_hand_from_str("p1,p2,p3,p4,p4,p4,p5,p6,p7,p8,s2,s3,s4", "", "p9", 's', 'e', 't', "", "", 0), Ok(Payment::Tsumo{dealer: 2600, non_dealer: 1300}));
        assert_eq!(score_hand_from_str("p1,p2,p3,p4,p4,p4,p5,p6,p7,p8,s2,s3,s4", "", "p9", 's', 'e', 'r', "", "", 0), Ok(Payment::Ron(3900)));

        // #2
        assert_eq!(score_hand_from_str("m2,m2,m3,m3,p3,p3,p5,p5,s6,s6,s7,s8,s8", "", "s7", 'e', 'e', 't', "", "", 0), Ok(Payment::DealerTsumo(3200)));
        assert_eq!(score_hand_from_str("m2,m2,m3,m3,p3,p3,p5,p5,s6,s6,s7,s8,s8", "", "s7", 'e', 'e', 'r', "", "", 0), Ok(Payment::Ron(4800)));
        assert_eq!(score_hand_from_str("m2,m2,m3,m3,p3,p3,p5,p5,s6,s6,s7,s8,s8", "", "s7", 's', 'e', 't', "", "", 0), Ok(Payment::Tsumo{dealer: 3200, non_dealer: 1600}));
        assert_eq!(score_hand_from_str("m2,m2,m3,m3,p3,p3,p5,p5,s6,s6,s7,s8,s8", "", "s7", 's', 'e', 'r', "", "", 0), Ok(Payment::Ron(3200)));

        // #3
        assert_eq!(score_hand_from_str("m3,m5,m6,m7,m8,m8,m8", "p8,p8,p8|m2,m2,m2", "m3", 'e', 'e', 't', "", "", 0), Ok(Payment::DealerTsumo(700)));
        assert_eq!(score_hand_from_str("m3,m5,m6,m7,m8,m8,m8", "p8,p8,p8|m2,m2,m2", "m3", 'e', 'e', 'r', "", "", 0), Ok(Payment::Ron(1500)));
        assert_eq!(score_hand_from_str("m3,m5,m6,m7,m8,m8,m8", "p8,p8,p8|m2,m2,m2", "m3", 's', 'e', 't', "", "", 0), Ok(Payment::Tsumo{dealer: 700, non_dealer: 400}));
        assert_eq!(score_hand_from_str("m3,m5,m6,m7,m8,m8,m8", "p8,p8,p8|m2,m2,m2", "m3", 's', 'e', 'r', "", "", 0), Ok(Payment::Ron(1000)));

        // #4
        assert_eq!(score_hand_from_str("p2,p2,we,we", "m8,m8,m8|p3,p3,p3|s8,s8,s8", "p2", 'e', 'e', 't', "", "", 0), Ok(Payment::DealerTsumo(1300)));
        assert_eq!(score_hand_from_str("p2,p2,we,we", "m8,m8,m8|p3,p3,p3|s8,s8,s8", "p2", 'e', 'e', 'r', "", "", 0), Ok(Payment::Ron(3900)));
        assert_eq!(score_hand_from_str("p2,p2,we,we", "m8,m8,m8|p3,p3,p3|s8,s8,s8", "p2", 's', 'e', 't', "", "", 0), Ok(Payment::Tsumo{dealer: 1300, non_dealer: 700}));
        assert_eq!(score_hand_from_str("p2,p2,we,we", "m8,m8,m8|p3,p3,p3|s8,s8,s8", "p2", 's', 'e', 'r', "", "", 0), Ok(Payment::Ron(2000)));

        // #5
        assert_eq!(score_hand_from_str("p2,p3,p4,p5,p6,p7,p7,p7,we,we", "ws,ws,ws", "p1", 'e', 'e', 't', "", "", 0), Ok(Payment::DealerTsumo(1300)));
        assert_eq!(score_hand_from_str("p2,p3,p4,p5,p6,p7,p7,p7,we,we", "ws,ws,ws", "p1", 'e', 'e', 'r', "", "", 0), Ok(Payment::Ron(3900)));
        assert_eq!(score_hand_from_str("p2,p3,p4,p5,p6,p7,p7,p7,we,we", "ws,ws,ws", "p1", 's', 'e', 't', "", "", 0), Ok(Payment::Tsumo{dealer: 2600, non_dealer: 1300}));
        assert_eq!(score_hand_from_str("p2,p3,p4,p5,p6,p7,p7,p7,we,we", "ws,ws,ws", "p1", 's', 'e', 'r', "", "", 0), Ok(Payment::Ron(3900)));

        // #6
        assert_eq!(score_hand_from_str("p3,p3,p4,p4,p5,p5,p2", "s8,s8,s8|!s7,s7,s7,s7", "p2", 'e', 'e', 't', "", "", 0), Ok(Payment::DealerTsumo(800)));
        assert_eq!(score_hand_from_str("p3,p3,p4,p4,p5,p5,p2", "s8,s8,s8|!s7,s7,s7,s7", "p2", 'e', 'e', 'r', "", "", 0), Ok(Payment::Ron(2000)));
        assert_eq!(score_hand_from_str("p3,p3,p4,p4,p5,p5,p2", "s8,s8,s8|!s7,s7,s7,s7", "p2", 's', 'e', 't', "", "", 0), Ok(Payment::Tsumo{dealer: 800, non_dealer: 400}));
        assert_eq!(score_hand_from_str("p3,p3,p4,p4,p5,p5,p2", "s8,s8,s8|!s7,s7,s7,s7", "p2", 's', 'e', 'r', "", "", 0), Ok(Payment::Ron(1300)));

        // #7
        assert_eq!(score_hand_from_str("m2,m2,m4,m4,m3,s7,s7,s7,ws,ws", "!wn,wn,wn,wn", "m3", 'e', 'e', 't', "", "", 0), Ok(Payment::DealerTsumo(2000)));
        assert_eq!(score_hand_from_str("m2,m2,m4,m4,m3,s7,s7,s7,ws,ws", "!wn,wn,wn,wn", "m3", 'e', 'e', 'r', "", "", 0), Ok(Payment::Ron(3400)));
        assert_eq!(score_hand_from_str("m2,m2,m4,m4,m3,s7,s7,s7,ws,ws", "!wn,wn,wn,wn", "m3", 's', 'e', 't', "", "", 0), Ok(Payment::Tsumo{dealer: 2300, non_dealer: 1200}));
        assert_eq!(score_hand_from_str("m2,m2,m4,m4,m3,s7,s7,s7,ws,ws", "!wn,wn,wn,wn", "m3", 's', 'e', 'r', "", "", 0), Ok(Payment::Ron(2300)));

        // #8
        assert_eq!(score_hand_from_str("s1,s1,s1,s2,s4,we,we", "m9,m9,m9|!dr,dr,dr,dr", "s3", 'e', 'e', 't', "", "", 0), Ok(Payment::DealerTsumo(1300)));
        assert_eq!(score_hand_from_str("s1,s1,s1,s2,s4,we,we", "m9,m9,m9|!dr,dr,dr,dr", "s3", 'e', 'e', 'r', "", "", 0), Ok(Payment::Ron(3400)));
        assert_eq!(score_hand_from_str("s1,s1,s1,s2,s4,we,we", "m9,m9,m9|!dr,dr,dr,dr", "s3", 's', 'e', 't', "", "", 0), Ok(Payment::Tsumo{dealer: 1200, non_dealer: 600}));
        assert_eq!(score_hand_from_str("s1,s1,s1,s2,s4,we,we", "m9,m9,m9|!dr,dr,dr,dr", "s3", 's', 'e', 'r', "", "", 0), Ok(Payment::Ron(2300)));

        // #9
        assert_eq!(score_hand_from_str("m7,m8,m9,p7,p8,p8,p8", "!ws,ws,ws,ws|!dg,dg,dg,dg", "p9", 'e', 'e', 't', "", "", 0), Ok(Payment::DealerTsumo(2900)));
        assert_eq!(score_hand_from_str("m7,m8,m9,p7,p8,p8,p8", "!ws,ws,ws,ws|!dg,dg,dg,dg", "p9", 'e', 'e', 'r', "", "", 0), Ok(Payment::Ron(4800)));
        assert_eq!(score_hand_from_str("m7,m8,m9,p7,p8,p8,p8", "!ws,ws,ws,ws|!dg,dg,dg,dg", "p9", 's', 'e', 't', "", "", 0), Ok(Payment::Tsumo{dealer: 4000, non_dealer: 2000}));
        assert_eq!(score_hand_from_str("m7,m8,m9,p7,p8,p8,p8", "!ws,ws,ws,ws|!dg,dg,dg,dg", "p9", 's', 'e', 'r', "", "", 0), Ok(Payment::Ron(6400)));

        // #10
        assert_eq!(score_hand_from_str("m2,m3,m4,m4,m5,m6,m7,s8,s8,s8", "we,we,we,we", "m1", 'e', 'e', 't', "", "rinshan", 0), Ok(Payment::DealerTsumo(2600)));
        assert_eq!(score_hand_from_str("m2,m3,m4,m4,m5,m6,m7,s8,s8,s8", "we,we,we,we", "m1", 'e', 'e', 'r', "", "", 0), Ok(Payment::Ron(3900)));
        assert_eq!(score_hand_from_str("m2,m3,m4,m4,m5,m6,m7,s8,s8,s8", "we,we,we,we", "m1", 's', 'e', 't', "", "rinshan", 0), Ok(Payment::Tsumo{dealer: 1300, non_dealer: 700}));
        assert_eq!(score_hand_from_str("m2,m3,m4,m4,m5,m6,m7,s8,s8,s8", "we,we,we,we", "m1", 's', 'e', 'r', "", "", 0), Ok(Payment::Ron(1300)));
    }
}