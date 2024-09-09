/*
While all riichi mahjong variants operate on the same basic principles, the variations between them influence how specific cases are scored. The list here is not exhaustive, and I hope to expand it.

TODO: are there any other commonly used rulesets?
TODO: option for custom ruleset loaded from JSON; wrap local yaku support into that.
*/

use std::env;
use std::cell::RefCell;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RiichiRuleset {
    JPML2022,
    JPML2023,
    WRC2022,
    EMA2016,
    MajSoul,
    Default // undefined faux-ruleset which uses the most common behavior where rulesets differ
}

pub trait RuleVariations {
    fn has_kiriage_mangan(&self) -> bool {true}     // round up to mangan
    fn has_yakuman_stacking(&self) -> bool {true}   // do multiple yakuman stack?
    fn has_double_yakuman(&self) -> bool {true}     // are special yakuman worth 2x limit?
    fn kazoe_yakuman_score(&self) -> u32 {8000}     // do 13+ han hands count as yakuman?
    fn double_wind_fu(&self) -> u8 {4}              // fu for seat+round wind pairs
    fn is_rinshan_tsumo(&self) -> bool {false}      // do rinshan winds score fu as a ron or tsumo?
    fn repeat_payment_ron(&self, counters: u8) -> i32 {i32::from(counters) * 300}
    fn repeat_payment_tsumo(&self, counters: u8) -> i32 {i32::from(counters) * 100}
    fn allows_all_green_hatsu(&self) -> bool {true} // is hatsu permitted in all green yakuman?
    fn requires_all_green_hatsu(&self) -> bool {false}  // ... is hatsu *required* in all green yakuman?
    fn allows_ippatsu(&self) -> bool {false}
    fn allows_double_riichi(&self) -> bool {true}
    fn allows_nagashi_mangan(&self) -> bool {true}
    fn counts_akadora(&self) -> bool {true}
    fn allows_open_tanyao(&self) -> bool {true}
}

impl RuleVariations for RiichiRuleset {
    fn has_kiriage_mangan(&self) -> bool {
        matches!(self, RiichiRuleset::WRC2022 | RiichiRuleset::MajSoul) } // TODO: verify MajSoul rules
    fn has_yakuman_stacking(&self) -> bool {
        !matches!(self, RiichiRuleset::EMA2016) } // TODO: verify JPML rules
    fn has_double_yakuman(&self) -> bool {
        matches!(self, RiichiRuleset::MajSoul) } // TODO: verify JPML rules
    fn kazoe_yakuman_score(&self) -> u32 { match self {
        RiichiRuleset::MajSoul => 8000, _ => 6000, } }
    fn double_wind_fu(&self) -> u8 { match self {
        RiichiRuleset::MajSoul | RiichiRuleset::JPML2022 => 4, _ => 2, } } // TODO: verify EMA rules
    fn is_rinshan_tsumo(&self) -> bool { 
        !matches!(self, RiichiRuleset::JPML2022) } // TODO: verify MajSoul rules
    fn allows_all_green_hatsu(&self) -> bool { true }
    fn requires_all_green_hatsu(&self) -> bool { 
        matches!(self, RiichiRuleset::JPML2022) }
    fn allows_ippatsu(&self) -> bool {
        !matches!(self, RiichiRuleset::JPML2022 | RiichiRuleset::JPML2023) }
    fn allows_nagashi_mangan(&self) -> bool {
        !matches!(self, RiichiRuleset::EMA2016) }
    fn counts_akadora(&self) -> bool {
        matches!(self, RiichiRuleset::MajSoul | RiichiRuleset::WRC2022) }
}

mod tests {
    use super::*;

    // use crate::yaku::Yaku;
    // use crate::rulesets::RiichiRuleset;
    // use crate::rulesets::RuleVariations;
    // use crate::scoring::{Payment, count_han, calc_base_points};
    // use crate::score_hand_from_str;

    // #[test]
    // fn test_kiriage_mangan() {
    //     assert_eq!(calc_base_points(4,30, &Vec::new(), RiichiRuleset::Default), Ok(1920));
    //     assert_eq!(calc_base_points(4,30, &Vec::new(), RiichiRuleset::MajSoul), Ok(2000));

    //     assert_eq!(calc_base_points(3,60, &Vec::new(), RiichiRuleset::Default), Ok(1920));
    //     assert_eq!(calc_base_points(3,60, &Vec::new(), RiichiRuleset::MajSoul), Ok(2000));

    //     // testing with full hands isn't really necessary, but it can't hurt.
    //     // 4han30fu: junchan pinfu
    //     assert_eq!(score_hand_from_str("s2,s3,s1,s3,s2,p7,p8,p9,p1,p1", "m1,m2,m3", "s1", 's', 'e', 'r', "p8", "", 0, ""), Ok(Payment::Ron(7700)));
    //     assert_eq!(score_hand_from_str("s2,s3,s1,s3,s2,p7,p8,p9,p1,p1", "m1,m2,m3", "s1", 's', 'e', 'r', "p8", "", 0, "majsoul"), Ok(Payment::Ron(8000)));
    
    //     assert_eq!(score_hand_from_str("s2,s3,s1,s3,s2,p7,p8,p9,p1,p1", "m1,m2,m3", "s1", 'e', 'e', 'r', "p8", "", 0, ""), Ok(Payment::Ron(11600)));
    //     assert_eq!(score_hand_from_str("s2,s3,s1,s3,s2,p7,p8,p9,p1,p1", "m1,m2,m3", "s1", 'e', 'e', 'r', "p8", "", 0, "majsoul"), Ok(Payment::Ron(12000)));
        
    //     // 3han60fu hand
    //     assert_eq!(score_hand_from_str("s1,s1,p1,p1,p3,p3,p3", "we,we,we,we|wn,wn,wn,wn", "s1", 's', 'e', 'r', "p8", "", 0, ""), Ok(Payment::Ron(7700)));
    //     assert_eq!(score_hand_from_str("s1,s1,p1,p1,p3,p3,p3", "we,we,we,we|wn,wn,wn,wn", "s1", 's', 'e', 'r', "p8", "", 0, "majsoul"), Ok(Payment::Ron(8000)));

    //     assert_eq!(score_hand_from_str("s1,s1,p1,p1,p3,p3,p3", "we,we,we,we|wn,wn,wn,wn", "s1", 'e', 's', 'r', "p8", "", 0, ""), Ok(Payment::Ron(11600)));
    //     assert_eq!(score_hand_from_str("s1,s1,p1,p1,p3,p3,p3", "we,we,we,we|wn,wn,wn,wn", "s1", 'e', 's', 'r', "p8", "", 0, "majsoul"), Ok(Payment::Ron(12000)));
    // }
    
    // #[test]
    // fn test_yakuman_rules() {
    //     // regardless of the ruleset we try to record all valid yakus; stacking/value is only relevant for count_han()

    //     // double yakuman
    //     assert_eq!(count_han(&vec![Yaku::Daisushi], 0, true, RiichiRuleset::Default), 13);
    //     assert_eq!(count_han(&vec![Yaku::Daisushi], 0, true, RiichiRuleset::MajSoul), 26);

    //     // yakuman stacking
    //     assert_eq!(count_han(&vec![Yaku::Ryuiso, Yaku::Tenho], 0, true, RiichiRuleset::Default), 26);
    //     assert_eq!(count_han(&vec![Yaku::Ryuiso, Yaku::Tenho], 0, true, RiichiRuleset::EMA2016), 13);

    //     // kazoe yakuman
    //     assert_eq!(calc_base_points(14,20,&vec![Yaku::Riichi], RiichiRuleset::Default), Ok(6000));
    //     assert_eq!(calc_base_points(14,20,&vec![Yaku::Riichi], RiichiRuleset::MajSoul), Ok(8000));

    //     assert_eq!(calc_base_points(15,20,&vec![Yaku::Ryuiso], RiichiRuleset::MajSoul), Ok(8000));
    //     assert_eq!(calc_base_points(25,20,&vec![Yaku::Ryuiso], RiichiRuleset::MajSoul), Ok(8000));
    //     // there's an edge case here where a hand with >13 dora and a yakuman will be scored as if it has two yakuman ...
    //     // but count_han() should zero out the dora count when it first encounters a yakuman in the list of yaku,
    //     // so that's only an issue if calc_base_points() is called directly.
    // }

    // #[test]
    // fn test_fu_rules() {
    //     // rinshan fu
    //     // jpml2022 treats rinshan as ron for the purposes of fu (ie: no +2 fu for tsumo); all others don't
    //     assert_eq!(score_hand_from_str("m7,m9,m9,m9,s9,s9,s9", "ws,ws,ws,ws|s9,s9,s9", "m8", 'e', 'e', 't', "", "rinshan", 0, "jpml2022"), Ok(Payment::DealerTsumo(1600)));
    //     assert_eq!(score_hand_from_str("m7,m9,m9,m9,s9,s9,s9", "ws,ws,ws,ws|s9,s9,s9", "m8", 'e', 'e', 't', "", "rinshan", 0, "default"), Ok(Payment::DealerTsumo(2000)));

    //     assert_eq!(score_hand_from_str("m2,m3,m4,m4,m5,m6,m7,s8,s8,s8", "we,we,we,we", "m1", 'e', 'e', 't', "", "rinshan", 0, "jpml2022"), Ok(Payment::DealerTsumo(2600)));
    //     assert_eq!(score_hand_from_str("m2,m3,m4,m4,m5,m6,m7,s8,s8,s8", "we,we,we,we", "m1", 'e', 'e', 't', "", "rinshan", 0, "default"), Ok(Payment::DealerTsumo(3200)));

    //     // double wind fu
    //     // jpml2022 and majsoul(?) give 4 fu for double wind pairs, all others don't
    //     // that nudges this hand up from 70 fu (20+2+4+8+32+2+2) to 80 fu (20+2+4+8+32+2+4, rounded up to 80)
    //     assert_eq!(score_hand_from_str("s1,s1,s1,s2,s4,we,we", "m9,m9,m9|!dr,dr,dr,dr", "s3", 'e', 'e', 't', "", "", 0, "jpml2022"), Ok(Payment::DealerTsumo(1300)));
    //     assert_eq!(score_hand_from_str("s1,s1,s1,s2,s4,we,we", "m9,m9,m9|!dr,dr,dr,dr", "s3", 'e', 'e', 't', "", "", 0, "default"), Ok(Payment::DealerTsumo(1200)));
    // }

    // #[test]
    // fn test_all_green_rules() {
    //     use crate::{Yaku, Wind, WinType};
    //     use crate::hand::{compose_hand, HandTools};
    //     use crate::tiles::{make_tiles_from_string, Tile, MakeTile};

    //     // hatsu is normally optional
    //     assert!(compose_hand(make_tiles_from_string("s2,s3,s4,s2,s3,s4,s6,s6,s6,dg,dg,dg,s8,s8").unwrap(), None, Tile::from_string("s2").unwrap(),
    //         WinType::Ron, Wind::East, Wind::South, None, None, RiichiRuleset::Default).unwrap().get_yaku().contains(&Yaku::Ryuiso));
    //     assert!(compose_hand(make_tiles_from_string("s2,s2,s2,s3,s3,s3,s4,s4,s4,s6,s6,s6,s8,s8").unwrap(), None, Tile::from_string("s2").unwrap(),
    //         WinType::Ron, Wind::East, Wind::South, None, None, RiichiRuleset::Default).unwrap().get_yaku().contains(&Yaku::Ryuiso));

    //     // jpml2022 made hatsu mandatory
    //     assert!(compose_hand(make_tiles_from_string("s2,s3,s4,s2,s3,s4,s6,s6,s6,dg,dg,dg,s8,s8").unwrap(), None, Tile::from_string("s2").unwrap(),
    //         WinType::Ron, Wind::East, Wind::South, None, None, RiichiRuleset::JPML2022).unwrap().get_yaku().contains(&Yaku::Ryuiso));
    //     assert!(!compose_hand(make_tiles_from_string("s2,s2,s2,s3,s3,s3,s4,s4,s4,s6,s6,s6,s8,s8").unwrap(), None, Tile::from_string("s2").unwrap(),
    //         WinType::Ron, Wind::East, Wind::South, None, None, RiichiRuleset::JPML2022).unwrap().get_yaku().contains(&Yaku::Ryuiso));
    // }

    // #[test]
    // fn test_allowed_yaku() {

    // }
}


