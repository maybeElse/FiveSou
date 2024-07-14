/*
While all riichi mahjong variants operate on the same basic principles, the variations between them influence how specific cases are scored. The list here is not exhaustive, and I hope to expand it.

TODO: are there any other commonly used rulesets?
TODO: option for custom ruleset loaded from JSON; wrap local yaku support into that.
*/

#[derive(Debug, Clone, Copy)]
pub enum RiichiRuleset {
    JPML2022,
    JPML2023,
    WRC2022,
    EMA2016,
    MajSoul,
}

pub trait RuleVariations {
    fn has_kiriage_mangan(&self) -> bool {true}
    fn has_yakuman_stacking(&self) -> bool {true}
    fn has_double_yakuman(&self) -> bool {true}
    fn kazoe_yakuman_score(&self) -> i32 {8000}
    fn double_wind_fu(&self) -> i8 {4}
    fn rinshan_fu(&self) -> bool {false}
    fn repeat_payment_ron(&self, counters: i8) -> i32 {counters as i32 * 300}
    fn repeat_payment_tsumo(&self, counters: i8) -> i32 {counters as i32 * 100}
    fn allows_all_green_hatsu(&self) -> bool {true}
    fn requires_all_green_hatsu(&self) -> bool {false}
    fn allows_ippatsu(&self) -> bool {false}
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
    fn rinshan_fu(&self) -> bool { match self {
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