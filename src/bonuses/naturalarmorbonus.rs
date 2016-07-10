use bonuses;

/// An object which tracks natural armor bonus values.
pub struct NaturalArmorBonus {
	tracker: bonuses::NonStackingTracker,
}

impl NaturalArmorBonus {
	/// Create an instance of NaturalArmorBonus.
	pub fn new() -> NaturalArmorBonus {
		NaturalArmorBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for NaturalArmorBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut NaturalArmorBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut NaturalArmorBonus {
		self.tracker.remove(amt);
		return self;
	}
}