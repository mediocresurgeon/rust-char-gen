use bonuses;

/// An object which tracks untyped bonus values.
pub struct UntypedBonus {
	tracker: bonuses::StackingTracker,
}

impl UntypedBonus {
	/// Create an instance of UntypedBonus.
	pub fn new() -> UntypedBonus {
		UntypedBonus {
			tracker: bonuses::StackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for UntypedBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut UntypedBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut UntypedBonus {
		self.tracker.remove(amt);
		return self;
	}
}