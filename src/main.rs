extern crate rand;
extern crate sfml;

use rand::{Rng, thread_rng};

use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::window::{Event, Style, Key};
use sfml::system::Clock;
use rand::prelude::ThreadRng;

const TETROMINOS: [[usize; 4]; 7] = [
    [0, 2, 4, 6],
    [0, 1, 2, 4],
    [0, 2, 4, 5],
    [0, 1, 2, 3],
    [0, 2, 3, 5],
    [0, 2, 3, 4],
    [1, 2, 3, 4]
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
const GAME_HEIGHT: usize = 20;

const GAME_SIZE: usize = GAME_WIDTH * (GAME_HEIGHT + 4);
const GAME_UNPLAYABLE_SIZE: usize = GAME_WIDTH * 4;

const GAME_DELAY: f32 = 0.5f32;
const GAME_QUICK_DELAY: f32 = 0.2f32;

struct Game<'s> {
    field: [usize; GAME_SIZE],
    current: [(usize, usize); 4],
    current_color: usize,
    next: [(usize, usize); 4],
    next_color: usize,
    time: f32,
    square: RectangleShape<'s>,
    window: RenderWindow,
    rng: ThreadRng
}

impl Game<'_> {
    fn new() -> Self {
        let mut square = RectangleShape::new();
        square.set_size((28f32, 28f32));


        let mut window = RenderWindow::new((800, 600),
                                           "Tetris",
                                           Style::DEFAULT,
                                           &Default::default());
        window.set_vertical_sync_enabled(true);

        let rng = thread_rng();

        Game {
            field: [0; GAME_SIZE],
            time: 0f32,
            current: [(0, 0); 4],
            current_color: 0,
            next: [(0, 0); 4],
            next_color: 0,
            square,
            window,
            rng
        }
    }

    fn random_tetromino(self: &mut Self) -> ([(usize, usize); 4], usize) {
        let color = self.rng.gen_range(1, 7);
        let mut tetromino = [(0, 0); 4];

        let mut i = 0;
        for point in TETROMINOS[color].iter() {
            tetromino[i] = (point % 2, point / 2);
            i += 1;
        }

        (tetromino, color)
    }

    fn update(self: &mut Self, dt: f32, up: bool, down: bool, left: bool, right: bool) {
        // (1) Check if down, then accelerate delay
        let delay = if down { GAME_QUICK_DELAY } else { GAME_DELAY };

        // Delay movement of tetrominos
        if self.time > delay {
            let mut copy = self.current;
            let mut maxy = 0;

            if left || right {
                let mut correct = true;

                for i in copy.iter_mut() {
                    if ((i.0 < 1 || self.field[i.0 + (i.1 + 1) * GAME_WIDTH - 1] != 0) && left)
                        || ((i.0 > GAME_WIDTH || self.field[i.0 + (i.1 + 1) * GAME_WIDTH + 1] != 0) && right) {
                        correct = false;
                        break;
                    }

                    if left { (*i).0 -= 1 } else { (*i).0 += 1 }
                }

                if !correct {
                    copy = self.current;
                }
            }

            let mut something_underneath = false;

            // (4) Move tetromino down
            for i in copy.iter_mut() {
                (*i).1 += 1;
                if i.1 > maxy { maxy = i.1 }

                if self.field[i.0 + (i.1 + 1) * GAME_WIDTH] != 0 {
                    something_underneath = true;
                    break;
                }
            }

            self.current = copy;

            // (5) If tetromino has hit bottom, or there is a non-empty block directly underneath
            //     then copy current in to field, move next tetromino in to current,
            //     generate a new tetromino for the next.

            if something_underneath || maxy >= GAME_HEIGHT - 1 {
                for point in self.current.iter() {
                    self.field[point.0 + point.1 * GAME_WIDTH] = self.current_color;
                }

                self.current = self.next;
                self.current_color = self.next_color;

                let (next, next_color) = self.random_tetromino();
                self.next = next;
                self.next_color = next_color;
            }

            // (6) Reset delay timer
            self.time = 0f32;
        }
    }

    fn draw(self: &mut Self) {
        for i in GAME_UNPLAYABLE_SIZE..GAME_SIZE - GAME_UNPLAYABLE_SIZE {
            if self.field[i] == 0 {
                continue;
            }

            let x = i % GAME_WIDTH;
            let y = i / GAME_WIDTH - 4;

            self.square.set_fill_color(TETROMINO_COLORS[self.field[i]]);
            self.square.set_position((x as f32 * 30f32 + 60f32, y as f32 * 30f32 + 60f32));
            self.window.draw(&self.square);
        }

        for i in self.current.iter() {
            if i.1 > 4 {
                self.square.set_fill_color(TETROMINO_COLORS[self.current_color]);
                self.square.set_position((i.0 as f32 * 30f32 + 60f32, (i.1 - 4) as f32 * 30f32 + 60f32));
                self.window.draw(&self.square);
            }
        }
    }

    fn run(self: &mut Self) {
        let mut clock = Clock::start();

        let (current, current_color) = self.random_tetromino();
        self.current = current;
        self.current_color = current_color;

        let (next, next_color) = self.random_tetromino();
        self.next = next;
        self.next_color = next_color;

        // Flags to signify which key is being held
        let mut up = false;
        let mut down = false;
        let mut left = false;
        let mut right = false;

        while self.window.is_open() {
            let dt = clock.elapsed_time().as_seconds();
            clock.restart();

            self.time += dt;

            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed => self.window.close(),

                    Event::KeyPressed { code: Key::Up, .. } => up = true,
                    Event::KeyPressed { code: Key::Down, .. } => down = true,
                    Event::KeyPressed { code: Key::Left, .. } => left = true,
                    Event::KeyPressed { code: Key::Right, .. } => right = true,

                    // Clear the key-held flags once that key is released.
                    Event::KeyReleased { code: Key::Up, .. } => up = false,
                    Event::KeyReleased { code: Key::Down, .. } => down = false,
                    Event::KeyReleased { code: Key::Left, .. } => left = false,
                    Event::KeyReleased { code: Key::Right, .. } => right = false,

                    _ => {}
                }
            }

            self.window.clear(Color::BLACK);

            self.update(dt, up, down, left, right);
            self.draw();

            self.window.display();
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.run();
}
