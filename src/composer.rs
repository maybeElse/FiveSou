use crate::tiles::{Tile, Dragon, Wind, Suit, TileIs, TileRelations};
use crate::errors::errors::{HandError, ParsingError};
use crate::hand::{HandShape, Meld, Pair, Wait, PartialHand, PartialHandTrait};
use crate::conversions::TileConversions;

////////////
// traits //
////////////

pub trait Counter {
    type T;

    fn to_counted_tuples(&mut self) -> Vec<(usize, Self::T)>;
}

pub trait Composer {
    fn compose_tiles(&self, consider_waits: Option<u8>, consider_kokushi: bool) -> Option<Vec<PartialHand>>;
    fn unpack(&self) -> Vec<Tile>;
}

/////////////////////
// implementations //
/////////////////////

impl Counter for Vec<Tile> {
    type T = Tile;

    fn to_counted_tuples(&mut self) -> Vec<(usize, Self::T)> {
        self.sort();

        let mut vec: Vec<(usize, Tile)> = Vec::with_capacity(self.len());
        let mut iter = self.iter();
        let mut count: usize = 1;
        let mut last: &Tile = iter.next().unwrap();

        while let Some(new) = iter.next() {
            if new != last {
                vec.push((count, *last));
                count = 0;
                last = new;
            }
            count += 1;
        }
        vec.push((count, *last));

        vec
    }
}

impl Composer for Vec<(usize, Tile)> {
    fn compose_tiles(&self, consider_waits: Option<u8>, consider_kokushi: bool) -> Option<Vec<PartialHand>> {
        if self.is_empty() { return None }
        else {
            let length: usize = self.len();
            let depth: usize = self[0].0;

            // TODO: tune reserved length
            let mut partials: Vec<PartialHand> = Vec::with_capacity(self.len());

            if depth >= 1 && length >= 3 { // check for sequence
                if let Some(temp) = [self[0].1, self[1].1, self[2].1].make_meld(false) {
                    let mut subs = self.clone();
                    if subs[2].0 == 1 { subs.remove(2); } else { subs[2].0 -= 1 }
                    if subs[1].0 == 1 { subs.remove(1); } else { subs[1].0 -= 1 }
                    if subs[0].0 == 1 { subs.remove(0); } else { subs[0].0 -= 1 }

                    if let Some(recursions) = subs.compose_tiles(consider_waits, false) {
                        for mut value in recursions {
                            value.push_meld(temp);
                            partials.push(value);
                        }
                    } else if consider_waits.is_some() || subs.is_empty() {
                        let mut t = Vec::with_capacity(4);
                        t.push(temp);
                        partials.push( PartialHand::new(
                            subs.unpack(), t, Vec::new() 
                        ) );
                    }
                }
            }
            if depth >= 2 { // pair is possible
                if let Some(temp) = [self[0].1; 2].make_pair() {
                    let mut subs = self.clone();
                    if subs[0].0 == 2 { subs.remove(0); } else { subs[0].0 -= 2 }

                    if let Some(recursions) = subs.compose_tiles(consider_waits, consider_kokushi && !self[0].1.is_simple()) {
                        for mut value in recursions {
                            value.push_pair(temp);
                            partials.push(value);
                        }
                    } else if consider_waits.is_some() || consider_kokushi || (length == 1 && depth == 2) {
                        let mut t = Vec::with_capacity(7);
                        t.push(temp);
                        partials.push( PartialHand::new(
                            subs.unpack(), Vec::new(), t
                        ) );
                    }
                }
            }
            if depth >= 3 { // triplet is possible
                if let Some(temp) = [self[0].1; 3].make_meld(false) {
                    let mut subs = self.clone();
                    if subs[0].0 == 3 { subs.remove(0); } else { subs[0].0 -= 3 }

                    if let Some(recursions) = subs.compose_tiles(consider_waits, false) {
                        for mut value in recursions {
                            value.push_meld(temp);
                            partials.push(value);
                        }
                    } else if consider_waits.is_some() || subs.is_empty() {
                        let mut t = Vec::with_capacity(4);
                        t.push(temp);
                        partials.push( PartialHand::new(
                            subs.unpack(), t, Vec::new() 
                        ) );
                    }
                }
            }
            
            if depth == 1 && ((consider_waits.is_some()) || (consider_kokushi && !self[0].1.is_simple())) {
                if let Some(recursion) = self[1..].to_vec()
                    .compose_tiles(if let Some(num) = consider_waits { if num > 1 { Some(num-1) } else { None } } else { None },
                    !self[0].1.is_simple() ) {
                    for mut value in recursion {
                        value.push_tile(self[0].1);
                        partials.push(value);
                    }
                }
            }

            partials.sort();
            partials.dedup();
            
            if partials.is_empty() { return None }
            else { return partials.into() }
        }
    }

    fn unpack(&self) -> Vec<Tile> {
        if self.is_empty() { Vec::new() }
        else {
            let mut vec: Vec<Tile> = Vec::new();

            for (count, tile) in self {
                for i in 0..*count { vec.push(*tile) }
            }

            vec
        }
    }
}

///////////
// tests //
///////////

mod tests {
    use super::*;
    use crate::conversions::StringConversions;
    use crate::hand::PartialHandTrait;
    use crate::hand::compose_tiles;

    #[test]
    fn test_composing() {
        assert_eq!(("we,we").to_tiles().unwrap().to_counted_tuples().compose_tiles(None, false),
                compose_tiles(&("we,we").to_tiles().unwrap(), false, None, false));

        assert_eq!(("dw,dw,dw,we,we,we").to_tiles().unwrap().to_counted_tuples().compose_tiles(None, false),
                compose_tiles(&("dw,dw,dw,we,we,we").to_tiles().unwrap(), false, None, false));
    }
}
