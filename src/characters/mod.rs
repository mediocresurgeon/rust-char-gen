use ability_scores::AbilityScore;
use ability_scores::Charisma;
use ability_scores::Constitution;
use ability_scores::Dexterity;
use ability_scores::Intelligence;
use ability_scores::Strength;
use ability_scores::Wisdom;
use ability_scores;


pub trait Character {
	fn get_level(&self) -> &u8;
    fn set_level(&mut self, u8) -> &mut Self where Self: Sized;

    fn get_strength(&mut self) -> &mut AbilityScore;
    fn get_dexterity(&mut self) -> &mut AbilityScore;
    fn get_constitution(&mut self) -> &mut AbilityScore;
    fn get_intelligence(&mut self) -> &mut AbilityScore;
    fn get_wisdom(&mut self) -> &mut AbilityScore;
    fn get_charisma(&mut self) -> &mut AbilityScore;
}


pub struct Humanoid {
    level: u8,
    strength: ability_scores::Strength,
    dexterity: ability_scores::Dexterity,
    constitution: ability_scores::Constitution,
    intelligence: ability_scores::Intelligence,
    wisdom: ability_scores::Wisdom,
    charisma: ability_scores::Charisma,
}

impl Humanoid {
    pub fn new() -> Humanoid {
        Humanoid {
            level: 1,
            strength: Strength::default(),
            dexterity: Dexterity::default(),
            constitution: Constitution::default(),
            intelligence: Intelligence::default(),
            wisdom: Wisdom::default(),
            charisma: Charisma::default(),
        }
    }
}

impl Character for Humanoid {
    /// Returns this character's level.
    /// Expected return values are 1-20 (inclusive).
    fn get_level(&self) -> &u8 {
        return &self.level;
    }

    /// Sets this character's level.
    /// Expected values are 1-20 (inclusive).
    fn set_level(&mut self, level: u8) -> &mut Humanoid {
        self.level = level;
        return self;
    }

    /// Returns this character's Strength ability score.
    fn get_strength(&mut self) -> &mut AbilityScore {
        &mut self.strength
    }

    /// Returns this character's Dexterity ability score.
    fn get_dexterity(&mut self) -> &mut AbilityScore {
        &mut self.dexterity
    }

    /// Returns this character's Constitution ability score.
    fn get_constitution(&mut self) -> &mut AbilityScore {
        &mut self.constitution
    }

    /// Returns this character's Intelligence ability score.
    fn get_intelligence(&mut self) -> &mut AbilityScore {
        &mut self.intelligence
    }

    /// Returns this character's Wisdom ability score.
    fn get_wisdom(&mut self) -> &mut AbilityScore {
        &mut self.wisdom
    }

    /// Returns this character's Charisma ability score.
    fn get_charisma(&mut self) -> &mut AbilityScore {
        &mut self.charisma
    }
}