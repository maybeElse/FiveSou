# Mahjong Scorer Design Document
(reflects initial planning, not current state)

## Required Information:
- Hand
	Open parts, closed parts	Sum to 13
- Winning tile
- Ron/tsumo
- Seat and Round Winds (infer dealer/non-dealer)
- Dora and Ura-dora markers
- Special criteria:
	- Riichi
	- Double Riichi
	- Ippatsu
	- Under the Sea (very last tile drawn, tsumo)
	- Under the River (very last tile discarded, ron)
	- After a Kan
	- Robbing a Kan
	- Nagashi Mangan
- Counters/Repeats

## Implied signature:
```
	score_hand(
		open_tiles = Vec: Tile
		closed_tiles = Vec: Tile
		winning_tile = Enum: Tile
		win_type = Enum: WinType
		seat_wind = Enum: Wind
		round_wind = Enum: Wind
		dora_markers = Vec: Tile
		special_yaku = Vec: YakuSpecial
		repeats = Int
	)
```

## Steps:
1. Count han
	a. Find all applicable yaku
	b. Count dora and ura-dora
	c. Sum them
	-> if multiple possible arrangements, use the most valuable one; save this arrangement for fu counting
	-> if five+ han, it's a mangan and fu counting can be skipped
2. Count fu
	a. 20fu for winning
	b. 10fu for winning by ron from a closed hand
	c. variable fu from melds and pairs
								simple	terminal	honor	
		triplet		open		2		4 			4
					closed		4		8 			8
		quadruplet	open		8		16			16
					closed		16		32 			32
		sequence				0 		0 			n/a
		pair 	2fu for seat wind, prevailing wind, and dragon.
				4fu if seat wind and prevailing wind match.
	d. variable fu from the wait
		single tile wait 		2fu 	includes waits for inner side of outermost tiles, ie 12 waiting on 3 or 89 waiting on 7
										includes waits for a pair meld, as long as it's not shanpon
		all other waits			0fu
	e. 2fu for tsumo, unless hand is pinfu
	f. round up to nearest tens
	-> if yaku includes Seven Pairs, set fu to 25 and do not round
	-> if win is via ron, hand is open, and no fu were awarded (ie: open pinfu), set fu to 30
3. Calculate base points from han&fu
	a. fu * 2 ^ (2+han)
	-> if base points are more than mangan, treat as a mangan (2,000 base points, multiplied as follows)
		ie: 3han/70+fu, 4han/40+fu, any 5han or higher
		Mangan						1x
		Haneman				6-7han		1.5x
		Baiman 				8-10han		2x
		Sanbaiman			11-12han	3x
		Kazoe Yakuman		13+han		4x
		Yakuman		Contains Yakuman	4x
		Multiple Yakuman
4. Multiply base points based on dealer/non-dealer, and tsumo/ron; round up to nearest 100
	Tsumo	Dealer 			All non-dealers pay 2x base
			Non-dealer		Dealer pays 2x base, non-dealers pay 1x base
	Ron 	Dealer 			Discarding player pays 6x base
			Non-dealer 		Discarding player pays 4x base
5. Add bonus based on repeats/counters
	a. winner(s) get bonus points equal to 300 x counters
		Ron 	300 added to discarding player's payment
		Tsumo 	100 added to all payments

## Program Elements:
```
Enum: WinType
		Ron
		Tsumo
Enum: Tile
		Simple ->	Struct: Simple
						Enum: Suit		Characters/Man, Bamboo/Sou, Circles/Pin
						Int: Number		(constrain: 1-9 inclusive)
		Honor ->	Enum: Honor
						Dragon ->	Enum: Dragon	White, Green, Red
						Wind ->		Enum: Wind		East, South, West, North

Enum: Yaku 			(Yaku which can be determined from hand shape)
Enum: YakuSpecial 	(Yaku which cannot be determined from hand shape)
				Riichi (entire hand must be closed)
				Double Riichi (entire hand must be closed)
				Ippatsu (must be riichi/double riichi)
				Under the Sea (must be tsumo)
				Under the River (must be ron)
				After a Kan (must be tsumo)
				Robbing a Kan (must be ron)
				Nagashi Mangan
Enum: HandError
```

# Reading Waits

## What does it mean to be in Tenpai?
The addition of one tile will complete the hand, causing it to have *either* 4-melds-and-a-pair, 7-pairs, or to match the 13-orphan pattern of 12-orphans-and-a-pair. Under some rulesets (*atozuke nashi*) a hand cannot win if it only gains yaku upon the winning call.

## So ...
Not after a draw:
- If the incomplete hand has three melds and a pair, the two remaining tiles are either identical (waiting on a third; this implies a second wait involving the pair), adjacent numbers in the same suit (a double-side or edge wait), or one-apart numbers in the same suit (a middle wait).
- If the incomplete hand has four melds, the one remaining tile is the wait.
- If the incomplete hand has six pairs, the one remaining tile is the wait.
- Kokushi is a special case.

After a draw:
- If the incomplete hand has three melds and a pair, at least two of the remaining three tiles satisfy the conditions above and the three tiles are not a complete meld. One of them needs to be discarded.
- If the incomplete hand has four melds and two singleton tiles, one of the singletons must be discarded for the remaining one to become the wait.
- If the incomplete hand has six pairs and two singletons, one of the singletons must be discarded for the remaining one to become the wait.
- Kokushi is a special case.

Incomplete hands which do not satisfy at least one of these criteria might be Iishanten; this should be a seperate pipeline.

## So ...
```
	read_waits(
		called_melds = Vec: Meld
		closed_tiles = Vec: Tile
	) -> Vec: Waits
```

This wraps `compose_tiles()` (see hand.rs) and operates on the `Vec<PartialHand>` it returns.

Waits is an enum representing the two possibilities (a list of waits and the discard/wait pairs). Discard/wait pairs are tuples like `(tile, [tile])`.

... I could also extend `compose_hand`'s Err(HandError) return to handle tenpai (&etc) hands.