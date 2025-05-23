mod board;
use board::Board;
use board::ParsingError;

use std::path::PathBuf;
use std::fs::File;

use clap::{command, arg, ArgAction, value_parser};

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
    let _black_ai = matches.get_one::<u8>("BLACK").expect("default ensures there is always a value");
    let _white_ai = matches.get_one::<u8>("WHITE").expect("default ensures there is always a value");
    let _contest = matches.get_one::<bool>("contest").expect("flag always has value");
    let _verbose = matches.get_one::<bool>("verbose").expect("flag always has value");

    let board = match matches.get_one::<PathBuf>("FILE") {
        Some(file) => Board::try_from(File::open(file)?)?,
        _ => Board::new(size),
    };

    board.pretty_print();

    Ok(())
}
