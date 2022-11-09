#![allow(dead_code)]
use std::io::Result;

mod color;
mod console;
mod date;
mod difficulty;
mod graphics;
mod input;
mod player;
mod settings;
mod state;
mod text;

use text::TextColor;

use crate::state::ExitStatus;

fn main() -> Result<()> {
    // Start timer
    let time = std::time::Instant::now();

    // Setup terminal
    let stdout = std::io::stdout();
    #[cfg(target_os = "windows")]
    console::enable_color(&stdout)?;
    console::clear(&stdout)?;

    // Set window title
    print!("\x1B]0;{}\x07", "Oregon Trail");

    // title screen
    graphics::title();

    // init state
    let mut state = state::State::new(&stdout)?;

    console::pause().expect("Console: Could not pause terminal!");

    // command screen
    console::clear(&stdout)?;
    graphics::commands();

    console::pause().expect("Console: Could not pause terminal!");

    // Main game loop
    let exit_status: ExitStatus = 'game: loop {
        // reset screen
        console::clear(&stdout)?;
        graphics::logo();
        graphics::bar1();
        state.status();

        // print alerts if any
        state.alerts();

        // get user action
        println!("What will you do next?");
        print!("{}", "> ".rgb(255, 222, 173));
        let action = console::read_line()?.trim().to_lowercase();

        let tick = match action.as_str() {
            "t" | "travel" => state.travel(),
            "r" | "rest" => state.rest(),
            "e" | "eat" => state.eat(),
            "a" | "activity" => state.activity(&stdout),
            "f" | "forage" => state.forage(&stdout),
            "m" | "medbay" => state.medbay(&stdout),
            "h" | "help" | "hc" => state.help(&stdout),
            "cr" | "credits" => None,
            "q" | "quit" => state.quit(),
            _ => {
                console::alert("Not a valid action!");
                None
            }
        };

        if let Some(exit) = tick {
            break 'game exit;
        }
    };

    // Exit
    console::clear(&stdout)?;
    match exit_status {
        ExitStatus::Win => graphics::win(),
        ExitStatus::TimeDeath => graphics::time_death(),
        ExitStatus::HealthDeath => graphics::health_death(),
        ExitStatus::FoodDeath => graphics::food_death(),
        ExitStatus::StaminaDeath => graphics::stamina_death(),
        ExitStatus::MoralityDeath => graphics::morality_death(),
        ExitStatus::FunnyDeath => graphics::food_death(),
        ExitStatus::Quit => graphics::quit(),
    }

    println!(
        "{} {:?}",
        "Succesfully exited in".rgb(175, 238, 238),
        time.elapsed()
    );
    console::pause().expect("Console: Could not pause terminal!");
    Ok(())
}
