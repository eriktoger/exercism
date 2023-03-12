use self::Allergen::*;
pub struct Allergies {
    pub allergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}
const ALLERGEN_VEC: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        let binary: Vec<char> = format!("{:b}", score % 256).chars().rev().collect();
        let allergies = binary.iter().enumerate().fold(vec![], |mut acc, (idx, c)| {
            if *c == '1' {
                acc.push(ALLERGEN_VEC[idx].clone());
            }
            acc
        });

        Allergies { allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.iter().any(|a| a == allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
