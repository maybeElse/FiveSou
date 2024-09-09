# FiveSou: A Riichi Mahjong Hand Scorer

Given a completed hand and various relevant information, determine how the hand is scored and what payment split should be applied.

## Usage

The intended use case for this library is as the scoring and tile reading engine for some larger program (I intend to use it to build and test simple bots). I think these functions are the most relevant for that:

- `state.rs`, `tiles.rs`, and `yaku.rs` provide structs and enums used everywhere throughout the library, as well as traits for interacting with them.
- `hand::Hand::new()` takes information about the player and the game and turns it into a hand. `HandTrait` then simplifies extracting information from it.
- `composer.rs`'s traits add a function to `Vec<Tile>`s which converts it into a `Vec<(usize, Tile)>` and then reads the hand's composition from that (I'm very pleased with this; it removed the need for binary searches and improved performance a fair bit). While this is generally expected to be called on the tiles in a hand, it can also be used for more speculative purposes.
- `lib.rs` provides `score_hand_from_str()`, which is primarily meant for unit tests and suchlike.

## Planned Features

**Sanma**: 3-player mahjong introduces one new mechanic (*Kita*, in which north wind tiles can be called to count as additional dora) and removes or adjusts several other parts of the game. Adding support for it as a ruleset is an obvious step.

**Reading waits**: Currently, scoring only considers possible waits for the purpose of finding the highest han/fu reads during scoring. Being able to identify possible waits from an incomplete hand—as well as which yaku would apply if it's finished on different waits—is the obvious next step. The skeleton of this feature is already in place.

**Understanding the entire board**: Taking in more information about the board's state to identify special yaku (ie riichi, ippatsu, nagashi mangan, etc) would be extremely neat. However, it would be a massive pain to generate test cases for, and the amount of information would be impractical to enter unless plugged into a full game. I do not expect to implement it until I need to.