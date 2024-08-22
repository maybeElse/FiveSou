use crate::tiles::{Tile, Dragon, Wind, Suit, TileIs, TileVecTrait};
// use crate::hand::{Hand, FullHand, HandHelpers, Meld, MeldHelpers, HandTools, Pair, SequenceHelpers, VecHelpers};
use crate::errors::errors::{HandError, ParsingError};
use crate::rulesets::{RiichiRuleset, RuleVariations};
use crate::state::{GameState, SeatState, TileType, WinType, InferWin, SeatAccess};
use crate::hand::{HandShape, Pair, Meld, MeldIs, MeldHas, MeldVecHas, PairTrait};
use core::fmt;
use std::collections::HashSet;
use itertools::Itertools;

///////////
// enums //
///////////

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Yaku {
    Chiitoi,        // unique shape, fully closed hand          2 han closed
    ClosedTsumo,    // tsumo, fully closed hand                 1 han closed

    // based on sequence
    Pinfu,          // no fu awarded                            1 han closed
    Ipeiko,         // two identical sequences, closed hand     1 han closed
    SanshokuDoujun, // same sequence in each suit               2 han closed / 1 han open
    Ittsuu,         // straight (1-9 in a suit)                 2 han closed / 1 han open
    Ryanpeiko,      // ipeiko twice. replaces ipeiko            3 han closed

    // based on triplets/quads
    Toitoi,         // all triplets                             2 han
    Sananko,        // three concealed triplets                 2 han
    SanshokuDouko,  // same triplet in each suit                2 han
    Sankantsu,      // three quads                              2 han

    // based on terminal/honor
    Tanyao,         // no honor or terminal                     1 han
    Yakuhai(u8),     // triplets or quads of dragons,           1 han per triplet
                    // seat winds, or round winds. Round+Seat wind counts for double.
    Chanta,         // each sequence/meld contains a terminal   2 han closed / 1 han open
                    // or honor tile
    Junchan,        // each sequence/meld contains a terminal   3 han closed / 2 han open
                    // tile; no honor tiles present in hand
    Honro,          // hand contains only terminal and honor    2 han
                    // tiles; always paired with one of
                    // chitoi or toitoi
    Shosangen,      // two triplets/quads of dragons, and a     2 han
                    // pair of the third dragon
    
    // based on suits
    Honitsu,        // half flush. simple tiles from only one   3 han closed / 2 han open
                    // suit, plus honor tiles.
                    // can apply to chitoi hands
    Chinitsu,       // full flush. hand only contains simple    6 han closed / 5 han open
                    // tiles from one suit; no honors.
                    // can apply to chitoi hands

    // yakuman hands
    Kokushi,        // thirteen orphans, single-tile wait       limit           closed
    Suuankou,       // four concealed triplets, double wait     limit           closed
    SuuankouTanki,  // four concealed triplets, single wait     double limit    open or closed
    Daisangen,      // big three dragons                        limit           open or closed
    Shosushi,       // little four winds (3 triplets + pair)    limit           open or closed
    Daisushi,       // big four winds (4 triplets)              double limit    open or closed
    Tsuiso,         // all honors                               limit           open or closed
    Daichiishin,    // all honors (chiitoi variant)             limit           closed
    Chinroto,       // all terminals                            limit           open or closed
    Ryuiso,         // all green (sou 2,3,4,6,8 + green dragon) limit           open or closed
    ChurenPoto,     // nine gates                               limit           closed
    Sukantsu,       // four kans                                limit           open or closed
    
    SpecialWait,    // the yakuman's wait adds additional value limit
                    // ie: four concealed triplets, single wait
                    //     thirteen orphans, thirteen-way wait
                    //     nine gates, nine-way wait
                    // I think that breaking this out into a unique criteria will simplify code somewhat

    // special yaku
    Riichi,         // declared Riichi, fully closed hand       1 han closed
    DoubleRiichi,   // declared Riichi on first turn            2 han closed
    Ippatsu,        // win within one go-around after Riichi    1 han closed
    UnderSea,       // very last tile drawn, tsumo only         1 han
    UnderRiver,     // very last tile discarded, ron only       1 han
    AfterKan,       // win on dead wall draw, tsumo only        1 han
    RobbedKan,      // ron only                                 1 han
    NagashiMangan,  // counts as tsumo, ignores other yaku      automatic mangan

    // special yakuman hands
    Tenho,          // blessing of heaven. tsumo.               limit   closed, dealer only
    Chiho,          // blessing of earth. tsumo.                limit   closed, non-dealer only
}

pub static YAKUMAN: [Yaku; 15] = [Yaku::Kokushi, Yaku::Suuankou, Yaku::SuuankouTanki, Yaku::Daisangen, Yaku::Shosushi,
                                Yaku::Daisushi, Yaku::Tsuiso, Yaku::Chinroto, Yaku::Ryuiso, Yaku::ChurenPoto, Yaku::Sukantsu,
                                Yaku::Daichiishin, Yaku::SpecialWait, Yaku::Tenho, Yaku::Chiho];
pub static YAKU_SPECIAL: [Yaku; 10] = [Yaku::Riichi, Yaku::DoubleRiichi, Yaku::Ippatsu, Yaku::UnderSea, Yaku::UnderRiver,
                                Yaku::AfterKan, Yaku::RobbedKan, Yaku::NagashiMangan, Yaku::Tenho, Yaku::Chiho];

////////////
// traits //
////////////

// some yaku are mutually exclusive; for instance, Ipeiko cannot coexist with Ryanpeiko
// hopefully my yaku-identification logic will handle this, but just in case ...
// see https://riichi.wiki/Yaku_compatibility
pub trait YakuHelpers {
    type T;

    fn push_checked(&mut self, yaku: Self::T);
    fn append_checked(&mut self, yaku: &Vec<Self::T>);
    fn contains_any(&self, yaku: &Self) -> bool;
}

pub trait FindYaku {
    fn yaku(&self, game_state: &GameState, seat_state: &SeatState) -> Vec<Yaku>;
}

/////////////////////
// implementations //
/////////////////////

impl YakuHelpers for Vec<Yaku> {
    type T = Yaku;

    fn push_checked(&mut self, yaku: Yaku) {
        if self.contains_any(&YAKUMAN.to_vec()) && !YAKUMAN.contains(&yaku) {
            // don't add anything except another yakuman if a yakuman is already present
        } else if self.contains(&yaku) {
            // just in case, since each yaku can only be present once
        } else {
            match yaku {
                Yaku::Ryanpeiko => {
                    if self.contains(&Yaku::Ipeiko) {
                        self.retain(|x| *x != Yaku::Ipeiko);
                    } self.push(yaku); },
                Yaku::DoubleRiichi => {
                    if self.contains(&Yaku::Riichi) {
                        self.retain(|x| *x != Yaku::Riichi);
                    } self.push(yaku); },
                // yakuhai(0) isn't real and can't hurt you, but pretending that it is makes code cleaner elsewhere.
                Yaku::Yakuhai(count) => if count > 0 { self.push(yaku) },
                // nagashi mangan is incompatible with all other yaku
                Yaku::NagashiMangan => { self.clear(); self.push(yaku) },
                // and yakuman are incompatible with non-yakuman
                _ if YAKUMAN.contains(&yaku) => {
                    self.retain(|x| YAKUMAN.contains(x));
                    self.push(yaku)
                }
                _ => { self.push(yaku); }
            }
        }
    }

    fn append_checked(&mut self, other: &Vec<Self::T>) {
        for yaku in other { self.push_checked(*yaku); } }

    fn contains_any(&self, list: &Vec<Yaku>) -> bool {
        list.iter().any(|y| self.contains(y)) }
}

impl FindYaku for HandShape {
    fn yaku(&self, game_state: &GameState, seat_state: &SeatState) -> Vec<Yaku> {
        let win_type: WinType = seat_state.latest_type.unwrap().as_win();
        let mut yaku: Vec<Yaku> = { if let Some(y) = &seat_state.special_yaku { y.to_vec() } else { Vec::new() }};

        match self {
            HandShape::Standard {melds, pair} => {yaku.append_checked(&find_yaku_standard(
                melds, pair, win_type, game_state, seat_state))},
            HandShape::Chiitoi {pairs} => {yaku.append_checked(&find_yaku_chiitoi(pairs, win_type))},
            HandShape::Kokushi(y) => {yaku.append_checked(&y)},
            _ => panic!("reading yaku for incomplete hands is not implemented"),
        }

        yaku
    }
}

///////////////
// functions //
///////////////

// there are a lot of yaku to check for.
pub fn find_yaku_standard(melds: &[Meld; 4], pair: &Pair, win_type: WinType, game_state: &GameState, seat_state: &SeatState) -> Vec<Yaku> {
    let mut yaku: Vec<Yaku> = Vec::new();
    
    // closed tsumo
    if matches!(win_type, WinType::Tsumo) && seat_state.called_melds.is_none() { yaku.push_checked(Yaku::ClosedTsumo) }

    let win_tile = seat_state.latest_tile.unwrap();
    let all_tiles = seat_state.all_tiles();

    if !(all_tiles.has_any_honor() || all_tiles.has_any_terminal()) { yaku.push_checked(Yaku::Tanyao) }
    else if !all_tiles.has_any_simple() { // yaku defined by lack of simple tiles
        yaku.push_checked(Yaku::Honro);
        if !all_tiles.has_any_honor() { yaku.push_checked(Yaku::Chinroto) }
        if !all_tiles.has_any_terminal() { yaku.push_checked(Yaku::Tsuiso) }
    } else { // chanta and junchan
        if melds.iter().all(|m| m.has_terminal() || m.is_honor()) && !pair.is_simple() {
            if all_tiles.has_any_honor() { yaku.push_checked(Yaku::Chanta) }
            else { yaku.push_checked(Yaku::Junchan) }
        }
    }

    let hand_seqs: Vec<_> = melds.iter().filter(|m| m.is_seq()).collect();
    let hand_trips: Vec<_> = melds.iter().filter(|m| !m.is_seq() && m.is_numbered()).collect();

    match hand_seqs.len() {
        0 => { // toitoi and friends
            yaku.push_checked(Yaku::Toitoi);
            if seat_state.called_melds.is_none() { // suuankou variants
                if pair.tile() == win_tile { yaku.push_checked(Yaku::SuuankouTanki) }
                else if matches!(WinType::Tsumo, win_type) { yaku.push_checked(Yaku::Suuankou) }
                else { yaku.push_checked(Yaku::Sananko) } // ronning opened one of the triplets
            } else { // check if enough of the hand is closed for sananko
                if check_sananko(melds, pair, &win_type, &win_tile) { yaku.push_checked(Yaku::Sananko) }
            }
        },
        1 if check_sananko(melds, pair, &win_type, &win_tile) => yaku.push_checked(Yaku::Sananko),
        4 if !pair.is_dragon() => { // check for pinfu
            // for the pinfu wait to be valid, there must be one closed sequence where the winning tile wasn't in the center.
            // ... but it can't be a one-sided edge wait.
            if melds.iter().any(|m| !m.is_open && m.contains(&win_tile) && m.tiles[1].is_some_and(|t| t != win_tile) )
                // and the pair can't be the seat or round wind, because those both give fu.
                && pair.tile().wind() != Some(game_state.round_wind) && pair.tile().wind() != Some(seat_state.seat_wind)
                { yaku.push_checked(Yaku::Pinfu) }
        },
        _ => (),
    }

    // ipeiko and ryanpeiko are possible
    if hand_seqs.len() >= 2 && seat_state.called_melds.is_none() { 

    }

    // ittsuu is simple
    if hand_seqs.len() >= 3 && check_ittsuu(&hand_seqs) {
        yaku.push_checked(Yaku::Ittsuu)
    }

    // kan-based yaku are just a count away
    match melds.iter().filter(|m| m.is_quad()).count() {
        3 => yaku.push_checked(Yaku::Sankantsu),
        4 => yaku.push_checked(Yaku::Sukantsu),
        _ => ()
    }

    // suit-dependant yaku
    match seat_state.all_tiles().iter().map(|t| *t).collect::<Vec<_>>().count_suits() {
        1 => {
            if melds.has_any_honor() || pair.is_honor() { yaku.push_checked(Yaku::Honitsu) }
            else { yaku.push_checked(Yaku::Chinitsu);
                // ... and then check for churenpoto.
                if seat_state.called_melds.is_none() && melds.iter().all(|m| !m.is_quad()) 
                && check_churenpoto(&seat_state.all_tiles()) {
                    yaku.push_checked(Yaku::ChurenPoto);
                    if seat_state.all_tiles().count_occurrences(&win_tile) >= 2 {
                        yaku.push_checked(Yaku::SpecialWait)
                    }
                }
            }

            // also check for ryuiso
            if pair.is_pure_green(&game_state.ruleset)
            && melds.iter().all(|m| m.is_pure_green(&game_state.ruleset) ) {
                yaku.push_checked(Yaku::Ryuiso)
            }
        },
        3 if hand_seqs.len() >= 3 && check_sanshoku_doujun(&hand_seqs) => { // sanshoku is possible
            yaku.push_checked(Yaku::SanshokuDoujun);
        },
        3 if hand_seqs.len() <= 1 && check_sanshoku_douko(&hand_trips) => { // sanshoku douku is possible
            yaku.push_checked(Yaku::SanshokuDouko)
        },
        _ => ()
    }

    // vec<yaku> rejects yakuhai(0), so we can just try pushing one without checking that it makes sense to.
    yaku.push_checked(Yaku::Yakuhai({
        melds.iter().filter(|m| m.is_honor()).fold(0, |acc, m| {
            if m.is_dragon() { acc + 1 }
            else if m.wind().is_some_and(|w| w == game_state.round_wind && w == seat_state.seat_wind) { acc + 2 }
            else if m.wind().is_some_and(|w| w == game_state.round_wind || w == seat_state.seat_wind) { acc + 1 }
            else { acc }
        })
    }));

    // big and little winds
    if seat_state.all_tiles().iter().filter(|t| t.is_wind()).map(|t| t.wind().unwrap()).collect::<HashSet<_>>().len() == 4 {
        if pair.is_wind() { yaku.push_checked(Yaku::Shosushi) }
        else { yaku.push_checked(Yaku::Daisushi) }
    }

    // big and little dragons
    if seat_state.all_tiles().iter().filter(|t| t.is_dragon()).map(|t| t.dragon().unwrap()).collect::<HashSet<_>>().len() == 3 {
        if pair.is_dragon() { yaku.push_checked(Yaku::Shosangen) }
        else { yaku.push_checked(Yaku::Daisangen) }
    }

    yaku
}

// chiitoi is only eligible for a few yaku:
// tanyao, honro, honitsu, chinitsu, and daichiishin.
pub fn find_yaku_chiitoi( hand: &[Pair; 7], win_type: WinType ) -> Vec<Yaku> {
    let mut yaku: Vec<Yaku> = Vec::new();
    yaku.push_checked(Yaku::Chiitoi);
    if let WinType::Tsumo = win_type { yaku.push_checked(Yaku::ClosedTsumo) }
    if !hand.has_any_honor() {
        yaku.push_checked(Yaku::Tanyao);
        if hand.count_suits() == 1 { yaku.push_checked(Yaku::Chinitsu) }
    } else if !hand.has_any_simple() {
        yaku.push_checked(Yaku::Honro);
        if !hand.has_any_terminal() { yaku.push_checked(Yaku::Daichiishin) }
    } else {
        if hand.count_suits() == 1 { yaku.push_checked(Yaku::Honitsu) }
    }
    yaku
}

fn check_sananko(melds: &[Meld; 4], pair: &Pair, win_type: &WinType, win_tile: &Tile) -> bool {
    match melds.iter().fold(0, |acc, m| {
        if !m.is_seq() && !m.is_open { acc + 1 } else { acc }
    }) {
        4 => true,
        3 if matches!(WinType::Tsumo, win_type) => true,
        3 if pair.tile() == *win_tile => true,
        3 if melds.iter().filter(|m| m.is_seq() && !m.is_open).any(|m| m.contains(win_tile)) => true,
        _ => false
    }
}

// checks if a Vec<Meld> (length <= 4) contains tiles 1..=9 in a single suit.
// probably behaves if given a mixture of sequences and melds, but will return false negatives if a quad is present.
// the yaku-checking function passes a pre-filtered vec to avoid that.
fn check_ittsuu(melds: &Vec<&Meld>) -> bool {
    if melds.len() >= 3 { // only proceed if there are enough melds
        // TODO: rewrite to use hashset?
        let mut tiles = melds.iter().map(|m| m.as_tiles()).collect::<Vec<_>>().concat();
        tiles.sort();
        tiles.dedup();
        match tiles.len() {
            9 if tiles.count_suits() == 1 => return true, // easy.
            12 if tiles.count_suits() == 2 => { // if two suits are present, we check if one of them has 9 occurences
                if matches!( tiles.iter().filter(|t| t.suit() == tiles[0].suit() ).count(), 9 | 3 ) { return true }
            },
            _ => (),
        }
    }
    false
}

// checks if a Vec<Meld> contains three sequences of the same numbers in different suits.
// doesn't filter input.
fn check_sanshoku_doujun(melds: &Vec<&Meld>) -> bool {
    if melds.len() >= 3 {
        return melds.iter()
        .map(|m| m.tiles[0].unwrap())
        .circular_tuple_windows::<(_,_,_)>()
        .any(|(a, b, c)| {
            a.number() == b.number() && b.number() == c.number()
            && a.suit() != b.suit() && b.suit() != c.suit() && c.suit() != a.suit()
        })
    } false
}

// checks if a Vec<Meld> contains three trips/quads of the same number.
// doesn't check whether they have different suits.
// doesn't filter input.
fn check_sanshoku_douko(melds: &Vec<&Meld>) -> bool {
    if melds.len() >= 3 {
        return melds.iter()
        .map(|m| m.number())
        .circular_tuple_windows::<(_,_,_)>()
        .any(|(a, b, c)| a == b && b == c)
    } false
}

// checks if a Vec<Tile> meets criteria for churenpoto's shape.
// assumes that only one suit is present; will return false positives otherwise.
// will fail if given honor tiles.
fn check_churenpoto(tiles: &Vec<Tile>) -> bool {
    if tiles.iter().collect::<HashSet<&Tile>>().len() == 9 {
        // naive approach
        // TODO: refactor, test
        let mut arr: [i8; 9] = [0; 9];

        tiles.iter().for_each(|t| arr[(t.number().unwrap() - 1) as usize] += 1 );

        if [0,8].iter().all(|n| matches!(arr[*n as usize], 3|4))
        && arr[1..=7].iter().all(|n| matches!(arr[*n as usize], 1|2)) { return true }
    } false
}

fn three_in_common<T: std::cmp::PartialEq>(a: T, b: T, c: T, d: T) -> bool {
    (a == b || c == d ) && (a == c || b == d)
}

///////////
// tests //
///////////

mod tests {
    use super::*;
    use crate::tiles::{Tile, Dragon, Wind, Suit};
    use crate::conversions::{StringConversions};
    use crate::hand::{Hand, HandTrait};

    #[test]
    fn test_reading_hands(){
        let mut game = GameState{
            ruleset: RiichiRuleset::Default, round_wind: Wind::East,
            dora_markers: None, ura_dora_markers: None };
        let mut seat = SeatState{
            closed_tiles: "m2,m3,m4,p2,p3,p4,s2,s3,s4,dr,dr,dr,m9".to_tiles().unwrap(),
            called_melds: None, seat_wind: Wind::East, special_yaku: None,
            latest_tile: Some("m9".to_tile().unwrap()), latest_type: Some(TileType::Call), 
        };
        let mut hand = Hand::new(game, seat);
        assert_eq!(if let Hand::Agari {yaku, ..} = hand { yaku } else { panic!() }, vec![Yaku::SanshokuDoujun, Yaku::Yakuhai(1)]);
    }
}