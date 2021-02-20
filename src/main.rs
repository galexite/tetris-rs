extern crate rand;
extern crate raylib;

use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use raylib::color::Color;
use raylib::prelude::*;
use std::convert::TryInto;

const TETROMINOS: [[usize; 4]; 7] = [
    [0, 2, 4, 6],
    [0, 1, 2, 4],
    [0, 2, 4, 5],
    [0, 1, 2, 3],
    [0, 2, 3, 5],
    [0, 2, 3, 4],
    [1, 2, 3, 4],
];

const TETROMINO_COLORS: [Color; 8] = [
    Color::BLANK,
    Color::SKYBLUE,
    Color::BLUE,
    Color::ORANGE,
    Color::YELLOW,
    Color::GREEN,
    Color::MAGENTA,
    Color::RED,
];

const GAME_WIDTH: usize = 10;
const GAME_HEIGHT: usize = 20;

const GAME_SIZE: usize = GAME_WIDTH * (GAME_HEIGHT + 4);
const GAME_UNPLAYABLE_SIZE: usize = GAME_WIDTH * 4;

const GAME_DELAY: f64 = 0.5f64;
const GAME_QUICK_DELAY: f64 = 0.2f64;

const SQUARE_SIZE: i32 = 28;

struct Game {
    field: [usize; GAME_SIZE],
    current: [(usize, usize); 4],
    current_color: usize,
    next: [(usize, usize); 4],
    next_color: usize,
    time: f64,
    rl: RaylibHandle,
    thread: RaylibThread,
    rng: ThreadRng,
}

impl Game {
    fn new() -> Self {
        let (rl, thread) = raylib::init().size(800, 600).title("tetris-rs").build();

        let rng = thread_rng();

        Game {
            field: [0; GAME_SIZE],
            time: 0f64,
            current: [(0, 0); 4],
            current_color: 0,
            next: [(0, 0); 4],
            next_color: 0,
            rl,
            thread,
            rng,
        }
    }

    fn random_tetromino(self: &mut Self) -> ([(usize, usize); 4], usize) {
        let color = self.rng.gen_range(1, 7);
        let mut tetromino = [(0, 0); 4];

        let mut i = 0;
        for point in TETROMINOS[color].iter() {
            tetromino[i] = (point / 2, point % 2);
            i += 1;
        }

        (tetromino, color)
    }

    fn update(self: &mut Self, dt: f64, up: bool, down: bool, left: bool, right: bool) {
        // (1) Check if down, then accelerate delay
        let delay = if down { GAME_QUICK_DELAY } else { GAME_DELAY };

        // Delay movement of tetrominos
        if self.time > delay {
            let mut copy = self.current;
            let mut correct = true;
            let mut swap = false;

            for i in copy.iter_mut() {
                if (left && (i.0 <= 0 || self.field[i.0 + (i.1 + 1) * GAME_WIDTH - 1] != 0))
                    || (right
                        && (i.0 >= GAME_WIDTH || self.field[i.0 + (i.1 + 1) * GAME_WIDTH + 1] != 0))
                {
                    println!("can't go that way: l:{}, r:{}; i.0:{}, i.1:{}", left, right, i.0, i.1);
                    correct = false;
                    break;
                }

                if left {
                    (*i).0 -= 1;
                }

                if right {
                    (*i).0 += 1;
                }
            }

            if correct {
                self.current = copy;
            }

            for i in self.current.iter_mut() {
                (*i).1 += 1;

                if i.1 >= GAME_HEIGHT - 1 || self.field[i.0 + (i.1 + 1) * GAME_WIDTH] != 0 {
                    swap = true;
                }
            }

            if swap {
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
            self.time = 0f64;
        }
    }

    fn draw(self: &mut Self) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);

        for i in GAME_UNPLAYABLE_SIZE..GAME_SIZE - GAME_UNPLAYABLE_SIZE {
            if self.field[i] == 0 {
                continue;
            }

            let x: i32 = (i % GAME_WIDTH).try_into().unwrap();
            let y: i32 = (i / GAME_WIDTH - 4).try_into().unwrap();

            d.draw_rectangle(
                x * 30 + 60,
                y * 30 + 60,
                SQUARE_SIZE,
                SQUARE_SIZE,
                TETROMINO_COLORS[self.field[i]],
            )
        }

        for i in self.current.iter() {
            if i.1 > 4 {
                let x: i32 = i.0.try_into().unwrap();
                let y: i32 = i.1.try_into().unwrap();

                d.draw_rectangle(
                    x * 30 + 60,
                    (y - 4) * 30 + 60,
                    SQUARE_SIZE,
                    SQUARE_SIZE,
                    TETROMINO_COLORS[self.current_color],
                )
            }
        }
    }

    fn run(self: &mut Self) {
        use raylib::consts::KeyboardKey::*;

        let mut last_time = self.rl.get_time();

        let (current, current_color) = self.random_tetromino();
        self.current = current;
        self.current_color = current_color;

        let (next, next_color) = self.random_tetromino();
        self.next = next;
        self.next_color = next_color;

        while !self.rl.window_should_close() {
            let curr_time = self.rl.get_time();
            let dt = curr_time - last_time;
            last_time = curr_time;
            self.time += dt;

            let up = self.rl.is_key_down(KEY_UP);
            let down = self.rl.is_key_down(KEY_DOWN);
            let left = self.rl.is_key_down(KEY_LEFT);
            let right = self.rl.is_key_down(KEY_RIGHT);

            self.update(dt, up, down, left, right);
            self.draw();
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.run();
}
