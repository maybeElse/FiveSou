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
		if let Some(calls) = &self.called_melds {
			let mut all = [self.closed_tiles.clone(), 
				[calls.iter().map(|m| m.as_tiles()).collect::<Vec<_>>()].concat().concat()
			].concat();
			all.sort();
			all
		} else { self.closed_tiles.clone() }
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