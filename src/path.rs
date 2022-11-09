pub mod audio {
    pub const THEME1: &str = "audio/Ecstacy of 8-bit Gold - Timmer.mp3";
    pub const THEME2: &str = "audio/Fastest Gun in the 8-bit West - OkamiDeluxe.mp3";
    pub const GAME_OVER: &str = "audio/Game Over - MB Music.mp3";
}

pub mod embedded {
    pub const THEME1: &[u8] = include_bytes!("../audio/Ecstacy of 8-bit Gold - Timmer.mp3");
    pub const THEME2: &[u8] =
        include_bytes!("../audio/Fastest Gun in the 8-bit West - OkamiDeluxe.mp3");
    pub const GAME_OVER: &[u8] = include_bytes!("../audio/Game Over - MB Music.mp3");
}

pub mod graphics {
    pub const TITLE: &str = "ascii/title.txt";
    pub const CREDITS: &str = "ascii/credits.txt";
    pub const DESCRIPTION: &str = "ascii/description.txt";
    pub const GAME_LOSE: &str = "ascii/game_lose.txt";
    pub const GAME_QUIT: &str = "ascii/game_quit.txt";
    pub const GAME_WIN: &str = "ascii/game_win.txt";
}
