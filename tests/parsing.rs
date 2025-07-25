use rustversi::board::{Board, ParsingError, Square, Player};

use std::fs::File;

// pub enum ParsingError {
//     IOError,
//     Generic,
//     EmptyFile,
//     InvalidCharacter,
//     BadSize,
//     InconsistentSize,
// }

#[test]
fn board_0x0_fail() {
    let file = File::open("tests/board_parsing/board-0x0.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::BadSize));
}

#[test]
fn board_1x1_fail() {
    let file = File::open("tests/board_parsing/board-1x1.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::BadSize));
}

#[test]
fn board_2x2_pass() {
    let file = File::open("tests/board_parsing/board-2x2.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(Board::new(2)));
}

#[test]
fn board_3x3_fail() {
    let file = File::open("tests/board_parsing/board-3x3.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::BadSize));
}

#[test]
fn board_4x4_pass() {
    let file = File::open("tests/board_parsing/board-4x4.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(Board::new(4)));
}

#[test]
fn board_5x5_fail() {
    let file = File::open("tests/board_parsing/board-5x5.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::BadSize));
}

#[test]
fn board_6x6_pass() {
    let file = File::open("tests/board_parsing/board-6x6.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(Board::new(6)));
}

#[test]
fn board_7x7_fail() {
    let file = File::open("tests/board_parsing/board-7x7.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::BadSize));
}

#[test]
fn board_8x8_pass() {
    let file = File::open("tests/board_parsing/board-8x8.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(Board::new(8)));
}

#[test]
fn board_9x9_fail() {
    let file = File::open("tests/board_parsing/board-9x9.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::BadSize));
}

#[test]
fn board_10x10_pass() {
    let file = File::open("tests/board_parsing/board-10x10.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(Board::new(10)));
}

#[test]
fn board_11x11_fail() {
    let file = File::open("tests/board_parsing/board-11x11.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::BadSize));
}

#[test]
fn board_12x12_fail() {
    let file = File::open("tests/board_parsing/board-12x12.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::BadSize));
}

#[test]
fn almost_full_board_pass() {
    let mut board = Board::new(8);
    for x in 0..8 {
        for y in 0..8 {
            board = board.set(x, y, Square::Disc(Player::White));
        }
    }
    board = board.set(7, 7, Square::Empty);
    board.player = None;

    let file = File::open("tests/board_parsing/board-almost_full_board.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(board));
}

#[test]
#[should_panic]
fn current_player_get_two_chars_fail() {
    let file = File::open("tests/board_parsing/board-current_player_get_two_chars.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::InconsistentSize));
}

#[test]
fn empty_board_pass() {
    let mut board = Board::new(8)
        .set(3, 3, Square::Empty)
        .set(3, 4, Square::Empty)
        .set(4, 3, Square::Empty)
        .set(4, 4, Square::Empty);
    board.player = None;

    let file = File::open("tests/board_parsing/board-empty_board.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(board));
}

#[test]
fn empty_file_fail() {
    let file = File::open("tests/board_parsing/board-empty_file.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::EmptyFile));
}

#[test]
fn empty_stone_as_current_player_fail() {
    let file = File::open("tests/board_parsing/board-empty_stone_as_current_player.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::PlayerParseError('_')));
}

#[test]
fn eof_after_current_player_fail() {
    let file = File::open("tests/board_parsing/board-eof_after_current_player.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::BadSize));
}

#[test]
fn eof_before_end_of_the_board_fail() {
    let file = File::open("tests/board_parsing/board-eof_before_end_of_the_board.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::InconsistentSize));
}

#[test]
fn extra_empty_lines_pass() {
    let board = Board::new(8);

    let file = File::open("tests/board_parsing/board-extra_empty_lines.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(board));
}

#[test]
fn extra_spaces_around_chars_pass() {
    let board = Board::new(8);

    let file = File::open("tests/board_parsing/board-extra_spaces_around_chars.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(board));
}

#[test]
fn first_line_overflow_fail() {
    let file = File::open("tests/board_parsing/board-first_line_overflow.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::BadSize));
}

#[test]
fn full_board_pass() {
    let mut board = Board::new(8);
    for x in 0..8 {
        for y in 0..8 {
            board = board.set(x, y, Square::Disc(Player::White));
        }
    }
    board.player = None;

    let file = File::open("tests/board_parsing/board-full_board.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(board));
}

#[test]
fn impossible_board_01_pass() {
    let mut board = Board::new(8);
    for x in 0..8 {
        for y in 0..8 {
            if (x + y) % 2 == 0 {
                board = board.set(x, y, Square::Disc(Player::White));
            }
        }
    }
    board = board
        .set(3, 4, Square::Empty)
        .set(4, 3, Square::Empty);
    board.player = None;

    let file = File::open("tests/board_parsing/board-impossible_board-01.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(board));
}

#[test]
fn impossible_board_02_pass() {
    let board = Board::new(8)
        .set(2, 1, Square::Disc(Player::White))
        .set(3, 1, Square::Disc(Player::White))
        .set(4, 1, Square::Disc(Player::White))
        .set(2, 5, Square::Disc(Player::White))
        .set(3, 5, Square::Disc(Player::White))
        .set(4, 5, Square::Disc(Player::White))
        .set(1, 2, Square::Disc(Player::White))
        .set(1, 3, Square::Disc(Player::White))
        .set(1, 4, Square::Disc(Player::White))
        .set(5, 2, Square::Disc(Player::White))
        .set(5, 3, Square::Disc(Player::White))
        .set(5, 4, Square::Disc(Player::White))
        .set(3, 3, Square::Disc(Player::White))
        .set(3, 2, Square::Disc(Player::Black))
        .set(3, 4, Square::Disc(Player::Black))
        .set(2, 3, Square::Disc(Player::Black))
        .set(4, 3, Square::Disc(Player::Black))
        .set(4, 4, Square::Empty);

    let file = File::open("tests/board_parsing/board-impossible_board-02.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(board));
}

#[test]
fn line_too_long_fail() {
    let file = File::open("tests/board_parsing/board-line_too_long.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::InconsistentSize));
}

#[test]
fn line_too_short_fail() {
    let file = File::open("tests/board_parsing/board-line_too_short.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::InconsistentSize));
}

#[test]
fn line_too_short_with_comment_fail() {
    let file = File::open("tests/board_parsing/board-line_too_short_with_comment.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::InconsistentSize));
}

#[test]
fn line_too_short_with_no_newline_fail() {
    let file = File::open("tests/board_parsing/board-line_too_short_with_no_newline.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::InconsistentSize));
}

#[test]
fn long_line_filled_with_spaces_pass() {
    let board = Board::new(8);

    let file = File::open("tests/board_parsing/board-long_line_filled_with_spaces.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(board));
}

#[test]
fn missing_board_fail() {
    let file = File::open("tests/board_parsing/board-missing_board.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::BadSize));
}

#[test]
fn missing_current_player_fail() {
    let file = File::open("tests/board_parsing/board-missing_current_player.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::PlayerParseError('_')));
}

#[test]
fn missing_newline_after_current_player_pass() {
    let board = Board::new(8);

    let file = File::open("tests/board_parsing/board-missing_newline_after_current_player.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(board));
}

#[test]
fn no_final_newline_pass() {
    let board = Board::new(8);

    let file = File::open("tests/board_parsing/board-no_final_newline.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(board));
}

#[test]
fn stop_at_first_line_without_newline_fail() {
    let file = File::open("tests/board_parsing/board-stop_at_first_line_without_newline.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::InconsistentSize));
}

#[test]
fn too_few_lines_fail() {
    let file = File::open("tests/board_parsing/board-too_few_lines.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::InconsistentSize));
}

#[test]
fn too_few_lines_with_comment_fail() {
    let file = File::open("tests/board_parsing/board-too_few_lines_with_comment.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::InconsistentSize));
}

#[test]
fn too_many_lines_fail() {
    let file = File::open("tests/board_parsing/board-too_many_lines.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::InconsistentSize));
}

#[test]
fn with_comments_pass() {
    let board = Board::new(8);

    let file = File::open("tests/board_parsing/board-with_comments.pass").unwrap();
    assert_eq!(Board::try_from(file), Ok(board));
}

#[test]
fn wrong_character_fail() {
    let file = File::open("tests/board_parsing/board-wrong_character.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::InvalidCharacter('Z')));
}

#[test]
fn wrong_current_player_char_fail() {
    let file = File::open("tests/board_parsing/board-wrong_current_player_char.fail").unwrap();
    assert_eq!(Board::try_from(file), Err(ParsingError::PlayerParseError('Z')));
}
