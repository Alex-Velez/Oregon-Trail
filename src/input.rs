use crate::{
    console,
    difficulty::Difficulty,
    graphics,
    text::{self, TextColor},
};
use std::io::{Result, Stdout};

/// Dialouge for player name
pub fn name(stdout: &Stdout) -> Result<String> {
    print!("What is your name traveller?: ");
    text::set_text_color(&stdout, 206, 141, 216)?;
    let name = console::read_line()?;
    text::clear_text_color(&stdout)?;
    println!();

    Ok(name)
}

/// Dialouge for difficulty option
pub fn difficulty(stdout: &Stdout) -> Result<Difficulty> {
    // difficulty
    print!(
        "    {} {} {} {}",
        "[Easy]".rgb(100, 200, 100),
        "[Medium]".rgb(200, 200, 100),
        "[Hard]".rgb(200, 100, 100),
        "[Impossible]".rgb(255, 50, 50),
    );
    println!("\n");
    print!("Which gamemode would you like to play in?: ");
    text::set_text_color(stdout, 206, 141, 216)?;

    // input loop
    let difficulty = loop {
        let diff = match console::read_line()?.to_lowercase().as_str() {
            "e" | "easy" => Some(Difficulty::EASY),
            "m" | "medium" => Some(Difficulty::MEDIUM),
            "h" | "hard" => Some(Difficulty::HARD),
            "i" | "impossible" => Some(Difficulty::IMPOSSIBLE),
            _ => None,
        };

        if let Some(difficulty) = diff {
            break difficulty;
        } else {
            print!("{}", "[Not a valid difficulty!]: ".rgb(255, 0, 0));
        }
    };

    text::clear_text_color(stdout)?;
    println!();

    // difficulty reprint
    match difficulty {
        Difficulty::EASY => print!("    {}", "[Easy]".rgb(100, 200, 100)),
        Difficulty::MEDIUM => print!("    {}", "[Medium]".rgb(200, 200, 100)),
        Difficulty::HARD => print!("    {}", "[Hard]".rgb(200, 100, 100)),
        Difficulty::IMPOSSIBLE => print!("    {}", "[Impossible]".rgb(255, 50, 50)),
        _ => println!("{}", "[ERROR]: no difficulty recorded!".rgb(255, 0, 0)),
    };
    println!("\n");

    Ok(difficulty)
}

/// Dialouge for activity actions
pub fn activity(stdout: &Stdout) -> Result<String> {
    graphics::bar1();
    let letters: Vec<String> = "se".chars().map(|c| c.rgb(102, 205, 255)).collect();
    println!("    [{}/{}]", letters[0], letters[1]);
    print!("What would you like to do?: ");
    text::set_text_color(&stdout, 206, 141, 216)?;
    let activity = console::read_line()?.trim().to_lowercase();
    text::clear_text_color(&stdout)?;
    println!();

    Ok(activity)
}

/// Dialouge for forage actions
pub fn forage(stdout: &Stdout) -> Result<String> {
    graphics::bar1();
    let letters: Vec<String> = "fhdcms".chars().map(|c| c.rgb(102, 205, 255)).collect();
    println!(
        "    [{}/{}/{}/{}/{}/{}]",
        letters[0], letters[1], letters[2], letters[3], letters[4], letters[5],
    );
    print!("What would you like to forage?: ");
    text::set_text_color(&stdout, 206, 141, 216)?;
    let forage = console::read_line()?.trim().to_lowercase();
    text::clear_text_color(&stdout)?;
    println!();

    Ok(forage)
}

/// Dialouge for medbay actions
pub fn medbay(stdout: &Stdout) -> Result<String> {
    graphics::bar1();
    let letters: Vec<String> = "mpb".chars().map(|c| c.rgb(102, 205, 255)).collect();
    println!("[{}/{}/{}]", letters[0], letters[1], letters[2]);
    print!("How do you heal yourself?: ");
    text::set_text_color(&stdout, 206, 141, 216)?;
    let medbay = console::read_line()?.trim().to_lowercase();
    text::clear_text_color(&stdout)?;
    println!();

    Ok(medbay)
}
