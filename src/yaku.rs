use crate::tiles::{Tile, Dragon, Wind, Suit, TileHelpers};
use crate::hand::{Hand, FullHand, HandHelpers, Meld, MeldHelpers, HandTools, Pair, SequenceHelpers, VecTileHelpers};
use crate::errors::errors::{ScoringError, ParsingError};
use core::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WinType {Tsumo, Ron,}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Yaku {
    Chiitoi,        // unique shape, fully closed hand          2 han closed
    ClosedTsumo,    // tsumo, fully closed hand                 1 han closed

    // based on sequence
    Pinfu,          // no fu awarded                            1 han closed
    Ipeiko,         // two identical sequences, closed hand     1 han closed
    Sanshoku,       // same sequence in each suit               2 han closed / 1 han open
    Ittsuu,         // straight (1-9 in a suit)                 2 han closed / 1 han open
    Ryanpeiko,      // ipeiko twice. replaces ipeiko            3 han closed

    // based on triplets/quads
    Toitoi,         // all triplets                             2 han
    Sananko,        // three concealed triplets                 2 han
    SanshokuDouko,  // same triplet in each suit                2 han
    Sankantsu,      // three quads                              2 han

    // based on terminal/honor
    Tanyao,         // no honor or terminal                     1 han
    Yakuhai(i8),     // triplets or quads of dragons,           1 han per triplet
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

// some yaku are mutually exclusive; for instance, Ipeiko cannot coexist with Ryanpeiko
// hopefully my yaku-identification logic will handle this, but just in case ...
// see https://riichi.wiki/Yaku_compatibility
pub trait YakuHelpers {
    type T;

    fn from_string(special: &str) -> Result<Self, ParsingError> where Self: Sized { Err(ParsingError::Unimplemented) }
    fn push_checked(&mut self, yaku: Self::T);
    fn append_checked(&mut self, yaku: &Vec<Self::T>);
    fn contains_any(&self, yaku: &Self) -> bool;
}
impl YakuHelpers for Vec<Yaku> {
    type T = Yaku;

    fn from_string(special_yaku: &str) -> Result<Self, ParsingError> {
        let input: Vec<&str> = special_yaku.split(',').collect::<Vec<&str>>();
        let mut result: Vec<Yaku> = vec![];
        for yaku in input {
            match yaku.to_lowercase().as_str() {
                "riichi" => result.push_checked(Yaku::Riichi),
                "ippatsu" => result.push_checked(Yaku::Ippatsu),
                "doubleriichi" => result.push_checked(Yaku::DoubleRiichi),
                "undersea" | "underthesea" | "haiteiraoyue" | "haitei" => result.push_checked(Yaku::UnderSea),
                "underriver" | "undertheriver" | "houteiraoyui" | "houtei" => result.push_checked(Yaku::UnderRiver),
                "afterkan" | "rinshan" | "rinshankaiho" => result.push_checked(Yaku::AfterKan),
                "robbedkan" | "robbingakan" | "chankan" => result.push_checked(Yaku::RobbedKan),
                "nagashimangan" => result.push_checked(Yaku::NagashiMangan),
                "tenho" | "blessingofheaven" => result.push_checked(Yaku::Tenho),
                "chiho" | "blessingofearth" => result.push_checked(Yaku::Chiho),
                _ => return Err(ParsingError::BadString)
            }
        }
        Ok(result)
    }

    fn push_checked(&mut self, yaku: Yaku) {
        if self.contains_any(&YAKUMAN.to_vec()) && !YAKUMAN.contains(&yaku) {
            // don't add anything except another yakuman if a yakuman is already present
        } else if self.contains(&yaku) {
            // just-in-case, since each yaku can only be present once
        } else {
            match yaku {
                Yaku::Ryanpeiko => {
                    if self.contains(&Yaku::Ipeiko) {
                        self.retain(|x| *x != Yaku::Ipeiko);
                    }
                    self.push(yaku);
                },
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
        for yaku in other {
            self.push_checked(*yaku);
        }
    }

    fn contains_any(&self, list: &Vec<Yaku>) -> bool {
        for yaku in list {
            if self.contains(yaku) {
                return true
            }
        }
        false
    }
} 
// impl YakuHelpers for Vec<YakuSpecial> {
//     type T = YakuSpecial;

//     fn from_string(special_yaku: &str) -> Result<Self, ParsingError> {
//         let input: Vec<&str> = special_yaku.split(',').collect::<Vec<&str>>();
//         let mut result: Vec<YakuSpecial> = vec![];
//         for yaku in input {
//             match yaku.to_lowercase().as_str() {
//                 "riichi" => result.push_checked(YakuSpecial::Riichi),
//                 "ippatsu" => result.push_checked(YakuSpecial::Ippatsu),
//                 "doubleriichi" => result.push_checked(YakuSpecial::DoubleRiichi),
//                 "undersea" | "underthesea" | "haiteiraoyue" | "haitei" => result.push_checked(YakuSpecial::UnderSea),
//                 "underriver" | "undertheriver" | "houteiraoyui" | "houtei" => result.push_checked(YakuSpecial::UnderRiver),
//                 "afterkan" | "rinshan" | "rinshankaiho" => result.push_checked(YakuSpecial::AfterKan),
//                 "robbedkan" | "robbingakan" | "chankan" => result.push_checked(YakuSpecial::RobbedKan),
//                 "nagashimangan" => result.push_checked(YakuSpecial::NagashiMangan),
//                 "tenho" | "blessingofheaven" => result.push_checked(YakuSpecial::Tenho),
//                 "chiho" | "blessingofearth" => result.push_checked(YakuSpecial::Chiho),
//                 _ => return Err(ParsingError::BadString)
//             }
//         }
//         Ok(result)
//     }
//     fn push_checked(&mut self, yaku: YakuSpecial) { 
//         if self.contains_any(&YAKUMAN_SPECIAL.to_vec()) || self.contains(&YakuSpecial::NagashiMangan) {
//             // nagashi mangan is incompatible with other yaku,
//             // and tenho/chiho are incompatible with each other
//         } else {
//             match yaku {
//                 YakuSpecial::NagashiMangan | YakuSpecial::Tenho | YakuSpecial::Chiho 
//                   => { self.clear(); self.push(yaku) },
//                 _ => { self.push(yaku); }
//             }
//         }
//     }
//     fn contains_any(&self, list: &Vec<YakuSpecial>) -> bool {
//         for yaku in list {
//             if self.contains(yaku) {
//                 return true
//             }
//         }
//         false
//     }
// }

/////////////////////////////
// YAKU CHECKING FUNCTIONS //
/////////////////////////////

pub fn find_yaku_standard(
    hand: FullHand,
    winning_tile: Tile,
    special_yaku: &Option<Vec<Yaku>>,
    open: bool,
    seat_wind: Wind,
    round_wind: Wind,
    win_type: WinType
) -> Result<Vec<Yaku>, ScoringError> {
    // given four values, returns true if three of them are equal
    fn three_in_common(a: &[u8], b: &[u8], c: &[u8], d: &[u8]) -> bool {
        (a == b || c == d ) && (a == c || b == d) }

    // returns true if any two of the melds in the array are equal
    fn check_ipeiko(melds: Vec<Meld>) -> bool {
        for i in 0..(&melds.len()-1) { if melds[i+1..].contains(&melds[i]) {  return true } } return false }

    // TODO: refactor
    // check if the narrow requirements for a sananko are satisfied
    fn check_sananko(hand: FullHand, winning_tile: Tile, win_type: WinType) -> bool {
        let closed_melds = hand.without_sequences().iter().fold(0, |acc, meld| { if meld.is_closed() { 1 } else { 0 } } );
        if closed_melds == 4 { return true
        } else if closed_melds == 3 {
            if let WinType::Tsumo = win_type { return true
            } else if hand.pair.tile == winning_tile { return true
            } else {    // at this point, it's only a sananko if the winning tile was in a closed sequence
                let seqs: Vec<Meld> = hand.only_sequences();
                if seqs.len() == 1 {
                    if let Meld::Sequence {..} = seqs[0] {
                        return seqs[0].is_closed()
                } }
        } } return false }

    // TODO: refactor
    // checks if a vec<meld> contains a valid ittsuu
    // note: the first meld's suit must be the ittsuu suit
    fn check_ittsuu(seqs: Vec<Meld>) -> bool {
        let mut tracker: [bool; 3] = [false, false, false];
        for meld in &seqs { 
            if seqs[0].get_suit() == meld.get_suit() { match meld.as_numbers()[0] { 
                1 => tracker[0] = true,
                4 => tracker[1] = true,
                7 => tracker[2] = true,
                _ => panic!()
        } } }
        tracker[0] && tracker[1] && tracker[2]
    }

    // TODO: refactor
    // checks if the hand shape matches churenpoto's 1-1-1-2-3-4-5-6-7-8-9-9-9
    fn check_churenpoto(hand: FullHand) -> bool {
        fn tiles_as_array(hand: FullHand) -> [i8; 9] {
            let mut arr: [i8; 9] = [0; 9];
            arr[hand.pair.tile.get_number().unwrap() as usize - 1] = 2;
            for meld in hand.melds {
                match meld {
                    Meld::Triplet {tile, ..} => arr[tile.get_number().unwrap() as usize - 1] += 1,
                    Meld::Sequence {tiles, ..} => {
                        for tile in tiles {  arr[tile.get_number().unwrap() as usize - 1] += 1 } },
                    Meld::Kan {..} => panic!()
            } } arr }

        let arr: [i8; 9] = tiles_as_array(hand);

        if ([3,4].contains(&arr[0]) && [3,4].contains(&arr[8])) && {
            for i in &arr[1..8] { if ![1,2].contains(i) { return false } } return true
        } { true } else { false } }

    let mut yaku: Vec<Yaku> = special_yaku.clone().unwrap_or_default();

    if let WinType::Tsumo = win_type { if !open { yaku.push_checked(Yaku::ClosedTsumo) } }

    if !hand.has_any_honor() && !hand.has_any_terminal() {
        yaku.push_checked(Yaku::Tanyao);
    } else if !hand.has_any_simple() { // yaku defined by the lack of simple tiles
        yaku.push_checked(Yaku::Honro);
        if !hand.has_any_honor() { yaku.push_checked(Yaku::Chinroto) }
        if !hand.has_any_terminal() { yaku.push_checked(Yaku::Tsuiso) }
    } else { // chanta and junchan
        // first, check for chanta (each meld/pair has a terminal or honor)
        // if that's the case, check for junchan (hand has no honors)
        let mut end_or_honor_in_all: bool = true;
        if hand.pair.has_honor() || hand.pair.has_terminal() {
            for meld in hand.melds {
                if !meld.has_terminal() || !meld.has_honor() {
                    end_or_honor_in_all = false } }
        } else { end_or_honor_in_all = false }

        if end_or_honor_in_all {
            if hand.has_any_honor() { // chanta
                yaku.push_checked(Yaku::Chanta);
            } else { // junchan
                yaku.push_checked(Yaku::Junchan);
    } } }

    let hand_seqs: Vec<Meld> = hand.only_sequences();

    match hand_seqs.len() { // toitoi and pinfu
        0 => { yaku.push_checked(Yaku::Toitoi); 
            if !open { // suuankou variants and downgrade to sananko for ronning to complete a triplet
                if hand.pair.contains_tile(&winning_tile) { yaku.push_checked(Yaku::SuuankouTanki)
                } else if let WinType::Tsumo = win_type { yaku.push_checked(Yaku::Suuankou)
                } else if let WinType::Ron = win_type { yaku.push_checked(Yaku::Sananko) }
            } else { // ... but if it's open, enough could still be closed for sananko
                if check_sananko(hand, winning_tile, win_type) { yaku.push_checked(Yaku::Sananko) }
        } },
        1 => { // sananko is possible here too
            if check_sananko(hand, winning_tile, win_type) { yaku.push_checked(Yaku::Sananko) 
        } },
        4 => { // pinfu (including open pinfu, which is 0 han but awards fu)
            for meld in hand.melds {
                if meld.contains_tile(&winning_tile) { yaku.push_checked(Yaku::Pinfu); break }
        } },
        _ => () }

    let mut ittsuu_seqs = hand_seqs.clone();
    ittsuu_seqs.retain(|x| x.ittsuu_viable()); ittsuu_seqs.sort();
    if ittsuu_seqs.len() >= 3 { // ittsuu is possible
        // ittsuu_seqs's melds are sorted at this point, so if there are only 3 sequences ...
        if ittsuu_seqs.len() == 3 && ittsuu_seqs[0].get_suit() == ittsuu_seqs[1].get_suit() &&  ittsuu_seqs[1].get_suit() == ittsuu_seqs[2].get_suit() 
        && ittsuu_seqs[0].as_numbers().contains(&1) && ittsuu_seqs[1].as_numbers().contains(&4) && ittsuu_seqs[2].as_numbers().contains(&7) {
            yaku.push_checked(Yaku::Ittsuu)
        } else {
            match { // TODO: this should be a trait on Vec<Meld>
                let mut suits: Vec<Suit> = vec![];
                for meld in &ittsuu_seqs{ if !suits.contains(&meld.get_suit().unwrap()) { suits.push(meld.get_suit().unwrap()) } }
                suits.len()
            } {
                1 => {
                    if check_ittsuu(ittsuu_seqs) { yaku.push_checked(Yaku::Ittsuu) }
                },
                2 => match ittsuu_seqs[1..].iter().fold(0, |acc, meld| { if meld.get_suit() == ittsuu_seqs[0].get_suit() { 1 } else { 0 } }) 
                {
                    1 => if check_ittsuu(ittsuu_seqs[1..].to_vec()) { yaku.push_checked(Yaku::Ittsuu) },
                    3 => if check_ittsuu(ittsuu_seqs) { yaku.push_checked(Yaku::Ittsuu) },
                    _ => ()
                },
                _ => ()
            }
        }
    }
    if hand_seqs.len() >= 2 && !open { // ipeiko and ryanpeiko are possible
        // the hand's melds should be sorted at this point, so ...
        if hand.melds[0] == hand.melds[1] && hand.melds[2] == hand.melds[3] { yaku.push_checked(Yaku::Ryanpeiko)
        } else if check_ipeiko(hand_seqs) { yaku.push_checked(Yaku::Ipeiko) } }

    match hand.count_kans() { // kan-based yaku
        3 => yaku.push_checked(Yaku::Sankantsu),
        4 => yaku.push_checked(Yaku::Sukantsu),
        _ => () }

    match hand.count_suits() { // suit-dependant yaku
        1 => {
            if hand.has_any_honor() { yaku.push_checked(Yaku::Honitsu)
            } else { yaku.push_checked(Yaku::Chinitsu);
                // ... but we'll also check for churenpoto after pushing that.
                // basic criteria: no kans, and either 1 or 2 triplets
                if !open && hand.count_kans() == 0 && check_churenpoto(hand) {
                    yaku.push_checked(Yaku::ChurenPoto);
                    if hand.as_tiles().count_occurances(winning_tile) >= 2 {
                        yaku.push_checked(Yaku::SpecialWait)
            } } }

            if hand.is_pure_green() { yaku.push_checked(Yaku::Ryuiso) }
        },
        3 => { // sanshoku is possible
            let seqs: Vec<Meld> = hand.only_sequences();
            if seqs.len() == 3 { // the case that's easy to check
                if seqs[0].as_numbers() == seqs[1].as_numbers() && seqs[0].as_numbers() == seqs[2].as_numbers() {
                    yaku.push_checked(Yaku::Sanshoku) }
            } else if seqs.len() == 4 && three_in_common(
                &seqs[0].as_numbers(), &seqs[1].as_numbers(), &seqs[2].as_numbers(), &seqs[3].as_numbers()) {
                    yaku.push_checked(Yaku::Sanshoku)
            } else if seqs.len() <= 1 { // Sanshoku Douku might be possible
                let trips: Vec<Meld> = hand.without_sequences();
                if trips.len() == 3 {
                    if trips[0].get_tile().unwrap().get_number() == trips[1].get_tile().unwrap().get_number()
                    && trips[1].get_tile().unwrap().get_number() == trips[2].get_tile().unwrap().get_number() {
                        yaku.push_checked(Yaku::SanshokuDouko) }
                } else if seqs.len() == 4 && three_in_common( // this is really ugly, sorry
                    &[trips[0].get_tile().unwrap().get_number().unwrap()], &[trips[1].get_tile().unwrap().get_number().unwrap()],
                    &[trips[2].get_tile().unwrap().get_number().unwrap()], &[trips[3].get_tile().unwrap().get_number().unwrap()]) {
                        yaku.push_checked(Yaku::SanshokuDouko) }
        } },
        _ => () }

    // yakuhai counting
    let yakuhai_counter: i8 = hand.melds.iter().fold(0, |acc, meld| {
        match meld {
            Meld::Triplet {tile, ..} | Meld::Kan {tile, ..} => {
                match tile {
                    Tile::Wind(wind) => {
                        if seat_wind == round_wind && *wind == seat_wind { acc + 2
                        } else if *wind == seat_wind { acc + 1
                        } else if *wind == round_wind { acc + 1 } else { acc } },
                    Tile::Dragon(dragon) => acc + 1,
                    _ => acc } },
           _ => acc, }
    });
    if yakuhai_counter > 0 { yaku.push_checked(Yaku::Yakuhai(yakuhai_counter))}

    if hand.count_winds() == 4 { // big and little winds
        if hand.pair.is_wind() { yaku.push_checked(Yaku::Shosushi)
        } else { yaku.push_checked(Yaku::Daisushi) } }

    if hand.count_dragons() == 3 { // big and little dragons
        if hand.pair.is_dragon() { yaku.push_checked(Yaku::Shosangen)
        } else { yaku.push_checked(Yaku::Daisangen) } }

    if yaku.len() == 0 { Err(ScoringError::NoYaku) } else { Ok(yaku) }
}

// chiitoi is only eligible for a few yaku:
// tanyao, honro, honitsu, chinitsu, and daichiishin
pub fn find_yaku_chiitoi(
    hand: [Pair; 7], winning_tile: Tile, special_yaku: &Option<Vec<Yaku>>, win_type: WinType,
) -> Result<Vec<Yaku>, ScoringError> {
    let mut yaku: Vec<Yaku> = special_yaku.clone().unwrap_or_default();
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
    Ok(yaku)
}

///////////
// tests //
///////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hand;
    use crate::tiles;
    use crate::scoring;

    #[test]
    fn yaku_from_strings(){
        let mut yaku: Vec<Yaku> = Vec::from_string("riichi").unwrap();
        assert_eq!(yaku, vec![Yaku::Riichi]);
        
        yaku = Vec::from_string("riichi,ippatsu").unwrap();
        assert_eq!(yaku, vec![Yaku::Riichi, Yaku::Ippatsu]);
        
        yaku = Vec::from_string("riichi,ippatsu,nagashimangan").unwrap();
        assert_eq!(yaku, vec![Yaku::NagashiMangan]);
        
        yaku = Vec::from_string("chiho,robbedkan").unwrap();
        assert_eq!(yaku, vec![Yaku::Chiho]);
        
        yaku = Vec::from_string("robbedkan,chiho").unwrap();
        assert_eq!(yaku, vec![Yaku::Chiho]);
    }

    #[test]
    fn yaku_push_checked(){
        let mut yaku = vec![Yaku::Ipeiko];
        yaku.push_checked(Yaku::Pinfu);
        assert_eq!(yaku, vec![Yaku::Ipeiko, Yaku::Pinfu]);

        yaku = vec![Yaku::Yakuhai(2)];
        yaku.push_checked(Yaku::Shosangen);
        assert_eq!(yaku, vec![Yaku::Yakuhai(2), Yaku::Shosangen]);

        yaku = vec![Yaku::Ipeiko];
        yaku.push_checked(Yaku::Ryanpeiko);
        assert_eq!(yaku, vec![Yaku::Ryanpeiko]);

        yaku = vec![Yaku::Ipeiko];
        yaku.push_checked(Yaku::Ipeiko);
        assert_eq!(yaku, vec![Yaku::Ipeiko]);
    }

    #[test]
    fn basic_yaku_tests(){
        use crate::tiles::FromString;

        let hand: Hand = hand::compose_hand(tiles::make_tiles_from_string("m2,m3,m4,p5,p6,p7,p4,p5,p6,s3,s4,s5,m7,m7").unwrap(),
            None, Tile::from_string("m4").unwrap(), WinType::Tsumo, Wind::East, Wind::South, None, None).unwrap();
        assert_eq!(hand.get_yaku(), vec![Yaku::ClosedTsumo, Yaku::Tanyao, Yaku::Pinfu]);

        let hand: Hand = hand::compose_hand(tiles::make_tiles_from_string("m2,m2,m3,m3,m4,m4,s2,s3,s4,p2,p3,p4,p9,p9").unwrap(),
            None, Tile::Number{ suit: Suit::Man, number: 4, red: false }, WinType::Ron, Wind::East, Wind::South, None, None).unwrap();
        assert_eq!(hand.get_yaku(), vec![Yaku::Pinfu, Yaku::Ipeiko, Yaku::Sanshoku]);

        let hand: Hand = hand::compose_hand(tiles::make_tiles_from_string("m2,m2,m3,m3,m4,m4,s2,s3,s4,p2,p2,p2,p8,p8").unwrap(),
            None, Tile::from_string("m4").unwrap(), WinType::Ron, Wind::East, Wind::South, None, None).unwrap();
        assert_eq!(hand.get_yaku(), vec![Yaku::Tanyao, Yaku::Ipeiko, Yaku::Sanshoku]);

        let hand: Hand = hand::compose_hand(tiles::make_tiles_from_string("p1,p2,p3,p4,p4,p4,p5,p6,p7,p8,s2,s3,s4,p9").unwrap(),
            None, Tile::from_string("p9").unwrap(), WinType::Tsumo, Wind::East, Wind::East, None, None).unwrap();
        assert_eq!(hand.get_yaku(), vec![Yaku::ClosedTsumo, Yaku::Pinfu, Yaku::Ittsuu]);
        assert_eq!(hand.is_closed(), true);
        assert_eq!(hand.get_han(), 4);
        assert_eq!(hand.get_fu(), 20);

        let hand: Hand = hand::compose_hand(tiles::make_tiles_from_string("m2,m2,m3,m3,p3,p3,p5,p5,s6,s6,s7,s7,s8,s8").unwrap(),
            None, Tile::from_string("s7").unwrap(), WinType::Tsumo, Wind::East, Wind::East, None, None).unwrap();
        assert_eq!(hand.get_yaku(), vec![Yaku::Chiitoi, Yaku::ClosedTsumo, Yaku::Tanyao]);
        assert_eq!(hand.is_closed(), true);
        assert_eq!(hand.get_han(), 4);
        assert_eq!(hand.get_fu(), 25);

        let hand: Hand = hand::compose_hand(tiles::make_tiles_from_string("m3,m5,m6,m7,m8,m8,m8,m3").unwrap(),
            hand::make_melds_from_string("p8,p8,p8|m2,m2,m2", true), Tile::from_string("m3").unwrap(), WinType::Tsumo, Wind::East, Wind::East, None, None).unwrap();
        assert_eq!(hand.get_yaku(), vec![Yaku::Tanyao]);
        assert_eq!(hand.is_closed(), false);
        assert_eq!(hand.get_han(), 1);
        assert_eq!(hand.get_fu(), 40);

        let hand: Hand = hand::compose_hand(tiles::make_tiles_from_string("p2,p2,p2,we,we").unwrap(),
            hand::make_melds_from_string("m8,m8,m8|p3,p3,p3|s8,s8,s8", true), Tile::from_string("p2").unwrap(), WinType::Ron, Wind::South, Wind::East, None, None).unwrap();
        assert_eq!(hand.get_yaku(), vec![Yaku::Toitoi]);
        assert_eq!(hand.get_han(), 2);
        assert_eq!(hand.get_fu(), 30);

        let hand: Hand = hand::compose_hand(tiles::make_tiles_from_string("p1,p2,p3,p4,p5,p6,p7,p7,p7,we,we").unwrap(),
            hand::make_melds_from_string("ws,ws,ws", true), Tile::from_string("p1").unwrap(), WinType::Tsumo, Wind::South, Wind::East, None, None).unwrap();
        assert_eq!(hand.get_yaku(), vec![Yaku::Honitsu, Yaku::Yakuhai(1)]);
        assert_eq!(hand.get_han(), 3);
        assert_eq!(hand.get_fu(), 40);

        let hand: Hand = hand::compose_hand(tiles::make_tiles_from_string("p2,p3,p3,p4,p4,p5,p5,p2").unwrap(),
        hand::make_melds_from_string("s8,s8,s8|!s7,s7,s7,s7", true), Tile::from_string("p2").unwrap(), WinType::Tsumo, Wind::East, Wind::East, None, None).unwrap();
        assert_eq!(hand.get_yaku(), vec![Yaku::Tanyao]);
        assert_eq!(hand.get_fu(), 50);

        let hand: Hand = hand::compose_hand(tiles::make_tiles_from_string("m1,m2,m3,m4,m4,m5,m6,m7,s8,s8,s8").unwrap(),
        hand::make_melds_from_string("we,we,we,we", true), Tile::from_string("m3").unwrap(), WinType::Tsumo, Wind::East, Wind::East, None, None).unwrap();
        assert_eq!(hand.get_yaku(), vec![Yaku::Yakuhai(2)]);
        assert_eq!(hand.get_fu(), 50);
    }
}