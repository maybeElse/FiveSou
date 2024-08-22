pub mod errors {
    #[derive(Debug, PartialEq)]
    pub enum HandError {
        NoYaku,
        TileCount,
        ValueError,
        ParseError(ParsingError),
        Unimplemented,
        NotAgari
    }

    #[derive(Debug, PartialEq)]
    pub enum ParsingError {
        Empty,
        BadChar,
        BadString,
        BadInteger,
        BadMeld,
        Unimplemented,
        NothingFound
    }
}