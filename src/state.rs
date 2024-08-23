use crate::rulesets::{RiichiRuleset};
use crate::tiles::{Tile, Wind};
use crate::yaku::{Yaku};
use crate::hand::{Meld, MeldHas};
use crate::scoring::{Payment};

///////////////////////
// structs and enums //
///////////////////////

#[derive(Debug, PartialEq, Clone)]
pub struct GameState {
	pub ruleset: RiichiRuleset,
	pub round_wind: Wind,
	pub repeats: u8,
	pub dora_markers: Option<Vec<Tile>>,
	pub ura_dora_markers: Option<Vec<Tile>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SeatState {
	pub closed_tiles: Vec<Tile>,
	pub called_melds: Option<Vec<Meld>>,
	pub seat_wind: Wind,
    pub latest_tile: Option<Tile>,
    pub latest_type: Option<TileType>,
	pub special_yaku: Option<Vec<Yaku>>
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

pub trait SeatAccess {
	fn all_tiles(&self) -> Vec<Tile>;
}

pub trait InferWin {
	fn as_win(&self) -> WinType;
}

/////////////////////
// implementations //
/////////////////////

impl SeatAccess for SeatState {
	fn all_tiles(&self) -> Vec<Tile> {
		let mut all: Vec<Tile>;
		// if you gaze long enough into a void, the void will wink at you
		if let Some(calls) = &self.called_melds {
			all = self.closed_tiles.iter()
				.chain(calls.iter().map(|m| m.as_tiles()).collect::<Vec<_>>().concat().iter())
				.chain(self.latest_tile.iter()).map(|t| *t)
				.collect();
		} else {
			all = self.closed_tiles.iter()
				.chain(self.latest_tile.iter()).map(|t| *t)
				.collect();
		}
		all.sort();
		all
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