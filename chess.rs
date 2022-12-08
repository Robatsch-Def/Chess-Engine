#![allow(dead_code)]
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
fn is_valid_format(chess_move: String) -> bool {
	//e4, exd4, exg8=Q
	//Bf4, Bxf4, R1e2, Rfd8, Qh4e1, Qh2xe1
	let move_vec: Vec<char> = chess_move.chars().collect();
	const file_chars: [char;8] = [
		'a','b','c','d','e','f','g','h'
	];
	const rank_chars: [char;8] = [
		'1','2','3','4','5','6','7','8'
	];
	const type_chars: [char;5] = [
		'K','Q','R','B','N'
	];
	const other_chars: [char;2] = [
		'x','='
	]
	//Remember the 'x' for captures!
	if type_chars.contains(move_vec[0]) { //Is not a pawn

	} else { //Is a pawn

	}

}
fn create_piece(
	piece_type: PieceType, color: PieceColor, space: &str
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
enum PieceColor {
	Black, White
}
struct ChessPiece<'a> {
	piece_type: PieceType,
	color: PieceColor,
	space: &'a str,
	is_alive: bool
}
fn new_game() -> [[ChessPiece;16];2]{
	[
		[
			create_piece(PieceType::King, PieceColor::White, "e1"),
			create_piece(PieceType::Queen, PieceColor::White, "d1"),
			create_piece(PieceType::Rook, PieceColor::White, "a1"),
			create_piece(PieceType::Rook, PieceColor::White, "h1"),
			create_piece(PieceType::Bishop, PieceColor::White, "c1"),
			create_piece(PieceType::Bishop, PieceColor::White, "f1"),
			create_piece(PieceType::Knight, PieceColor::White, "b1"),
			create_piece(PieceType::Knight, PieceColor::White, "g1"),
			create_piece(PieceType::Pawn, PieceColor::White, "a2"),
			create_piece(PieceType::Pawn, PieceColor::White, "b2"),
			create_piece(PieceType::Pawn, PieceColor::White, "c2"),
			create_piece(PieceType::Pawn, PieceColor::White, "d2"),
			create_piece(PieceType::Pawn, PieceColor::White, "e2"),
			create_piece(PieceType::Pawn, PieceColor::White, "f2"),
			create_piece(PieceType::Pawn, PieceColor::White, "g2"),
			create_piece(PieceType::Pawn, PieceColor::White, "h2"),
		],
		[
			create_piece(PieceType::King, PieceColor::Black, "e8"),
			create_piece(PieceType::Queen, PieceColor::Black, "d8"),
			create_piece(PieceType::Rook, PieceColor::Black, "a8"),
			create_piece(PieceType::Rook, PieceColor::Black, "h8"),
			create_piece(PieceType::Bishop, PieceColor::Black, "c8"),
			create_piece(PieceType::Bishop, PieceColor::Black, "f8"),
			create_piece(PieceType::Knight, PieceColor::Black, "b8"),
			create_piece(PieceType::Knight, PieceColor::Black, "g8"),
			create_piece(PieceType::Pawn, PieceColor::Black, "a7"),
			create_piece(PieceType::Pawn, PieceColor::Black, "b7"),
			create_piece(PieceType::Pawn, PieceColor::Black, "c7"),
			create_piece(PieceType::Pawn, PieceColor::Black, "d7"),
			create_piece(PieceType::Pawn, PieceColor::Black, "e7"),
			create_piece(PieceType::Pawn, PieceColor::Black, "f7"),
			create_piece(PieceType::Pawn, PieceColor::Black, "g7"),
			create_piece(PieceType::Pawn, PieceColor::Black, "h7"),
		]
	]
}