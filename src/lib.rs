pub mod board;
use board::{Board, Move, ParsingError, Player};
mod computer;

use std::{
    fs::{File, write},
    io::{Write, stdin, stdout},
    path::PathBuf,
    time::Duration,
};
use clap::{ArgAction, arg, command, value_parser};
use rand::{rng, seq::IndexedRandom};
use heck::ToTitleCase;

#[derive(Clone)]
enum Tactic {
    Human,
    Random,
    Computer,
}

impl Tactic {
    fn choose_move(&self, board: &Board, timeout: Duration) -> Option<Move> {
        match self {
            Self::Human => Self::human(board),
            Self::Random => Self::random(board),
            Self::Computer => Self::computer(board, timeout),
        }
    }

    fn human(board: &Board) -> Option<Move> {
        loop {
            board.pretty_print();
            print!("Give your move (e.g. 'A5' or 'a5'), 'q' or 'Q' to quit: ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let content = input.trim();
            if content == "q" || content == "Q" {
                print!("Quitting, do you want to save this game (y/N)? ");
                stdout().flush().unwrap();
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                input.make_ascii_uppercase();
                let content = input.trim();
                if content == "Y" {
                    save(board);
                }
                return None;
            }
            match content.parse().ok().filter(|m| board.is_valid_move(m)) {
                Some(m) => return Some(m),
                None => println!("Invalid input. Try again."),
            };
        }
    }

    fn random(board: &Board) -> Option<Move> {
        let moves = board.moves();
        moves.choose(&mut rng()).copied()
    }

    fn computer(board: &Board, timeout: Duration) -> Option<Move> {
        computer::ab_minmax(board, timeout)
    }
}

impl From<&Tactic> for String {
    fn from(t: &Tactic) -> String {
        match *t {
            Tactic::Human => "human",
            Tactic::Random => "random",
            Tactic::Computer => "ai",
        }.into()
    }
}

fn game(mut board: Board, black: &Tactic, white: &Tactic, timeout: Duration) {
    println!("Welcome to this reversi game!");
    println!("{} player ({}) is {} and {} player ({}) is {}.",
        String::from(Player::Black).to_title_case(),
        char::from(Player::Black), String::from(black),
        String::from(Player::White),
        char::from(Player::White), String::from(white)
        );

    while let Some(player) = board.player {
        let chosen_move = match player {
            Player::Black => black,
            Player::White => white,
        }.choose_move(&board, timeout);

        let Some(m) = chosen_move else { break; };

        board = board.play(&m).expect("choose_move should return a valid move");
    }

    match board.player {
        Some(player) => {
            println!("{} resigned.", String::from(player).to_title_case());
            println!("{} wins!", String::from(player.other()).to_title_case());
        },
        None => {
            let (black, white) = board.score();
            match black.cmp(&white) {
                std::cmp::Ordering::Greater => {
                    println!("{} wins!", String::from(Player::Black).to_title_case());
                },
                std::cmp::Ordering::Less => {
                    println!("{} wins!", String::from(Player::White).to_title_case());
                },
                std::cmp::Ordering::Equal => {
                    println!("It's a tie!");
                },
            }
        }
    }

    board.pretty_print();
    println!("Thanks for playing, see you soon!");
}

fn save(board: &Board) {
    print!("Give a filename to save the game (default: 'board.txt'): ");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let name = input.trim();
    let name = if name.is_empty() {
        "board.txt"
    } else { name };

    write(name, String::from(board)).expect("could not write file");
}

pub fn run() -> Result<(), ParsingError> {
    let matches = command!(
        ).arg(arg!(-v --verbose "verbose output")
            .action(ArgAction::SetTrue)
        ).arg(arg!(-s --size <SIZE> "board size")
            .value_parser(value_parser!(u8)
                .range(1..6)
            ).default_value("4")
        ).arg(arg!(-t --timeout <TIMEOUT> "set AI timeout")
            .value_parser(value_parser!(u64)
                .range(1..)
            ).default_value("5")
        ).arg(arg!(-b [BLACK] "set tactic of black player")
            .long("black-ai")
            .value_parser(value_parser!(u8)
                .range(0..3)
            ).default_value("0")
        ).arg(arg!(-w [WHITE] "set tactic of white player")
            .long("white-ai")
            .value_parser(value_parser!(u8)
                .range(0..3)
            ).default_value("0")
        ).arg(arg!(-c --contest "enable 'contest' mode")
            .action(ArgAction::SetTrue)
        ).arg(arg!([FILE])
            .value_parser(value_parser!(PathBuf))
        ).get_matches();

    let size = matches.get_one::<u8>("size").expect("default ensures there is always a value") * 2;
    let timeout = Duration::from_secs(*matches.get_one::<u64>("timeout").expect("default ensures there is always a value"));
    let contest = *matches.get_one::<bool>("contest").expect("flag always has value");
    let _verbose = matches.get_one::<bool>("verbose").expect("flag always has value");

    let black_ai = match matches.get_one::<u8>("BLACK") {
        Some(0) => Tactic::Human,
        Some(1) => Tactic::Random,
        Some(2) => Tactic::Computer,
        _ => unreachable!(),
    };
    let white_ai = match matches.get_one::<u8>("WHITE") {
        Some(0) => Tactic::Human,
        Some(1) => Tactic::Random,
        Some(2) => Tactic::Computer,
        _ => unreachable!(),
    };

    let board = match matches.get_one::<PathBuf>("FILE") {
        Some(file) => Board::try_from(File::open(file)?)?,
        _ => Board::new(size),
    };

    if contest {
        println!("{}", String::from(Tactic::Computer.choose_move(&board, timeout).unwrap()));
    } else {
        game(board, &black_ai, &white_ai, timeout);
    }

    Ok(())
}
