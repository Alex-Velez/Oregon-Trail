use crate::{color::Color, TextColor};

/// Print out horizontal divider
pub fn bar1() {
    print!("{}", BAR1.color(Color::BAR_YELLOW));
}

/// Print out Oregon Trial logo
pub fn logo() {
    println!("{}", TITLE.color(Color::BAR_YELLOW));
}

/// Print out game title
pub fn title() {
    print!(
        "{}",
        (String::from("Welcome to the:") + &" ".repeat(81) + ".").color(Color::BAR_YELLOW)
    ); //81 sp
    logo();
    bar1();
    print!("{}", DESCRIPTION.rgb(210, 105, 30));
    bar1();
}

/// New Month Alert
pub fn new_month() {
    println!("New Month!");
    bar1();
}

/// New Year Alert
pub fn new_year() {
    println!("New Year!");
    bar1();
}

/// Print out avaliable commands
pub fn commands() {
    bar1();
    println!(
        "{}
    {} {}
    {} {}
    {} {}
    {} {}
     ╠══{} {}
     ╚══{} {}
    {} {}
     ╠══{} {}
     ╠══{} {}
     ╠══{} {}
     ╚══{} {}
    {} {}
     ╠══{} {}
     ╠══{} {}
     ╚══{} {}",
        "Commands:".rgb(75, 138, 138),
        "[t]".rgb(175, 238, 238),
        "Travel: Travel onwards.".rgb(75, 138, 138),
        "[r]".rgb(175, 238, 238),
        "Rest: Take a quick rest to replenish your stamina.".rgb(75, 138, 138),
        "[e]".rgb(175, 238, 238),
        "Eat: Sit down and eat to replenish your health.".rgb(75, 138, 138),
        "[a]".rgb(175, 238, 238),
        "Activity: Do activities to cheer up.".rgb(75, 138, 138),
        "[s]".rgb(175, 238, 238),
        "Sing: Sing a shanty to help the blues.".rgb(75, 138, 138),
        "[e]".rgb(175, 238, 238),
        "Exercise: Exercise to pump up your spirits.".rgb(75, 138, 138),
        "[f]".rgb(175, 238, 238),
        "Forage: Forage for nearby materials.".rgb(75, 138, 138),
        "[h]".rgb(175, 238, 238),
        "Hunt: Hunt for wildlife to replenish food supplies.".rgb(75, 138, 138),
        "[f]".rgb(175, 238, 238),
        "Fish: Search for a nearby lake for some fresh water.".rgb(75, 138, 138),
        "[d]".rgb(175, 238, 238),
        "Dig: Forage for medicinal fruit and veggies.".rgb(75, 138, 138),
        "[s]".rgb(175, 238, 238),
        "Search: Search the nearby landscape for supplies.".rgb(75, 138, 138),
        "[m]".rgb(175, 238, 238),
        "Medbay: Heal yourself of any effects and damage.".rgb(75, 138, 138),
        "[m]".rgb(175, 238, 238),
        "Medkit: Heal your wounds.".rgb(75, 138, 138),
        "[p]".rgb(175, 238, 238),
        "Potion: Drink a special brew to remove negative effects.".rgb(75, 138, 138),
        "[b]".rgb(175, 238, 238),
        "Bandage: Stop any bleeding.".rgb(75, 138, 138),
    );
    print!("{}", BAR1.rgb(255, 215, 0));
}

pub const CMD_COMMANDS: &'static str = r#"Oregon trail on the command line!

    Enable/disable features for oregon trail:
    add a flag by appending '--' with the command

        'oregontrail --help --no_sound'

Currently available commands:
    --help : lists available commands (terminates program)
    --no_sound : disables sound effects and music
"#;
pub const BAR1: &'static str = "=================================================================================================\n";
pub const BAR2: &'static str = "#################################################################################################\n";
pub const TITLE: &'static str = r#"
.  ██████╗ ██████╗ ███████╗ ██████╗  ██████╗ ███╗   ██╗    ████████╗██████╗  █████╗ ██╗██╗      .
. ██╔═══██╗██╔══██╗██╔════╝██╔════╝ ██╔═══██╗████╗  ██║    ╚══██╔══╝██╔══██╗██╔══██╗██║██║      .
. ██║   ██║██████╔╝█████╗  ██║  ███╗██║   ██║██╔██╗ ██║       ██║   ██████╔╝███████║██║██║      .
. ██║   ██║██╔══██╗██╔══╝  ██║   ██║██║   ██║██║╚██╗██║       ██║   ██╔══██╗██╔══██║██║██║      .
. ╚██████╔╝██║  ██║███████╗╚██████╔╝╚██████╔╝██║ ╚████║       ██║   ██║  ██║██║  ██║██║███████╗ .
.  ╚═════╝ ╚═╝  ╚═╝╚══════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═══╝       ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚══════╝ .
"#;
pub const DESCRIPTION: &'static str = r#"
    - You are in New York City, and you need to travel to Oregon before the winter storm comes.
    - But be carefull because you have limited supplies.
    - You only have a few months to get to Oregon.
    - This trip is possible, but don't get comfortable because you have a long journey ahead!

"#;
pub const COMMANDS: &'static str = r#"
Commands:
[t] Travel: Travel onwards.
[r] Rest: Take a quick rest to replenish your stamina.
[e] Eat: Sit down and eat to replenish your health.
[a] Activity: Do activities to cheer up.
    [s] Sing: Sing a shanty to help the blues.
    [e] Exercise: Exercise to pump up your spirits.
[f] Forage: Forage for nearby materials.
    [h] Hunt: Hunt for wildlife to replenish food supplies.
    [f] Fish: Search for a nearby lake for some fresh water.
    [d] Dig: Forage for medicinal fruit and veggies.
    [t] Chop: Chop down some nearby trees for wood.
    [m] Mine: Mine a nearby boulder for stone.
    [s] Shovel: Dig some nearby gravel for flint.
[m] Medbay: Heal yourself of any effects and damage.
    [m] Medkit: Heal your wounds.
    [p] Potion: Drink a special brew to remove negative effects.
    [b] Bandage: Stop any bleeding.
"#;

pub const COMMANDS_EXT: &'static str = r#"
Commands:
Wandering:
    [t] Travel: Travel onwards.
    [r] Rest: Take a quick rest to replenish your stamina.
    [e] Eat: Sit down and eat to replenish your health.
    [a] Activity: Do activities to cheer up.
        [s] Sing: Sing a shanty to help the blues.
        [e] Exercise: Exercise to pump up your spirits.
    [f] Forage: Forage for nearby materials.
        [h] Hunt: Hunt for wildlife to replenish food supplies.
        [f] Fish: Search for a nearby lake for some fresh water.
        [d] Dig: Forage for medicinal fruit and veggies.
        [t] Chop: Chop down some nearby trees for  wood.
        [m] Mine: Mine a nearby boulder for stone.
        [s] Shovel: Dig some nearby gravel for flint.
    [m] Medbay: Heal yourself of any effects and damage.
        [m] Medkit: Heal your wounds.
        [p] Potion: Drink a special brew to remove negative effects.
        [b] Bandage: Stop any bleeding.
    [c] Craft: Craft materials and tools.
        [b] Bed: Faster and better quality sleep.
        [p] Pickaxe: For mining.
        [a] Axe: For chopping.
        [s] Shovel: For digging.
        [m] Medical: Medical tools.
            [m] Medkit: To heal wounds.
            [p] Potion: To heal effects.
            [b] Bandage: To stop bleeding.
    [b] Backpack: List all the items in your backpack.
Battle:
    [a] Attack: Attack enemies
        []
        []
        []
        []
    [r] Run: Flee the scene! (if you can).
Info:
    [s] Status: Show your status and information.
    [h] Help: Help you with game commands and functions.
        [c] Commands: Lists all avaliable commands.
        [-] You can combine commands ([fh]: Hunt)
    [c] Credits: Show credits of the game.
    [q] Quit: Throw in the towel!
"#;

pub fn win() {
    println!("{}", GAME_WIN_MSG.rgb(100, 250, 250));
    println!("{}", GAME_WIN.rgb(50, 250, 50));
}
pub const GAME_WIN_MSG: &'static str = r#"
    You Finally Did It!
    A tough journey, but you endured and conquered the trail!"#;
pub const GAME_WIN: &'static str = r#"
###############################################################
     __                      __   ___      ___    ___  __      __   __   ___  __   __        |
\ / /  \ |  |     |\/|  /\  |  \ |__     |  |      |  /  \    /  \ |__) |__  / _` /  \ |\ |  |
 |  \__/ \__/     |  | /~~\ |__/ |___    |  |      |  \__/    \__/ |  \ |___ \__> \__/ | \|  o
                                   _______
      ___________________________.'.------`
     '---------------------------.'
       `.       You            .'
     .-//`.       Win!      .'
  .' .//.'/`================'
 =[=:====:=]=           \||
  '. `--' .'             \_|
"#;

pub fn time_death() {
    println!("{}", TIME_DEATH_MSG.rgb(100, 250, 250));
    println!("{}", TIME_DEATH.rgb(250, 50, 50));
}
pub const TIME_DEATH_MSG: &'static str = r#"
    The winter front caught up to you!
    You froze to death!"#;
pub const TIME_DEATH: &'static str = r#"
###############################################################
        _____
     _.'_____`._
   .'.-'  12 `-.`. 
  /,' 11      1 `.\            *  .  *
 // 10      /   2 \\         . _\/ \/_ .
;;         /       ::         \  \ /  /
|| 9  ----O      3 ||       -==>: X :<==-
::                 ;;         / _/ \_ \
 \\ 8           4 //         '  /\ /\  '     -jgs
  \`. 7       5 ,'/            *  '  *
   '.`-.__6__.-'.'
    ((-._____.-))
    _))       ((_
   '--'       '--'  -SSt
"#;

pub fn health_death() {
    println!("{}", HEALTH_DEATH_MSG.rgb(100, 250, 250));
    println!("{}", HEALTH_DEATH.rgb(250, 50, 50));
}
pub const HEALTH_DEATH_MSG: &'static str = r#"
    You have finally succumbed to your wounds!
    You bled to death!"#;
pub const HEALTH_DEATH: &'static str = r#"
###############################################################
                           ,--.
                          {    }
                          K,   }
                         /  `Y`
                    _   /   /
                   {_'-K.__/
                     `/-.__L._
                     /  ' /`\_}
                    /  ' /     -ART BY ZEUS-
            ____   /  ' /
     ,-'~~~~    ~~/  ' /_
   ,'             ``~~~%%',
  (                     %  Y
 {                      %% I
{      -                 %  `.
|       ',                %  )
|        |   ,..__      __. Y
|    .,_./  Y ' / ^Y   J   )|
\           |' /   |   |   ||
 \          L_/    . _ (_,.'(
  \,   ,      ^^""' / |      )
    \_  \          /,L]     /
      '-_`-,       ` `   ./`
         `-(_            )
             ^^\..___,.--`
"#;

pub fn food_death() {
    println!("{}", FOOD_DEATH_MSG.rgb(100, 250, 250));
    println!("{}", FOOD_DEATH.rgb(250, 50, 50));
}
pub const FOOD_DEATH_MSG: &'static str = r#"
    Your food supply ran dry!
    You starved to death!"#;
pub const FOOD_DEATH: &'static str = r#"
###############################################################
//\
V  \
 \  \_
  \,'.`-.
   |\ `. `.       
   ( \  `. `-.                        _,.-:\
    \ \   `.  `-._             __..--' ,-';/
     \ `.   `-.   `-..___..---'   _.--' ,'/
      `. `.    `-._        __..--'    ,' /
        `. `-_     ``--..''       _.-' ,'
          `-_ `-.___        __,--'   ,'
             `-.__  `----"""    __.-'
                  `--..____..--'            -hh
"#;

pub fn stamina_death() {
    println!("{}", STAMINA_DEATH_MSG.rgb(100, 250, 250));
    println!("{}", STAMINA_DEATH.rgb(250, 50, 50));
}
pub const STAMINA_DEATH_MSG: &'static str = r#"
    You got not energy in the tank!
    You exhausted to death!"#;
pub const STAMINA_DEATH: &'static str = r#"
###############################################################
            _( }
   -=  _  <<  \
      `.\__/`/\\
 -=     '--'\\  `
      -=    //
jgs         \)
"#;

pub fn morality_death() {
    println!("{}", MORALITY_DEATH_MSG.rgb(100, 250, 250));
    println!("{}", MORALITY_DEATH.rgb(250, 50, 50));
}
pub const MORALITY_DEATH_MSG: &'static str = r#"
    There is no motivation left!
    You died by your own blade!"#;
pub const MORALITY_DEATH: &'static str = r#"
###############################################################
                                ,-.
                               ("O_)
                              / `-/
                             /-. /
                            /   )
                           /   /  
              _           /-. /
             (_)"-._     /   )
               "-._ "-'""( )/    
                   "-/"-._" `. 
                    /     "-.'._
                   /\       /-._"-._
    _,---...__    /  ) _,-"/    "-(_)
___<__(|) _   ""-/  / /   /
 '  `----' ""-.   \/ /   /
               )  ] /   /
       ____..-'   //   /                       )
   ,-""      __.,'/   /   ___                 /,
  /    ,--""/  / /   /,-""   """-.          ,'/
 [    (    /  / /   /  ,.---,_   `._   _,-','
  \    `-./  / /   /  /       `-._  """ ,-'
   `-._  /  / /   /_,'            ""--"
       "/  / /   /"         
       /  / /   /
      /  / /   /  o!O
     /  |,'   /  
    :   /    /
    [  /   ,'   "Blackadder"
    | /  ,'
    |/,-'
    P'
"#;

pub fn funny_death() {
    println!("{}", FUNNY_DEATH_MSG.rgb(100, 250, 250));
    println!("{}", FUNNY_DEATH.rgb(250, 50, 50));
}
pub const FUNNY_DEATH_MSG: &'static str = r#"
    'Your time has come.'"#;
pub const FUNNY_DEATH: &'static str = r#"
###############################################################
                 /\
                 ||
   ____ (((+))) _||_
  /.--.\  .-.  /.||.\
 /.,   \\(0.0)// || \\
/;`";/\ \\|m|//  ||  ;\
|:   \ \__`:`____||__:|
|:    \__ \T/ (@~)(~@)|
|:    _/|     |\_\/  :|
|:   /  |     |  \   :|
|'  /   |     |   \  '|
 \_/    |     |    \_/
        |     |
        |_____|
    jgs |_____|

(you are 1 in 1000)
"#;

pub fn quit() {
    println!("{}", GAME_QUIT_MSG.rgb(100, 250, 250));
    println!("{}", GAME_QUIT.rgb(250, 50, 50));
}
pub const GAME_QUIT_MSG: &'static str = r#"
    You decide that your journey has gone on long enough.
    So you pack up your things and head back home."#;
pub const GAME_QUIT: &'static str = r#"
###############################################################
 W                  .__. .__.      
[ ]                 |::| |::|           
 E          ._.     |::| |::|   ._.     
 |\         |:| ._. |::| |::|   |/|     
 \ \\|/     |:|_|/| |::| |::|_  |/|     
  |-( )-    |:|"|/|_|::| |::|\|_|/| _   
  | V L     |:|"|/|||::| |::|\|||/||:|  
  \    `  ___   ~~~~~~~~~~~~~~~~~~~~~~~
   |    \/  /      ~~~~ ~~~~ ~~~ ~~~~~~
    "#;

pub const CREDITS: &'static str = r#"
Theme Music: Ecstacy of 8-bit Gold - Timmer
    - (https://www.youtube.com/channel/UCx40Ii1WlEhyMtS1I9cYuTA)
Adventure Music: Fastest Gun in the 8-bit West - OkamiDeluxe
    - (https://www.youtube.com/channel/UCwvLYm2460XKx__lLzPXK0A)
Game Over: Game Over - MB Music
    - (https://www.youtube.com/channel/UCRpV6MFrOgw1YM9vzbsBPDw)
"#;
