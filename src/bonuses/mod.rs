pub use self::alchemical_bonus::{ AlchemicalBonus };
pub use self::armor_bonus::{ ArmorBonus };
pub use self::circumstance_bonus::{ CircumstanceBonus };
pub use self::competence_bonus::{ CompetenceBonus };
pub use self::deflection_bonus::{ DeflectionBonus };
pub use self::dodge_bonus::{ DodgeBonus };
pub use self::enhancement_bonus::{ EnhancementBonus };
pub use self::inherent_bonus::{ InherentBonus };
pub use self::insight_bonus::{ InsightBonus };
pub use self::luck_bonus::{ LuckBonus };
pub use self::morale_bonus::{ MoraleBonus };
pub use self::natural_armor_bonus::{ NaturalArmorBonus };
pub use self::natural_armor_enhancement_bonus::{ NaturalArmorEnhancementBonus };
pub use self::profane_bonus::{ ProfaneBonus };
pub use self::racial_bonus::{ RacialBonus };
pub use self::resistance_bonus::{ ResistanceBonus };
pub use self::sacred_bonus::{ SacredBonus };
pub use self::shield_bonus::{ ShieldBonus };
pub use self::size_bonus::{ SizeBonus };
pub use self::trait_bonus::{ TraitBonus };
pub use self::untyped_bonus::{ UntypedBonus };
pub use self::penalty::{ Penalty };
mod alchemical_bonus;
mod armor_bonus;
mod circumstance_bonus;
mod competence_bonus;
mod deflection_bonus;
mod dodge_bonus;
mod enhancement_bonus;
mod inherent_bonus;
mod insight_bonus;
mod luck_bonus;
mod morale_bonus;
mod natural_armor_bonus;
mod natural_armor_enhancement_bonus;
mod profane_bonus;
mod racial_bonus;
mod resistance_bonus;
mod sacred_bonus;
mod shield_bonus;
mod size_bonus;
mod trait_bonus;
mod untyped_bonus;
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