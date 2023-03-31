#![no_std]
#![no_main]

use cortex_m_rt::entry;
use game::Game;
use microbit::{
    display::blocking::Display,
    hal::{prelude::InputPin, Timer},
    Board,
};
use panic_halt as _;

mod game;

/// How long to show each game-state, in milliseconds.
const DELAY_MS: u32 = 250;

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
    let a = board.buttons.button_a.into_floating_input();
    let b = board.buttons.button_b.into_floating_input();

    let mut game = Game::from_grid(INITIAL_GRID);
    let mut auto_play = false;

    // TODO: the button-handling feels pretty janky; e.g.
    // if you press and release a button quickly, it'll get ignored.
    // What can we do instead?

    loop {
        let led_grid = game.display();
        display.show(&mut timer, led_grid, DELAY_MS);

        let a_pressed = a.is_low().unwrap();
        let b_pressed = b.is_low().unwrap();

        // B button => toggle auto-play.
        if b_pressed {
            auto_play = !auto_play;
        }

        // Hold down both buttons to reset to the initial state.
        if a_pressed && b_pressed {
            game = Game::from_grid(INITIAL_GRID);
            auto_play = false;
        }
        // Press or hold just the A button to advance step-by-step.
        else if auto_play || a_pressed {
            game.evolve();
        }
    }
}
