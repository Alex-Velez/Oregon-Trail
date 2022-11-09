use crate::{console, graphics, text::TextColor};
use std::io::{stdin, stdout, Error, ErrorKind, Result, Write};

/// User input function
pub fn read_line() -> Result<String> {
    //use std::io::{stdin, stdout, Result, Error, ErrorKind, Write};
    stdout().flush()?; // print!() does not flush :(
    let mut input = String::new();
    match stdin().read_line(&mut input)? {
        0 => Err(Error::new(
            ErrorKind::UnexpectedEof,
            "input reached eof unexpectedly",
        )),
        _ => {
            if input.ends_with('\n') {
                input.pop();
                #[cfg(target_os = "windows")]
                if input.ends_with('\r') {
                    input.pop();
                }
            }
            Ok(input)
        }
    }
}

/// Clear terminal output
pub fn clear(mut stdout: &std::io::Stdout) -> Result<()> {
    stdout.write("\x1B[2J\x1B[1;1H".as_bytes())?;
    stdout.write("\x1B[3J\x1B[1;1H".as_bytes())?;
    stdout.flush()?;
    Ok(())
}

/// Pause the terminal with `read_line`
pub fn pause() -> Result<()> {
    //use std::io::{stdin, stdout, Result, Error, ErrorKind, Write};
    stdout().write("Press enter to continue . . .".as_bytes())?;
    stdout().flush()?; // print!() does not flush :(
    let mut _input = String::from("\n");
    match stdin().read_line(&mut _input)? {
        0 => Err(Error::new(
            ErrorKind::UnexpectedEof,
            "input reached eof unexpectedly",
        )),
        _ => Ok(()),
    }
}

/// Pause the terminal with alert message
pub fn alert(message: &str) {
    graphics::bar1();
    println!("{}", format!("[{}]", message).rgb(255, 0, 0));
    graphics::bar1();
    console::pause().expect("Console: Could not pause terminal!");
}

/// Set console mode to enable terminal color
#[cfg(target_os = "windows")]
pub fn enable_color(mut stdout: &std::io::Stdout) -> Result<()> {
    use winapi::um::{consoleapi, processenv, winbase, wincon};
    unsafe {
        let handle = processenv::GetStdHandle(winbase::STD_OUTPUT_HANDLE);
        let mut cmode = 0;
        if consoleapi::GetConsoleMode(handle, &mut cmode) == 0 {
            stdout.write("[Could not retrive console mode!]".as_bytes())?;
            pause()?;
            std::process::exit(1);
        }
        if consoleapi::SetConsoleMode(
            handle,
            cmode | // wincon::ENABLE_PROCESSED_OUTPUT |
            wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING |
            wincon::ENABLE_WRAP_AT_EOL_OUTPUT,
        ) == 0
        {
            stdout.write("[Could not set console mode!]".as_bytes())?;
            pause()?;
            std::process::exit(1);
        }
    }
    Ok(())
}
