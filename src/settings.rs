pub struct Settings {
    debug: bool,
    help: bool,
    quick_start: bool,
}

impl Settings {
    /// Create new state settings from args
    pub fn new(args: std::env::Args) -> Settings {
        let (mut debug, mut help, mut quick_start) = (false, false, false);

        // Scan args for enabling and disabling features
        for argument in args {
            match argument.as_str() {
                "--debug" | "--d" => debug = true,
                "--help" | "--h" => help = true,
                "--quick_start" | "--qs" => quick_start = true,
                _ => {}
            }
        }

        Settings {
            debug,
            help,
            quick_start,
        }
    }
}
