# FiveSou: A Riichi Mahjong Hand Scorer

Given a completed hand and various relevant information, this determines how the hand is scored and what payment split should be applied.

## Usage

Three functions are provided in main.rs: `score_hand_from_str()`, `score_hand_from_structs()`, and `human_readable_scoring()`. The first wraps the second, and the third outputs information about how the program read the hand, which yaku apply, and the hand's score.

If run as a stand-alone program it will guide the user through entering necessary information in the console; I imagine that this is an extremely niche use-case, as almost all experienced mahjong players have memorized how to score hands and less experienced players would be better served by using a [scoring table](https://riichi.wiki/Scoring_table). A better use case would be as the scoring engine for a game, but that's not really my goal in writing this program.

## Current Features

- Scores hands according to the [Japanese Professional Mahjong League's 2023 A-rules](https://cloudymahjong.com/wp-content/uploads/2023/12/JPML-A-rules-2023.pdf), with three exceptions: kan/uradora are counted if the markers are provided, ippatsu is considered valid if specified, and kazoe yakuman is scored as a yakuman.

## Planned Features

**Multiple Rulesets**: Riichi Mahjong has quite a few different rulesets with variations in how edge cases are handled (ie: does a rinshan win gain the 2 fu from a tsumo?), how yakuman are scored (are multiple yakuman valid? do special waits qualify as double yakuman?), and which yaku are valid. Providing support for them is an obvious next step.

**Sanma**: 3-player mahjong introduces one new mechanic (*Kita*, in which north wind tiles can be called to count as additional dora) and removes or adjusts several other parts of the game. Adding support for it as a ruleset is an obvious step.

**Reading waits**: Currently, scoring only considers possible waits for the purpose of finding the highest han/fu reads during scoring. Being able to identify possible waits from an incomplete hand—as well as which yaku would apply if it's finished in different waits—is an obvious place to expand beyond only scoring completed hands.

**Understanding the entire board**: Taking in more information about the board's state to identify special yaku (ie riichi, ippatsu, nagashi mangan, etc) would be extremely neat. However, it would be a massive pain to generate test cases for, and the amount of information would be impractical to enter unless plugged into a full game.