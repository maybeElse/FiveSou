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

pub trait FromString { fn from_string(str: &str) -> Self where Self: Sized; }

impl FromString for RiichiRuleset {
    fn from_string(str: &str) -> Self {
        match str.to_lowercase().as_str() {
            "jpml2022" => return RiichiRuleset::JPML2022,
            "jpml2023" => return RiichiRuleset::JPML2023,
            "wrc2022" => return RiichiRuleset::WRC2022,
            "ema2016" => return RiichiRuleset::EMA2016,
            "majsoul" | "mahjongsoul" => return RiichiRuleset::MajSoul,
            _ => RiichiRuleset::Default, } }
}

pub trait RuleVariations {
    fn has_kiriage_mangan(&self) -> bool {true}     // round up to mangan
    fn has_yakuman_stacking(&self) -> bool {true}   // do multiple yakuman stack?
    fn has_double_yakuman(&self) -> bool {true}     // are special yakuman worth 2x limit?
    fn kazoe_yakuman_score(&self) -> i32 {8000}     // do 13+ han hands count as yakuman?
    fn double_wind_fu(&self) -> i8 {4}              // fu for seat+round wind pairs
    fn is_rinshan_tsumo(&self) -> bool {false}      // do rinshan winds score fu as a ron or tsumo?
    fn repeat_payment_ron(&self, counters: i8) -> i32 {counters as i32 * 300}
    fn repeat_payment_tsumo(&self, counters: i8) -> i32 {counters as i32 * 100}
    fn allows_all_green_hatsu(&self) -> bool {true} // is hatsu permitted in all green yakuman?
    fn requires_all_green_hatsu(&self) -> bool {false}  // ... is hatsu *required* in all green yakuman?
    fn allows_ippatsu(&self) -> bool {false}
    fn allows_double_riichi(&self) -> bool {true}
    fn allows_nagashi_mangan(&self) -> bool {true}
    fn counts_akadora(&self) -> bool {true}
}

impl RuleVariations for RiichiRuleset {
    fn has_kiriage_mangan(&self) -> bool { match self {
        RiichiRuleset::WRC2022 | RiichiRuleset::MajSoul => true, _ => false, } } // TODO: verify MajSoul rules
    fn has_yakuman_stacking(&self) -> bool { match self {
        RiichiRuleset::EMA2016 => false, _ => true, } } // TODO: verify JPML rules
    fn has_double_yakuman(&self) -> bool { match self {
        RiichiRuleset::MajSoul => true, _ => false, } } // TODO: verify JPML rules
    fn kazoe_yakuman_score(&self) -> i32 { match self {
        RiichiRuleset::MajSoul => 8000, _ => 6000, } }
    fn double_wind_fu(&self) -> i8 { match self {
        RiichiRuleset::MajSoul | RiichiRuleset::JPML2022 => 4, _ => 2, } } // TODO: verify EMA rules
    fn is_rinshan_tsumo(&self) -> bool { match self {
        RiichiRuleset::JPML2022 => false, _ => true, } } // TODO: verify MajSoul rules
    fn allows_all_green_hatsu(&self) -> bool { true }
    fn requires_all_green_hatsu(&self) -> bool { match self {
        RiichiRuleset::JPML2022 => true, _ => false, } }
    fn allows_ippatsu(&self) -> bool { match self {
        RiichiRuleset::JPML2022 | RiichiRuleset::JPML2023 => false, _ => true, } }
    fn allows_nagashi_mangan(&self) -> bool { match self {
        RiichiRuleset::EMA2016 => false, _ => true, } }
    fn counts_akadora(&self) -> bool { match self {
        RiichiRuleset::MajSoul | RiichiRuleset::WRC2022 => true, _ => false, } }
}

// RULESET will be set the first time it's accessed in a thread.
// If I was doing something more complex with global variables (ie lots of options) this wouldn't be ideal,
// but for a single one it seems like the safest option.
// ... although RefCell does make the code for testing stuff a bit more complicated.
// NOTE: as a result of this, test cases must be run using a containerized test runner (ie Maelstrom)
thread_local!(pub static RULESET: RefCell<RiichiRuleset> = RefCell::new(get_env_ruleset()));

fn get_env_ruleset() -> RiichiRuleset {
    match env::var("RIICHI_RULESET") {
        Ok(val) => { match val.to_lowercase().as_str() {
            "jpml2022" => return RiichiRuleset::JPML2022,
            "jpml2023" => return RiichiRuleset::JPML2023,
            "wrc2022" => return RiichiRuleset::WRC2022,
            "ema2016" => return RiichiRuleset::EMA2016,
            "majsoul" | "mahjongsoul" => return RiichiRuleset::MajSoul,
            _ => (), } } _ => (),
    } RiichiRuleset::MajSoul // default
}

mod tests {
    use super::*;

    use std::env;
    use std::borrow::Borrow;
    use crate::rulesets::RiichiRuleset;
    use crate::rulesets::RULESET;
    use crate::rulesets::RuleVariations;
    use crate::scoring::Payment;

    #[test]
    fn ruleset_usage_test() {
        env::set_var("RIICHI_RULESET", "JPML2022");

        RULESET.with_borrow(|r| assert!(r == &RiichiRuleset::JPML2022));
        assert_eq!(RULESET.with_borrow(|r| r.is_rinshan_tsumo()), false);
    }
}


