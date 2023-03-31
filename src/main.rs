#![no_std]
#![no_main]

use cortex_m_rt::entry;
use game::Game;
use microbit::{display::blocking::Display, hal::Timer, Board};
use panic_halt as _;

mod game;

/// How long to show each game-state, in milliseconds.
const DELAY_MS: u32 = 1_000;

const INITIAL_GRID: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 1, 1, 0],
    [0, 1, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
];

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut game = Game::from_grid(INITIAL_GRID);

    loop {
        let led_grid = game.display();
        display.show(&mut timer, led_grid, DELAY_MS);

        game.evolve();
    }
}
