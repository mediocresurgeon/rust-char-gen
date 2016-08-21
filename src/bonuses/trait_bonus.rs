use bonuses;

/// An object which tracks trait bonus values.
pub struct TraitBonus {
	tracker: bonuses::NonStackingTracker,
}

impl TraitBonus {
	/// Create an instance of TraitBonus.
	pub fn new() -> TraitBonus {
		TraitBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for TraitBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut TraitBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut TraitBonus {
		self.tracker.remove(amt);
		return self;
	}
}