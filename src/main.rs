use structopt::StructOpt;
use ethan_rust::game_module::EthanGame;
use ethan_rust::game_module::start;

#[derive(Debug, StructOpt)]
#[structopt(name = "gameoptions", about = "datatype containing all ethan game options")]
struct GameOptions {
    #[structopt(short="e", long="ethan-eyes")]
    ethan_eyes: bool,

    #[structopt(short="p", long="players", default_value="5")]
    player_count: usize,

    #[structopt(short="c", long="chips", default_value="5")]
    chips: i32,

    #[structopt(short="a", long="autoplay")]
    autoplay: bool,
}

fn main() {
    // parse options
    let gameopt = self::GameOptions::from_args();
    println!("{:?}", gameopt);

    let ethan_game = EthanGame::new(
        gameopt.player_count, gameopt.chips, gameopt.ethan_eyes, gameopt.autoplay);

    let boxed_game = Box::new(ethan_game);
    // start the game (autoplay in a separate thread for the hell of it)

    let result = start(boxed_game);
    match result {
        Ok(f) => {
            println!("game ran successfully");
            println!("final state:\n{:?}", f);
        },
        Err(e) => {
            println!("something catastrophic happened, game did not finish -- {:?}", e)
        }
    }
}
