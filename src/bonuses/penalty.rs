use bonuses;

/// An object which tracks penalty values.
pub struct Penalty {
	tracker: bonuses::StackingTracker,
}

impl Penalty {
	/// Create an instance of Penalty.
	pub fn new() -> Penalty {
		Penalty {
			tracker: bonuses::StackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for Penalty {
	/// Returns the total penalty.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a penalty value.
	fn add(&mut self, amt: u8) -> &mut Penalty {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a penalty value.
	fn remove(&mut self, amt: u8) -> &mut Penalty {
		self.tracker.remove(amt);
		return self;
	}
}