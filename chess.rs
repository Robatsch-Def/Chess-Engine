#![allow(dead_code)]
use regex::Regex;
pub fn chess() {
	let mut board = new_game();
	let mut input_move: String = String::new();
	let (mut outcome, mut is_whites_turn): (bool, bool);
	is_whites_turn = true;
	println!("New game!");
	loop {
		println!("{}'s turn. Enter your move.", if is_whites_turn {"White"} else {"Black"});
		std::io::stdin.read_line(&mut input_move).expect("Failed to read.");
		is_whites_turn = !is_whites_turn;
	}
	println!("End of game.");
}
fn is_valid_format(chess_move: String) -> bool { //TODO: Replace with Regex
	//e4, exd4, exg8=Q, e8=R
	//Bf4, Bxf4, R1e2, Rfd8, Qh4e1, Qh2xe1
	if Regex::new(r"").unwwrap().is_match(chess_move) {
	
	}
}
fn create_piece(
	piece_type: PieceType, color: bool, space: &str
) -> ChessPiece {
	ChessPiece {
		piece_type: piece_type,
		color: color,
		is_alive: true,
		space: space
	}
}
enum PieceType {
	King, Queen, Rook,
	Bishop, Knight, Pawn
}
struct ChessPiece<'a> {
	piece_type: PieceType,
	is_white: bool,
	space: &'a str,
	is_alive: bool
}
fn new_game() -> [[ChessPiece;16];2]{
	[
		[
			create_piece(PieceType::King, true, "e1"),
			create_piece(PieceType::Queen, true, "d1"),
			create_piece(PieceType::Rook, true, "a1"),
			create_piece(PieceType::Rook, true, "h1"),
			create_piece(PieceType::Bishop, true, "c1"),
			create_piece(PieceType::Bishop, true, "f1"),
			create_piece(PieceType::Knight, true, "b1"),
			create_piece(PieceType::Knight, true, "g1"),
			create_piece(PieceType::Pawn, true, "a2"),
			create_piece(PieceType::Pawn, true, "b2"),
			create_piece(PieceType::Pawn, true, "c2"),
			create_piece(PieceType::Pawn, true, "d2"),
			create_piece(PieceType::Pawn, true, "e2"),
			create_piece(PieceType::Pawn, true, "f2"),
			create_piece(PieceType::Pawn, true, "g2"),
			create_piece(PieceType::Pawn, true, "h2"),
		],
		[
			create_piece(PieceType::King, false, "e8"),
			create_piece(PieceType::Queen, false, "d8"),
			create_piece(PieceType::Rook, false, "a8"),
			create_piece(PieceType::Rook, false, "h8"),
			create_piece(PieceType::Bishop, false, "c8"),
			create_piece(PieceType::Bishop, false, "f8"),
			create_piece(PieceType::Knight, false, "b8"),
			create_piece(PieceType::Knight, false, "g8"),
			create_piece(PieceType::Pawn, false, "a7"),
			create_piece(PieceType::Pawn, false, "b7"),
			create_piece(PieceType::Pawn, false, "c7"),
			create_piece(PieceType::Pawn, false, "d7"),
			create_piece(PieceType::Pawn, false, "e7"),
			create_piece(PieceType::Pawn, false, "f7"),
			create_piece(PieceType::Pawn, false, "g7"),
			create_piece(PieceType::Pawn, false, "h7"),
		]
	]
}
