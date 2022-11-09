use crate::{
    color::Color,
    console,
    date::Date,
    difficulty::Difficulty,
    graphics, input,
    player::Player,
    settings::Settings,
    text::{StatusBar, TextColor},
};
use rand::Rng;
use std::io::{Result, Stdout};

pub struct State {
    rng: rand::rngs::ThreadRng,
    settings: Settings,
    difficulty: Difficulty,
    miles_left: i32,
    days_left: i32,
    date: Date,
    alerts: Vec<Alert>,
    player: Player,
}

impl State {
    pub fn new(stdout: &Stdout) -> Result<State> {
        let name = input::name(stdout)?;
        let difficulty = input::difficulty(&stdout)?;

        Ok(State {
            rng: rand::thread_rng(),
            settings: Settings::new(std::env::args()),
            difficulty,
            date: Date::new(3, 1, 1848),
            miles_left: difficulty.start_miles,
            days_left: difficulty.start_days,
            alerts: Vec::new(),
            player: Player::new(name, difficulty),
        })
    }
}

impl State {
    /// Travel command
    pub fn travel(&mut self) -> Option<ExitStatus> {
        StateModifier {
            miles_traveled: self.rng.gen_range(15..=45),
            days_took: self.rng.gen_range(3..=7),
            health_shift: 0,
            food_shift: self.rng.gen_range(-10..=-5),
            stamina_shift: self.rng.gen_range(-15..=-5),
            moral_shift: self.rng.gen_range(-10..=0),
        }
        .turn(self)
    }

    /// Rest command
    pub fn rest(&mut self) -> Option<ExitStatus> {
        StateModifier {
            miles_traveled: 0,
            days_took: self.rng.gen_range(2..=5),
            health_shift: 0,
            food_shift: 0,
            stamina_shift: self.rng.gen_range(8..=15),
            moral_shift: self.rng.gen_range(2..=3),
        }
        .turn(self)
    }

    /// Eat command
    pub fn eat(&mut self) -> Option<ExitStatus> {
        StateModifier {
            miles_traveled: 0,
            days_took: self.rng.gen_range(1..=2),
            health_shift: 0,
            food_shift: self.rng.gen_range(-8..=-15),
            stamina_shift: self.rng.gen_range(1..=2),
            moral_shift: self.rng.gen_range(2..=3),
        }
        .turn(self)
    }

    /// Activity command
    pub fn activity(&mut self, stdout: &Stdout) -> Option<ExitStatus> {
        // menu
        let player_action = input::activity(stdout).unwrap();
        // action
        match player_action.as_str() {
            "sing" | "s" => StateModifier {
                miles_traveled: 0,
                days_took: self.rng.gen_range(1..=2),
                health_shift: 0,
                food_shift: 0,
                stamina_shift: self.rng.gen_range(-5..=-2),
                moral_shift: self.rng.gen_range(3..=8),
            }
            .turn(self),
            "excercise" | "e" => StateModifier {
                miles_traveled: 0,
                days_took: self.rng.gen_range(1..=3),
                health_shift: 0,
                food_shift: 0,
                stamina_shift: self.rng.gen_range(-7..=-5),
                moral_shift: self.rng.gen_range(3..=15),
            }
            .turn(self),
            _ => {
                graphics::bar1();
                println!("{}", "[Not a valid activity!]".rgb(255, 0, 0));
                graphics::bar1();
                console::pause().expect("Console: Could not pause terminal!");
                None
            }
        }
    }

    /// Forage command
    pub fn forage(&mut self, stdout: &Stdout) -> Option<ExitStatus> {
        // menu
        let player_action = input::forage(stdout).unwrap();
        // action
        match player_action.as_str() {
            "hunt" | "h" => StateModifier {
                miles_traveled: 0,
                days_took: self.rng.gen_range(10..=15),
                health_shift: 0,
                food_shift: self.rng.gen_range(50..=75),
                stamina_shift: self.rng.gen_range(-15..=-10),
                moral_shift: 0,
            }
            .turn(self),
            "fish" | "f" => StateModifier {
                miles_traveled: 0,
                days_took: self.rng.gen_range(5..=10),
                health_shift: 0,
                food_shift: self.rng.gen_range(25..=50),
                stamina_shift: self.rng.gen_range(-10..=-5),
                moral_shift: 0,
            }
            .turn(self),
            "dig" | "d" => StateModifier {
                miles_traveled: 0,
                days_took: self.rng.gen_range(1..=5),
                health_shift: 0,
                food_shift: self.rng.gen_range(5..=25),
                stamina_shift: self.rng.gen_range(-5..=-2),
                moral_shift: 0,
            }
            .turn(self),
            "search" | "s" => {
                // choose material
                let found_material = Materials::random(&mut self.rng);
                let material_amount = self.rng.gen_range(0..=3);

                // no materials
                if material_amount == 0 {
                    console::alert("No materials found!");
                    return None;
                }

                // add materials
                self.player
                    .backpack
                    .entry(found_material)
                    .and_modify(|amount| *amount += material_amount)
                    .or_insert(material_amount);

                // action
                return StateModifier {
                    miles_traveled: 0,
                    days_took: self.rng.gen_range(5..=10),
                    health_shift: 0,
                    food_shift: 0,
                    stamina_shift: self.rng.gen_range(-10..=-5),
                    moral_shift: 0,
                }
                .turn(self);
            }
            _ => {
                console::alert("Not a valid action!");
                None
            }
        }
    }

    pub fn medbay(&mut self, stdout: &Stdout) -> Option<ExitStatus> {
        // menu
        let player_action = input::medbay(stdout).unwrap();
        // action
        match player_action.as_str() {
            "m" => {
                if let Some(medkits) = self.player.backpack.get_mut(&Materials::Medkits) {
                    if medkits != &mut 0 {
                        *medkits -= 1;
                        return StateModifier {
                            miles_traveled: 0,
                            days_took: self.rng.gen_range(1..=3),
                            health_shift: self.rng.gen_range(15..=20),
                            food_shift: 0,
                            stamina_shift: self.rng.gen_range(-5..=-1),
                            moral_shift: 0,
                        }
                        .turn(self);
                    } else {
                        // no medkits
                        console::alert("No medkits left!");
                        None
                    }
                } else {
                    // no medkits
                    console::alert("No medkits left!");
                    None
                }
            }
            "p" => {
                if let Some(potions) = self.player.backpack.get_mut(&Materials::Potions) {
                    if potions != &mut 0 {
                        *potions -= 1;
                        self.player.effects.clear();
                        return StateModifier {
                            miles_traveled: 0,
                            days_took: self.rng.gen_range(1..=2),
                            health_shift: 0,
                            food_shift: 0,
                            stamina_shift: 0,
                            moral_shift: 0,
                        }
                        .turn(self);
                    } else {
                        // no potions
                        console::alert("No potions left!");
                        None
                    }
                } else {
                    // no potions
                    console::alert("No potions left!");
                    None
                }
            }
            "b" => {
                if let Some(bandaids) = self.player.backpack.get_mut(&Materials::Bandaids) {
                    if self.player.bleeding {
                        if bandaids != &mut 0 {
                            *bandaids -= 1;
                            self.player.bleeding = false;
                            return StateModifier {
                                miles_traveled: 0,
                                days_took: self.rng.gen_range(0..=1),
                                health_shift: 0,
                                food_shift: 0,
                                stamina_shift: 0,
                                moral_shift: 0,
                            }
                            .turn(self);
                        } else {
                            // no bandaids
                            console::alert("No bandaids left!");
                            None
                        }
                    } else {
                        // not bleeding
                        console::alert("You are not bleeding!");
                        None
                    }
                } else {
                    // no bandaids
                    console::alert("No bandaids left!");
                    None
                }
            }
            _ => {
                console::alert("Not a valid aid!");
                None
            }
        }
    }

    /// Status: display status of player and game state
    pub fn status(&self) -> Option<ExitStatus> {
        let miles_color = Color::percentage_flip(self.miles_left, self.difficulty.start_miles);
        let miles_left = format!("{}", self.miles_left).color(miles_color);

        let days_color = Color::percentage(self.days_left, self.difficulty.start_days);
        let days_left = format!("{}", self.days_left).color(days_color);

        let date = format!("{}", self.date.display()).color(days_color);

        println!("{}", "-:Status:-".color(Color::BAR_YELLOW));

        println!(
            "Miles Left: {}    Days Left: {}    Date: {}",
            miles_left, days_left, date,
        );

        let health_bar = "♥".status_bar(self.player.health, self.difficulty.start_health, 10);
        let stamina_bar = "■".status_bar(self.player.stamina, self.difficulty.start_stamina, 10);
        let moral_bar = "■".status_bar(self.player.moral, self.difficulty.start_moral, 10);
        let food_bar = "■".status_bar(self.player.food, self.difficulty.start_food, 10);

        println!("Health:  {}", health_bar);
        println!("Stamina: {}", stamina_bar);
        println!("Moral:   {}", moral_bar);
        println!("Food:    {}", food_bar);

        // backpack
        if !self.player.backpack.is_empty() {
            if self.player.backpack.len() == 1 {
                print!("Backpack: ");
            } else {
                println!("Backpack: ");
            }

            for (material, amount) in &self.player.backpack {
                println!(
                    "{} bundles of {}",
                    amount.rgb(250, 128, 114),
                    material.rgb(206, 141, 216),
                );
            }
        }

        graphics::bar1();

        None
    }

    /// Help command
    pub fn help(&mut self, stdout: &Stdout) -> Option<ExitStatus> {
        console::clear(&stdout).expect("Console: Could not clear terminal!");
        graphics::commands();
        console::pause().expect("Console: Could not pause terminal!");
        None
    }

    /// Quit command
    pub fn quit(&mut self) -> Option<ExitStatus> {
        print!("{}", graphics::BAR1.rgb(255, 215, 0));
        Some(ExitStatus::Quit)
    }

    /// Alerts: display all alerts
    pub fn alerts(&mut self) {
        if !self.alerts.is_empty() {
            for alert in &self.alerts {
                match &alert {
                    Alert::NewMonth => graphics::new_month(),
                    Alert::NewYear => graphics::new_year(),
                }
            }
            self.alerts.clear();
        }
    }
}

pub struct StateModifier {
    miles_traveled: i32,
    days_took: i32,
    health_shift: i32,
    food_shift: i32,
    stamina_shift: i32,
    moral_shift: i32,
}

impl StateModifier {
    pub fn turn(self, state: &mut State) -> Option<ExitStatus> {
        // miles
        state.miles_left -= self.miles_traveled;
        if state.miles_left <= 0 {
            return Some(ExitStatus::Win);
        }
        // days
        state.days_left -= self.days_took;
        if state.days_left <= 0 {
            return Some(ExitStatus::TimeDeath);
        }
        // date
        if let Some(alert) = state.date.add_days(self.days_took) {
            state.alerts.push(alert);
        };
        // health
        state.player.health += self.health_shift;
        if state.player.health <= 0 {
            return Some(ExitStatus::HealthDeath);
        }
        // food
        state.player.food += self.food_shift;
        if state.player.food <= 0 {
            return Some(ExitStatus::FoodDeath);
        }
        // stamina
        state.player.stamina += self.stamina_shift;
        if state.player.stamina <= 0 {
            return Some(ExitStatus::StaminaDeath);
        }
        // moral
        state.player.moral += self.moral_shift;
        if state.player.moral <= 0 {
            return Some(ExitStatus::MoralityDeath);
        }
        // funny
        if state.rng.gen_range(0..=1000) == 500 {
            return Some(ExitStatus::FunnyDeath);
        }

        None
    }
}

#[derive(PartialEq, Debug)]
pub enum ExitStatus {
    Win,
    TimeDeath,
    HealthDeath,
    FoodDeath,
    StaminaDeath,
    MoralityDeath,
    FunnyDeath,
    Quit,
}

pub enum Alert {
    NewMonth,
    NewYear,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Materials {
    Medkits,
    Potions,
    Bandaids,
}

impl Materials {
    pub fn random(rng: &mut rand::rngs::ThreadRng) -> Materials {
        // match rng.gen_range(0..=2) { // rand 0, 1, 2
        match rng.gen_range(0..=2) {
            0 => Materials::Medkits,
            1 => Materials::Potions,
            _ => Materials::Bandaids,
        }
    }
}
