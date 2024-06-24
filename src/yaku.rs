use crate::errors::errors::{ScoringError, ParsingError};

#[derive(Debug)]
pub enum WinType {Tsumo, Ron,}

#[derive(Debug)]
pub enum Yaku {
    Chitoi,         // unique shape, fully closed hand          2 han closed
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
    Suanko,         // four concealed triplets, double wait     limit           closed
    Daisangen,      // big three dragons                        limit           open or closed
    Shosushi,       // little four winds (3 triplets + pair)    limit           open or closed
    Daisushi,       // big four winds (4 triplets)              double limit    open or closed
    Tsuiso,         // all honors                               limit           open or closed
    Chinroto,       // all terminals                            limit           open or closed
    Ryuiso,         // all green (sou 2,3,4,6,8 + green dragon) limit           open or closed
    ChurenPoto,     // nine gates                               limit           closed
    Sukantsu,       // four kans                                limit           open or closed
    
    SpecialWait,    // the yakuman's wait adds additional value limit
                    // ie: four concealed triplets, single wait
                    //     thirteen orphans, thirteen-way wait
                    //     nine gates, nine-way wait
                    // I think that breaking this out into a unique criteria will simplify code somewhat
}

#[derive(Debug)]
pub enum YakuSpecial {
    Riichi,         // declared Riichi, fully closed hand       1 han closed
    DoubleRiichi,   // declared Riichi on first turn            2 han closed
    Ippatsu,        // win within one go-around after Riichi    1 han closed
    UnderSea,       // very last tile drawn, tsumo only         1 han
    UnderRiver,     // very last tile discarded, ron only       1 han
    AfterKan,       // win on dead wall draw, tsumo only        1 han
    RobbedKan,      // ron only                                 1 han
    NagashiMangan,   // counts as tsumo, ignores other yaku      automatic mangan

    // special yakuman hands
    Tenho,          // blessing of heaven. tsumo.               limit   closed, dealer only
    Chiho,          // blessing of earth. tsumo.                limit   closed, non-dealer only
}
