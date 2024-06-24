pub mod errors {
    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum ScoringError {
        NoYaku,
        TileCount,
        ValueError(ValueError),
        ParseError(ParsingError),
        WrongPipeline, // nagashi mangan and yakuman hands shouldn't go through the normal scoring pipeline
        Unimplemented
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum ParsingError {
        BadChar,
        BadString,
        BadInteger,
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum ValueError {
        BadInput
    }
}