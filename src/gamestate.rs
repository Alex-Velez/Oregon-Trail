use crate::{color, console, graphics, random, Date, Flags, TextColor};
use std::io::Result;

/// Game state including player status
pub struct GameState {
    name: String,
    miles_left: u32,
    days_left: u32,
    date: Date,
    health: f32,
    stamina: f32,
    moral: f32,
    food: f32,
    water: f32,
    medicine: f32,
    effects: Vec<Effect>,
    difficulty: Difficulty,
    max: Max,
    pub exit_status: ExitStatus,
}

impl GameState {
    /// Creates a custom game state
    fn custom(
        miles_left: u32,
        days_left: u32,
        health: f32,
        stamina: f32,
        moral: f32,
        food: f32,
        water: f32,
        medicine: f32,
    ) -> GameState {
        GameState {
            name: String::new(),
            miles_left,
            days_left,
            date: Date::new(3, 1, 1843),
            health,
            stamina,
            moral,
            food,
            water,
            medicine,
            effects: Vec::new(),
            difficulty: Difficulty::None,
            exit_status: ExitStatus::None,
            max: Max {
                miles: miles_left,
                days: days_left,
                health,
                stamina,
                moral,
                food,
                water,
                medicine,
            },
        }
    }

    /// Travel command
    pub fn travel(&mut self) {
        let seed = random::system_time();
        self.miles_left -= random::range(seed, 30, 60);
        self.days_left -= random::range(seed, 3, 7);
        self.stamina -= random::range(seed, 3, 10) as f32;
        self.moral -= random::range(seed, 3, 10) as f32;
    }

    /// Rest command
    pub fn rest(&mut self) {
        let seed = random::system_time();
        self.days_left -= random::range(seed, 2, 5);
        self.stamina += random::range(seed, 5, 10) as f32;
        self.moral += random::range(seed, 0, 5) as f32;
    }

    /// Eat command
    pub fn eat(&mut self) {
        let seed = random::system_time();
        self.days_left -= random::range(seed, 2, 5);
        self.stamina += random::range(seed, 0, 5) as f32;

        if self.food <= 0.0 {
            self.moral -= random::range(seed, 2, 8) as f32;
        } else {
            self.health += random::range(seed, 0, 3) as f32;
            self.moral += random::range(seed, 0, 5) as f32;
            self.food -= random::range(seed, 2, 10) as f32;
        }
    }

    /// Status command, display status of player and game state
    pub fn status(&self) {
        println!(
            "Miles Left: {}    Days Left: {}    Date: {}",
            match self.miles_left as f32 / self.max.miles as f32 {
                y if y <= 0.25 => format!("{}", self.miles_left).rgb(200, 50, 100),
                y if y <= 0.50 => format!("{}", self.miles_left).rgb(200, 100, 50),
                y if y <= 0.75 => format!("{}", self.miles_left).rgb(200, 150, 50),
                _ => format!("{}", self.miles_left).rgb(150, 250, 0),
            },
            match self.days_left as f32 / self.max.days as f32 {
                y if y <= 0.25 => format!("{}", self.days_left).rgb(200, 50, 100),
                y if y <= 0.50 => format!("{}", self.days_left).rgb(200, 100, 50),
                y if y <= 0.75 => format!("{}", self.days_left).rgb(200, 150, 50),
                _ => format!("{}", self.days_left).rgb(150, 250, 0),
            },
            match self.date.to_days() as f32 / Date::new(3, 1, 1843).to_days() as f32 {
                y if y <= 0.25 => format!("{}", self.date.display()).rgb(200, 50, 100),
                y if y <= 0.50 => format!("{}", self.date.display()).rgb(200, 100, 50),
                y if y <= 0.75 => format!("{}", self.date.display()).rgb(200, 150, 50),
                _ => format!("{}", self.date.display()).rgb(150, 250, 0),
            },
        );
        status_bar("    Health  ", '♥', self.health, self.max.health);
        status_bar("    Stamina ", '#', self.stamina, self.max.stamina);
        status_bar("    Moral   ", '%', self.moral, self.max.moral);
        status_bar("    Food    ", '*', self.food, self.max.food);
        status_bar("    Water   ", '~', self.water, self.max.water);
        status_bar("    Medicine", '+', self.medicine, self.max.medicine);
        if !self.effects.is_empty() {
            println!(
                "    {} {}",
                "Effects:".rgb(135, 206, 250),
                format!("{:?}", self.effects).rgb(250, 50, 50)
            );
        }
    }

    /// Help command, display commands
    pub fn help(&self) {
        println!("{}", graphics::COMMANDS.rgb(200, 200, 200));
    }

    /// Credits command, display credits
    pub fn credits(&self) {
        println!("{}", graphics::CREDITS.rgb(200, 200, 200));
    }

    /// Check game state for game over
    pub fn check_over(&self) -> bool {
        true
    }
}

/// Wrapper struct for holding max values of game state
struct Max {
    miles: u32,
    days: u32,
    health: f32,
    stamina: f32,
    moral: f32,
    food: f32,
    water: f32,
    medicine: f32,
}

/// Game difficulty
enum Difficulty {
    None,
    Easy,
    Medium,
    Hard,
    Impossible,
    Custom,
}

/// Player status effects
#[derive(Debug)]
enum Effect {
    Poisoned,
    Fatigued,
    Hungry,
    Thirsty,
    Injured,
    Bleeding,
}

/// Game exit status
pub enum ExitStatus {
    None,
    Quit,
    Die,
    Win,
}

/// Create game state with set difficulty
pub fn init(stdout: &std::io::Stdout, flags: Flags) -> Result<GameState> {
    if !flags.quick_start {
        let name = 'name: loop {
            print!("What is your name traveller?: ");
            color::set_text_color(&stdout, 206, 141, 216)?;
            let user = console::read_line()?;
            color::clear_text_color(&stdout)?;
            if !user.is_empty() {
                break 'name user;
            } else {
                print!("{} ", "[enter a name]".rgb(255, 34, 34));
            }
        };

        println!("Which gamemode would you like to play in?");
        println!(
            "    {}    {}    {}    {}    {}\n",
            "[e] Easy".rgb(50, 205, 50),
            "[m] Medium".rgb(255, 205, 0),
            "[h] Hard".rgb(220, 20, 60),
            "[i] Impossible".rgb(148, 0, 211),
            "[c] Custom".rgb(135, 206, 250),
        );

        Ok({
            let game = 'game_state: loop {
                print!("{}", "==>: ".rgb(255, 222, 173));
                color::set_text_color(&stdout, 206, 141, 216)?;
                let input: &str = &console::read_line()?.to_lowercase();
                color::clear_text_color(&stdout)?;
                match input {
                    "e" | "easy" => {
                        let mut gamestate =
                            GameState::custom(500, 250, 100.0, 100.0, 100.0, 200.0, 200.0, 15.0);
                        gamestate.name = name;
                        gamestate.difficulty = Difficulty::Easy;
                        break 'game_state gamestate;
                    }
                    "m" | "medium" => {
                        let mut gamestate =
                            GameState::custom(1000, 306, 75.0, 75.0, 75.0, 150.0, 150.0, 10.0);
                        gamestate.name = name;
                        gamestate.difficulty = Difficulty::Medium;
                        break 'game_state gamestate;
                    }
                    "h" | "hard" => {
                        let mut gamestate =
                            GameState::custom(2000, 365, 50.0, 50.0, 50.0, 100.0, 100.0, 5.0);
                        gamestate.name = name;
                        gamestate.difficulty = Difficulty::Hard;
                        break 'game_state gamestate;
                    }
                    "i" | "impossible" => {
                        let mut gamestate =
                            GameState::custom(3000, 406, 50.0, 50.0, 50.0, 50.0, 50.0, 3.0);
                        gamestate.name = name;
                        gamestate.difficulty = Difficulty::Impossible;
                        break 'game_state gamestate;
                    }
                    "c" | "custom" => {
                        let mut gamestate = GameState::custom(
                            num_input(&stdout, &"    Miles: ".rgb(135, 206, 250))?,
                            num_input(&stdout, &"    Days: ".rgb(135, 206, 250))?,
                            num_input(&stdout, &"    Health: ".rgb(135, 206, 250))?,
                            num_input(&stdout, &"    Stamina: ".rgb(135, 206, 250))?,
                            num_input(&stdout, &"    Moral: ".rgb(135, 206, 250))?,
                            num_input(&stdout, &"    Food: ".rgb(135, 206, 250))?,
                            num_input(&stdout, &"    Water: ".rgb(135, 206, 250))?,
                            num_input(&stdout, &"    Medicine: ".rgb(135, 206, 250))?,
                        );
                        gamestate.name = name;
                        gamestate.difficulty = Difficulty::Custom;
                        break 'game_state gamestate;
                    }
                    _ => print!("{} ", "[not a valid difficulty]".rgb(255, 34, 34)),
                }
            };
            println!(
                "Are you ready for your journey, {}?\n",
                game.name.rgb(206, 141, 216)
            );
            color::set_text_color(&stdout, 206, 141, 216)?;
            console::pause(&stdout)?;
            color::clear_text_color(&stdout)?;
            game
        })
    } else {
        Ok(GameState::custom(
            500, 250, 100.0, 100.0, 100.0, 200.0, 200.0, 15.0,
        ))
    }
}

/// Wrapper function for getting an f32 value from user
fn num_input<T: std::str::FromStr>(stdout: &std::io::Stdout, name: &str) -> Result<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    Ok('custom_value: loop {
        print!("{}", name);
        color::set_text_color(&stdout, 206, 141, 216)?;
        let custom_state_value = console::read_line()?.parse::<T>();
        color::clear_text_color(&stdout)?;
        match custom_state_value {
            Ok(val) => break 'custom_value val,
            Err(e) => print!("{}", format!("[{:?}] ", e).rgb(255, 34, 34)),
        }
    })
}

/// Display a value bar for the given value
fn status_bar(name: &str, icon: char, value: f32, max: f32) {
    print!(
        "{}",
        format!("{} [{}]: ", name, value as u32).rgb(135, 206, 250)
    );
    let percent = value / max;
    for x in 0u8..10u8 {
        if f32::from(x) <= (percent * 10.0) {
            match percent {
                y if y <= 0.25 => print!("{}", icon.rgb(200, 50, 100)),
                y if y <= 0.50 => print!("{}", icon.rgb(200, 100, 50)),
                y if y <= 0.75 => print!("{}", icon.rgb(200, 150, 50)),
                _ => print!("{}", icon.rgb(150, 200, 50)),
            }
        } else {
            print!("{}", icon.rgb(192, 192, 192));
        }
    }
    println!();
}
