pub mod game_module {
    use rand::random;


    #[derive(Debug)]
    pub struct EthanGame {
        players : Vec<i32>,
        ethan   : i32,
        ethan_eyes : bool,
        autoplay: bool,
        starting_chips : i32,
        turn_count : i32,
        pub current_player : usize,
    }

    impl EthanGame {
        pub fn new(p: usize, c: i32, ee: bool, auto: bool) -> EthanGame {
            EthanGame{
                players : vec![c; p],
                ethan   : 0,
                ethan_eyes : ee,
                autoplay: auto,
                starting_chips : c,
                turn_count : 0,
                current_player : 0, 
            }
        }
    }

    pub trait FiniteGame {
        fn check_end_condition(&self) -> bool;
    }

    impl FiniteGame for EthanGame {
        fn check_end_condition(&self) -> bool {
            // is current player in possession of all the chips
            if self.players[self.current_player] == (self.starting_chips * (self.players.len() as i32)) {
                println!("HOORAY! PLAYER {} HAS WON THE GAME, ETHAN SUCKS", self.current_player);
                true
            } else if self.ethan == (self.starting_chips * (self.players.len() as i32)) {
            // do no players have any chips left?
                println!("WELP, ETHAN WINS, EVERYBODY LOSES. THE END");
                true
            } else {
            // otherwise, false
                false
            }
        }
    }

    pub fn start(game: &mut EthanGame) -> Result<(), &str> {
        if game.autoplay {
            println!("autoplay on, starting in a thread then waiting");
            run(game)
        } else {
            println!("autoplay off, running in main thread");
            run(game)
        }
    }

    fn run(game: &mut EthanGame) -> Result<(), &str> {
        println!("game execution begins");
        let mut the_game = game;
        loop {
            match execute_turn(the_game) {
                Ok(g) => {the_game = g;},
                Err(s) => break Err(s), 
            };
            if the_game.check_end_condition() {
                println!("game over");
                break Ok(()) 
            }
        }
    }

    fn execute_turn(game: &mut EthanGame) -> Result<&mut EthanGame, &str> {
        // check if current player has any chips. if not, return -1 to skip increment
        if game.players[game.current_player] < 1 {
            game.current_player = (game.current_player + 1) % game.players.len();
            return Ok(game);
        }
        println!("turn {} -- it is player {}'s turn", game.turn_count, game.current_player);
        // roll two die
        let d1: u8 = random(); // separate lines so random can infer type
        let d1 = (d1 % 6) + 1;
        let d2: u8 = random();
        let d2 = (d2 % 6) + 1;
        // if 2, lose it all
        if d1 + d2 == 2 && game.ethan_eyes {
            println!("player {} got ethan eyes, lost all {} chips to ethan", game.current_player, game.players[game.current_player]);
            game.ethan += game.players[game.current_player];
            game.players[game.current_player] = 0;
        } else if d1 + d2 == 4 {
            println!("jackpot, player {} gets all {} of ethan's chips", game.current_player, game.ethan);
            game.players[game.current_player] += game.ethan;
            game.ethan = 0;
        } else {
            println!("rolled a {}, player {} loses 1 chip to ethan", d1 + d2, game.current_player);
            game.ethan += 1;
            game.players[game.current_player] -= 1;
        }
        println!("current state:\n{:?}", game);
        game.turn_count += 1;
        game.current_player = (game.current_player + 1) % game.players.len();
        // else, lose one
        Ok(game)
    }
}