pub mod errors {
    #[derive(Debug)]
    pub enum ScoringError {
        NoYaku,
        TileCount,
        ParseError(ParsingError),
        Yakuman, // yakuman hands shouldn't go through the normal scoring pipeline
        Unimplemented
    }

    #[derive(Debug)]
    pub enum ParsingError {
        BadChar,
        BadString,
        BadInteger,
    }
}