pub use self::alchemicalbonus::{ AlchemicalBonus };
pub use self::armorbonus::{ ArmorBonus };
pub use self::circumstancebonus::{ CircumstanceBonus };
pub use self::competencebonus::{ CompetenceBonus };
pub use self::deflectionbonus::{ DeflectionBonus };
pub use self::dodgebonus::{ DodgeBonus };
pub use self::enhancementbonus::{ EnhancementBonus };
pub use self::inherentbonus::{ InherentBonus };
pub use self::insightbonus::{ InsightBonus };
pub use self::luckbonus::{ LuckBonus };
pub use self::moralebonus::{ MoraleBonus };
pub use self::naturalarmorbonus::{ NaturalArmorBonus };
pub use self::naturalarmorenhancementbonus::{ NaturalArmorEnhancementBonus };
pub use self::profanebonus::{ ProfaneBonus };
pub use self::racialbonus::{ RacialBonus };
pub use self::resistancebonus::{ ResistanceBonus };
pub use self::sacredbonus::{ SacredBonus };
pub use self::shieldbonus::{ ShieldBonus };
pub use self::sizebonus::{ SizeBonus };
pub use self::traitbonus::{ TraitBonus };
pub use self::untypedbonus::{ UntypedBonus };
pub use self::penalty::{ Penalty };
mod armorbonus;
mod alchemicalbonus;
mod circumstancebonus;
mod competencebonus;
mod deflectionbonus;
mod dodgebonus;
mod enhancementbonus;
mod inherentbonus;
mod insightbonus;
mod luckbonus;
mod moralebonus;
mod naturalarmorbonus;
mod naturalarmorenhancementbonus;
mod profanebonus;
mod racialbonus;
mod resistancebonus;
mod sacredbonus;
mod shieldbonus;
mod sizebonus;
mod traitbonus;
mod untypedbonus;
mod penalty;

pub trait BonusTracker {
    fn total(&self) -> u8;

    fn add(&mut self, u8) -> &mut Self;

    fn remove(&mut self, u8) -> &mut Self;
}



/// An object which tracks stacking bonus values.
struct StackingTracker {
	tracker: Vec<u8>,
}

impl StackingTracker {
	fn new() -> StackingTracker {
		StackingTracker {
			tracker: Vec::new()
		}
	}
}

impl BonusTracker for StackingTracker {
	fn total(&self) -> u8 {
		return self.tracker.iter().fold(0, |a, &b| a + b);
	}

	fn add(&mut self, amt: u8) -> &mut StackingTracker {
		self.tracker.push(amt);
		return self;
	}

	fn remove(&mut self, amt: u8) -> &mut StackingTracker {
		if let Some(index) = self.tracker.iter().position(|&i| i == amt) {
		    self.tracker.remove(index);
		}
		return self;
	}
}

#[cfg(test)]
mod stackingtrackertests {
	use super::*;
	use bonuses;

	#[test]
	fn total_default() {
		// tests to make sure the default value is 0
		let bt = bonuses::StackingTracker::new();
		assert_eq!(0, bt.total())
	}

	#[test]
	fn add_1() {
		// tests to make sure a single value makes a round trip
		let mut bt = bonuses::StackingTracker::new();
		bt.add(1);
		assert_eq!(1, bt.total())
	}

	#[test]
	fn add_1_1() {
		// bonuses should stack
		let mut bt = bonuses::StackingTracker::new();
		bt.add(1);
		bt.add(1);
		assert_eq!(2, bt.total())
	}

	#[test]
	fn add_1_2() {
		// bonuses should stack
		let mut bt = bonuses::StackingTracker::new();
		bt.add(1);
		bt.add(2);
		assert_eq!(3, bt.total())
	}

	#[test]
	fn add_2_1() {
		// bonuses should stack
		let mut bt = bonuses::StackingTracker::new();
		bt.add(2);
		bt.add(1);
		assert_eq!(3, bt.total())
	}

	#[test]
	fn remove_default() {
		// removing a nonexistant value should not panic
		let mut bt = bonuses::StackingTracker::new();
		bt.remove(1);
	}

	#[test]
	fn remove_1() {
		let mut bt = bonuses::StackingTracker::new();
		bt.add(1);
		bt.remove(1);
		assert_eq!(0, bt.total())
	}

	#[test]
	fn remove_1_1() {
		let mut bt = bonuses::StackingTracker::new();
		bt.add(1);
		bt.add(1);
		bt.remove(1);
		assert_eq!(1, bt.total())
	}

	#[test]
	fn remove_1_2() {
		let mut bt = bonuses::StackingTracker::new();
		bt.add(1);
		bt.add(2);
		bt.remove(2);
		assert_eq!(1, bt.total())
	}
}



/// An object which tracks non-stacking bonus values.
struct NonStackingTracker {
	tracker: Vec<u8>,
}

impl NonStackingTracker {
	fn new() -> NonStackingTracker {
		NonStackingTracker{
			tracker: Vec::new()
		}
	}
}

impl BonusTracker for NonStackingTracker {
	fn total(&self) -> u8 {
		if let Some(t) = self.tracker.iter().max() {
			return t.clone();
		}
		return 0;
	}

	fn add(&mut self, amt: u8) -> &mut NonStackingTracker {
		self.tracker.push(amt);
		return self;
	}

	fn remove(&mut self, amt: u8) -> &mut NonStackingTracker {
		if let Some(index) = self.tracker.iter().position(|&i| i == amt) {
		    self.tracker.remove(index);
		}
		return self;
	}
}

#[cfg(test)]
mod nonstackingtrackertests {
	use super::*;
	use bonuses;

	#[test]
	fn total_default() {
		// tests to make sure the default value is 0
		let bt = bonuses::NonStackingTracker::new();
		assert_eq!(0, bt.total())
	}

	#[test]
	fn add_1() {
		// tests to make sure a single value makes a round trip
		let mut bt = bonuses::NonStackingTracker::new();
		bt.add(1);
		assert_eq!(1, bt.total())
	}

	#[test]
	fn add_1_1() {
		// only the greatest value should be returned
		let mut bt = bonuses::NonStackingTracker::new();
		bt.add(1);
		bt.add(1);
		assert_eq!(1, bt.total())
	}

	#[test]
	fn add_1_2() {
		// only the greatest value should be returned
		let mut bt = bonuses::NonStackingTracker::new();
		bt.add(1);
		bt.add(2);
		assert_eq!(2, bt.total())
	}

	#[test]
	fn add_2_1() {
		// only the greatest value should be returned
		let mut bt = bonuses::NonStackingTracker::new();
		bt.add(2);
		bt.add(1);
		assert_eq!(2, bt.total())
	}

	#[test]
	fn remove_default() {
		// removing a nonexistant value should not panic
		let mut bt = bonuses::NonStackingTracker::new();
		bt.remove(1);
	}

	#[test]
	fn remove_1() {
		let mut bt = bonuses::NonStackingTracker::new();
		bt.add(1);
		bt.remove(1);
		assert_eq!(0, bt.total())
	}

	#[test]
	fn remove_1_1() {
		let mut bt = bonuses::NonStackingTracker::new();
		bt.add(1);
		bt.add(1);
		bt.remove(1);
		assert_eq!(1, bt.total())
	}

	#[test]
	fn remove_1_2() {
		let mut bt = bonuses::NonStackingTracker::new();
		bt.add(1);
		bt.add(2);
		bt.remove(2);
		assert_eq!(1, bt.total())
	}
}