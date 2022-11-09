#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Difficulty {
    pub start_miles: i32,
    pub start_days: i32,
    pub start_health: i32,
    pub start_food: i32,
    pub start_stamina: i32,
    pub start_moral: i32,
}

impl Difficulty {
    pub const EASY: Difficulty = Difficulty {
        start_miles: 500,
        start_days: 250,
        start_health: 100,
        start_food: 200,
        start_stamina: 100,
        start_moral: 100,
    };

    pub const MEDIUM: Difficulty = Difficulty {
        start_miles: 1000,
        start_days: 306,
        start_health: 75,
        start_food: 150,
        start_stamina: 75,
        start_moral: 75,
    };

    pub const HARD: Difficulty = Difficulty {
        start_miles: 2000,
        start_days: 365,
        start_health: 50,
        start_food: 100,
        start_stamina: 50,
        start_moral: 50,
    };

    pub const IMPOSSIBLE: Difficulty = Difficulty {
        start_miles: 3000,
        start_days: 406,
        start_health: 50,
        start_food: 50,
        start_stamina: 50,
        start_moral: 50,
    };
}
