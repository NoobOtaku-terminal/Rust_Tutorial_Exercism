#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,         // 1
    Peanuts,      // 2
    Shellfish,    // 4
    Strawberries, // 8
    Tomatoes,     // 16
    Chocolate,    // 32
    Pollen,       // 64
    Cats,         // 128
}

pub struct Allergies {
    score: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            score: score & 255, // Mask out irrelevant bits (only 8 bits)
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & Self::allergen_score(*allergen) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let all_allergens = [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];
        all_allergens
            .iter()
            .cloned()
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }

    fn allergen_score(allergen: Allergen) -> u32 {
        match allergen {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        }
    }
}
