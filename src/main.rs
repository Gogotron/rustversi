mod board;
use board::{Board, ParsingError, Player, Move};

use std::path::PathBuf;
use std::fs::File;
use clap::{command, arg, ArgAction, value_parser};
use heck::ToTitleCase;

#[derive(Clone)]
enum Tactic {
    Human,
    Random,
    Computer,
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

impl Tactic {
    fn choose_move(&self, board: &Board) -> Option<Move> {
        match self {
            Self::Human => Self::human(board),
            Self::Random => Self::random(board),
            Self::Computer => Self::computer(board),
        }
    }

    fn human(board: &Board) -> Option<Move> {
        todo!();
    }

    fn random(board: &Board) -> Option<Move> {
        todo!();
    }

    fn computer(board: &Board) -> Option<Move> {
        todo!();
    }
}

fn game(mut board: Board, black: &Tactic, white: &Tactic) {
    println!("Welcome to this reversi game!");
    println!("{} player ({}) is {} and {} player ({}) is {}.",
        String::from(Player::Black).to_title_case(),
        char::from(Player::Black), String::from(black),
        String::from(Player::White),
        char::from(Player::White), String::from(white)
        );
    board.pretty_print();

    loop {
        let chosen_move = match board.player {
            Some(Player::Black) => black.choose_move(&board),
            Some(Player::White) => white.choose_move(&board),
            None => todo!(),
        };

        match chosen_move {
            None => {
                println!("{} resigned.", String::from(board.player.expect("?")).to_title_case());
                break;
            },
            Some(m) => {
                board = board.play(m).expect("invalid move");
            },
        }

        todo!();
    }
}

fn main() -> Result<(), ParsingError> {
    let matches = command!(
        ).arg(arg!(-v --verbose "verbose output")
            .action(ArgAction::SetTrue)
        ).arg(arg!(-s --size <SIZE> "board size")
            .value_parser(value_parser!(u8)
                .range(1..6)
            ).default_value("4")
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
    let _white_ai = matches.get_one::<u8>("WHITE").expect("default ensures there is always a value");
    let _contest = matches.get_one::<bool>("contest").expect("flag always has value");
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

    game(board, &black_ai, &white_ai);

    Ok(())
}
