pub mod errors {
    #[derive(Debug, PartialEq)]
    pub enum HandError {
        NoYaku,
        TileCount,
        ValueError(ValueError),
        ParseError(ParsingError),
        TileError,
        WrongPipeline, // nagashi mangan shouldn't go through the normal scoring pipeline
        Unimplemented,
        NoHands,
        WrongMeldType,
    }

    #[derive(Debug, PartialEq)]
    pub enum ParsingError {
        BadChar,
        BadString,
        BadInteger,
        Unimplemented
    }

    #[derive(Debug, PartialEq)]
    pub enum ValueError {
        BadInput
    }

    #[derive(Debug, PartialEq)]
    pub enum CompositionError {
        NoYaku,
        InvalidHand,
        NotImplemented,
        DeadBranch,
        End,
        BadTiles
    }
}