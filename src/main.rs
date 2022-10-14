mod cards;
mod core;
mod extras;
mod utils;

fn main() {
    let mut board = core::generate_board(18).unwrap();

    board.flip(2, 2);
    board.flip(3, 3);
    println!("{}", board)
}
