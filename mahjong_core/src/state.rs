use crate::rulesets::RiichiRuleset;
use crate::tiles::{Tile, Wind};
use crate::yaku::Yaku;
use crate::hand::{Meld, MeldHas};
use crate::scoring::Payment;

///////////////////////
// structs and enums //
///////////////////////

#[derive(Debug, PartialEq, Clone)]
pub struct Game {
	pub ruleset: RiichiRuleset,
	pub round_wind: Wind,
	pub repeats: u8,
	pub dora_markers: Option<Vec<Tile>>,
	pub ura_dora_markers: Option<Vec<Tile>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Seat {
	pub closed_tiles: Vec<Tile>,
	pub called_melds: Option<Vec<Meld>>,
	pub seat_wind: Wind,
    pub latest_tile: Option<Tile>,
    pub latest_type: Option<TileType>,
	pub special_yaku: Option<Vec<Yaku>>,
	pub all_tiles: Option<Vec<Tile>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Win {
	pub win_type: WinType,
	pub winning_tile: Tile,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TileType {Call, Draw, Kan}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WinType {Tsumo, Ron}

////////////
// traits //
////////////

pub trait SeatHelper {
	fn new(
		closed_tiles: Vec<Tile>,
		called_melds: Option<Vec<Meld>>,
		seat_wind: Wind,
    	latest_tile: Option<Tile>,
    	latest_type: Option<TileType>,
		special_yaku: Option<Vec<Yaku>>,
	) -> Self where Self: Sized;
}

pub trait GameHelper {
	fn new(
		ruleset: RiichiRuleset,
		round_wind: Wind,
		epeats: u8,
		dora_markers: Option<Vec<Tile>>,
		ura_dora_markers: Option<Vec<Tile>>,
	) -> Self where Self: Sized;
}

pub trait SeatAccess {
	fn all_tiles(&self) -> Vec<Tile>;
}

pub trait InferWin {
	fn as_win(&self) -> WinType;
}

/////////////////////
// implementations //
/////////////////////

impl GameHelper for Game {
	fn new(
		ruleset: RiichiRuleset,
		round_wind: Wind,
		repeats: u8,
		dora_markers: Option<Vec<Tile>>,
		ura_dora_markers: Option<Vec<Tile>>,
	) -> Self where Self: Sized {
		Game {
			ruleset,
			round_wind,
			repeats,
			dora_markers,
			ura_dora_markers
		}
	}
}

impl SeatHelper for Seat {
	fn new(
		closed_tiles: Vec<Tile>,
		called_melds: Option<Vec<Meld>>,
		seat_wind: Wind,
    	latest_tile: Option<Tile>,
    	latest_type: Option<TileType>,
		special_yaku: Option<Vec<Yaku>>,
	) -> Self where Self: Sized {
		Seat {
			closed_tiles: closed_tiles.clone(),
			called_melds: called_melds.clone(),
			seat_wind,
			latest_tile,
			latest_type,
			special_yaku,
			all_tiles: {
				let mut tiles = [closed_tiles, called_melds.unwrap_or_default().iter().map(super::hand::MeldHas::as_tiles).collect::<Vec<_>>().concat(), {
					if latest_tile.is_some() { vec![latest_tile.unwrap()] } else { Vec::new() }
				}].concat();
				tiles.sort();
				Some(tiles)
			}
		}
	}
}

impl SeatAccess for Seat {
	fn all_tiles(&self) -> Vec<Tile> {
		if let Some(tiles) = self.all_tiles.clone() {
			tiles
		} else {
			let mut all_tiles = [self.closed_tiles.clone(), self.called_melds.clone().unwrap_or_default().iter().map(super::hand::MeldHas::as_tiles).collect::<Vec<_>>().concat(), {
				if self.latest_tile.is_some() { vec![self.latest_tile.unwrap()] } else { Vec::new() }
			}].concat();
			all_tiles.sort();
			all_tiles
		}
		// let mut all: Vec<Tile>;
		// // if you gaze long enough into a void, the void will wink at you
		// if let Some(calls) = &self.called_melds {
		// 	all = self.closed_tiles.iter()
		// 		.chain(calls.iter().map(|m| m.as_tiles()).collect::<Vec<_>>().concat().iter())
		// 		.chain(self.latest_tile.iter()).map(|t| *t)
		// 		.collect();
		// } else {
		// 	all = self.closed_tiles.iter()
		// 		.chain(self.latest_tile.iter()).map(|t| *t)
		// 		.collect();
		// }
		// all.sort();
		// all
	}
}

impl InferWin for TileType {
	fn as_win(&self) -> WinType {
		match self {
			TileType::Call => WinType::Ron,
			TileType::Draw | TileType::Kan => WinType::Tsumo,
		}
	}
}