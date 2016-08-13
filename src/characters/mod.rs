pub trait Character {
	fn get_level(&self) -> &u8;
    fn set_level(&mut self, u8) -> &mut Self;
}


pub struct Humanoid {
    level: u8,
}

impl Humanoid {
    pub fn new() -> Humanoid {
        Humanoid {
            level: 1
        }
    }
}

impl Character for Humanoid {
    fn get_level(&self) -> &u8 {
        return &self.level;
    }

    fn set_level(&mut self, level: u8) -> &mut Humanoid {
        self.level = level;
        return self;
    }
}