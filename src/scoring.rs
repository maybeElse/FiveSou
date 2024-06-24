use crate::errors::errors::{ScoringError, ParsingError, ValueError};
use crate::yaku::{Yaku, YakuSpecial, WinType};

#[derive(Debug, PartialEq)]
pub enum Payment{
    DealerTsumo(i32),
    Tsumo(PaymentSplit),
    Ron(i32)
}

#[derive(Debug, PartialEq)]
pub struct PaymentSplit{
    pub dealer: i32,
    pub non_dealer: i32,
}

pub fn find_yaku(

) -> Result<Vec<Yaku>, ScoringError> {
    Err(ScoringError::Unimplemented)
}

pub fn count_han(
    yaku_normal: Vec<Yaku>,
    yaku_special: Vec<YakuSpecial>,
    dora: i8,
    closed: bool,
) -> Result<i8, ScoringError>{
    let mut han_count: i8 = dora;

    for yaku in yaku_special {
        match yaku {
            YakuSpecial::Riichi | YakuSpecial::Ippatsu | YakuSpecial::UnderSea 
            | YakuSpecial::UnderRiver | YakuSpecial::AfterKan | YakuSpecial::RobbedKan
            => han_count += 1,
            YakuSpecial::DoubleRiichi => han_count += 2,
            _ => return Err(ScoringError::WrongPipeline) // nagashi mangan, tenho, and chiho
        }
    }

    for yaku in yaku_normal {
        match yaku {
            // special criteria
            Yaku::Chitoi => han_count += 2,

            // based on luck
            Yaku::ClosedTsumo => han_count += 1,

            // based on sequences
            Yaku::Pinfu | Yaku::Ipeiko => han_count += 1,
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

            // yakuman hands
            _ => return Err(ScoringError::WrongPipeline)
        }
    }

    Ok(han_count)
}

pub fn count_fu(
    
) -> Result<i8, ScoringError>{
    Err(ScoringError::Unimplemented)
}

pub fn calc_base_points( han: i8, fu: i8 ) -> Result<i32, ScoringError> {
    if han < 0 || fu < 20 {
        return Err(ScoringError::ValueError(ValueError::BadInput))
    } else {
        match han {
            0 => return Err(ScoringError::NoYaku),
            1 ..= 4 => {
                let bp: i32 = (fu as i32) * (2_i32.pow(2 + (han as u32)));
                if bp > 2000 { Ok(2000) } 
                else { Ok(bp) } },
            5 => Ok(2000),          // Mangan
            6 | 7 => Ok(3000),      // Haneman
            8 | 9 | 10 => Ok(4000), // Baiman
            11 | 12 => Ok(6000),    // Sanbaiman
            _ => Ok(8000)           // Kazoe Yakuman
        }
    }
}

pub fn calc_player_split(
    base: i32,
    is_dealer: bool,
    win_type: WinType,
    repeats: i8
) -> Result<Payment, ScoringError> {
    match win_type {
        WinType::Tsumo => {
            if is_dealer { Ok(Payment::DealerTsumo(base * 2)) }
            else { Ok(Payment::Tsumo(PaymentSplit{dealer: 2 * base, non_dealer: base})) }
        }
        WinType::Ron => Ok(Payment::Ron(base * {if is_dealer {6} else {4}}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn han_counts(){
        assert_eq!(count_han(
            vec![Yaku::Chitoi],
            vec![YakuSpecial::Riichi],
            0, true).unwrap(), 3)
    }

    #[test]
    fn base_point_calc(){
        assert_eq!(calc_base_points(1, 50).unwrap(), 400);
        assert_eq!(calc_base_points(2, 40).unwrap(), 640);
        assert_eq!(calc_base_points(3, 70).unwrap(), 2000);
        assert_eq!(calc_base_points(4, 40).unwrap(), 2000);
        assert_eq!(calc_base_points(7, 30).unwrap(), 3000);
        assert_eq!(calc_base_points(9, 50).unwrap(), 4000);
        assert_eq!(calc_base_points(11, 40).unwrap(), 6000);
        assert_eq!(calc_base_points(13, 50).unwrap(), 8000);
        
        assert_eq!(calc_base_points(0, 50), Err(ScoringError::NoYaku));
        assert_eq!(calc_base_points(0, 10), Err(ScoringError::ValueError(ValueError::BadInput)));
    }

    #[test]
    fn bp_and_split_calc(){
        assert_eq!(calc_player_split(calc_base_points(4, 40).unwrap(), false, WinType::Tsumo, 0).unwrap(),
                    Payment::Tsumo(PaymentSplit{dealer: 4000, non_dealer: 2000}));
        assert_eq!(calc_player_split(calc_base_points(2, 50).unwrap(), true, WinType::Tsumo, 0).unwrap(),
                    Payment::DealerTsumo(1600));
        assert_eq!(calc_player_split(calc_base_points(3, 70).unwrap(), true, WinType::Ron, 0).unwrap(),
                    Payment::Ron(12000));
    }
}