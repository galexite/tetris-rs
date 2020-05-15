extern crate rand;
extern crate sfml;

use rand::{Rng, thread_rng};

use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::window::{Event, Style};

const TETROMINOS: [i32; 28] = [
    0, 2, 4, 6,
    0, 1, 2, 4,
    0, 2, 4, 5,
    0, 1, 2, 3,
    0, 2, 3, 5,
    0, 2, 3, 4,
    1, 2, 3, 4
];

const TETROMINO_COLORS: [Color; 8] = [
    Color::TRANSPARENT,
    Color::CYAN,
    Color::BLUE,
    Color::WHITE, // should actually be orange
    Color::YELLOW,
    Color::GREEN,
    Color::MAGENTA,
    Color::RED
];

const GAME_WIDTH: usize = 10;
const GAME_HEIGHT: usize = 24;

const GAME_SIZE: usize = GAME_WIDTH * GAME_HEIGHT;
const GAME_UNPLAYABLE_SIZE: usize = GAME_WIDTH * 4;

fn main() {
    let mut window = RenderWindow::new((800, 600),
                                   "Tetris",
                                   Style::DEFAULT,
                                   &Default::default());
    window.set_vertical_sync_enabled(true);
    
    let mut square = RectangleShape::new();
    square.set_size((28f32, 28f32));

    let mut game = [0; GAME_SIZE];

    let mut rng = thread_rng();
    for i in 0..GAME_SIZE {
        game[i] = rng.gen_range(0, 7);
    }

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }

        window.clear(Color::BLACK);

        // Draw the game
        for i in GAME_UNPLAYABLE_SIZE..GAME_SIZE - GAME_UNPLAYABLE_SIZE {
            if game[i] == 0 {
                continue;
            }

            let x = i % GAME_WIDTH;
            let y = i / GAME_WIDTH - 4;

            square.set_fill_color(TETROMINO_COLORS[game[i]]);
            square.set_position((x as f32 * 30f32, y as f32 * 30f32));
            square.move_((60f32, 60f32));
            window.draw(&square);
        }

        window.display();
    }
}
