use rand::Rng;

use sdl2::pixels::Color;
use sdl2::rect::{Point,Rect};
use sdl2::video::Window;
use sdl2::render::Canvas;

use std::convert::{TryFrom,TryInto};

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone, Copy, Debug)]
pub enum GameStatus {
    InProgress(GameStats),
    Over
}

#[derive(Clone, Copy, Debug)]
pub struct GameStats {
    pub distance_to_food: f64,
    pub distance_to_wall: f64,
    pub score: u32
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Location {
    x: i16,
    y: i16
}

impl Location {
    fn random_location(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            x: rng.gen_range(0, width).try_into().unwrap(),
            y: rng.gen_range(0, height).try_into().unwrap()
        }
    }
}

pub struct Game {
    width: u16,
    height: u16,
    tile_size: u16,
    snake: Vec<Location>, // the "front" of the snake is stored at the end
    snake_direction: Direction,
    food_loc: Location,
    game_in_progress: bool
}

impl Game {
    const BACKGROUND_COLOR: Color = Color::RGB(127, 127, 127);
    const SNAKE_COLOR: Color = Color::RGB(255, 0, 0);
    const FOOD_COLOR: Color = Color::RGB(0, 255, 0);
    const LINE_COLOR: Color = Color::RGB(0, 0, 255);
    
    pub fn new(width: u16, height: u16, tile_size: u16) -> Self {
        let mut snake = Vec::new();
        
        snake.push(
            Location{
                x: 4,
                y: 4
            }
        );

        Self {
            width,
            height,
            tile_size,
            snake,
            snake_direction: Direction::Right,
            food_loc: Location::random_location(width, height),
            game_in_progress: true
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        let head = self.snake.last().unwrap();

        canvas.set_draw_color(Self::BACKGROUND_COLOR);
        canvas.clear();

        for y in 0..self.height {
            for x in 0..self.width {
                let curr_loc = Location {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap()
                };
                
                let snake_here = self
                    .snake
                    .iter()
                    .any(|&snake_loc| {
                        snake_loc == curr_loc
                    });

                let food_here = (self.food_loc == curr_loc);

                if snake_here {
                    canvas.set_draw_color(Self::SNAKE_COLOR);
                    canvas.fill_rect(Rect::new(
                        (x * u16::from(self.tile_size)).into(),
                        (y * u16::from(self.tile_size)).into(),
                        self.tile_size.into(),
                        self.tile_size.into()
                    )).unwrap();
                } else if food_here {
                    canvas.set_draw_color(Self::FOOD_COLOR);
                    canvas.fill_rect(Rect::new(
                        (x * u16::from(self.tile_size)).into(),
                        (y * u16::from(self.tile_size)).into(),
                        self.tile_size.into(),
                        self.tile_size.into()
                    )).unwrap();
                }
            }
        }
    }

    pub fn get_snake_head_location(&self) -> Location {
        *self.snake.last().unwrap()
    }

    pub fn step(&mut self) -> GameStatus {
        if !self.game_in_progress {
            return GameStatus::Over;
        }
        
        let old_front = self.get_snake_head_location();
        let mut new_front = old_front;

        self.snake.rotate_left(1);
        self.snake.pop().unwrap();

        let out_of_bounds = match self.snake_direction {
            Direction::Up => {
                new_front.y -= 1;
                new_front.y < 0
            },
            Direction::Down => {
                new_front.y += 1;
                new_front.y >= self.height.try_into().unwrap()
            },
            Direction::Left => {
                new_front.x -= 1;
                new_front.x < 0
            },
            Direction::Right => {
                new_front.x += 1;
                new_front.x >= self.width.try_into().unwrap()
            }
        };

        
        let collision_with_self = self.snake
            .iter()
            .any(|&snake_loc| {
                snake_loc == new_front
            });

        if out_of_bounds || collision_with_self {
            self.snake.push(old_front);
            self.game_in_progress = false;
            return GameStatus::Over;
        }
        
        self.snake.push(new_front);

        if new_front == self.food_loc {
            let mut food_overlap = true;
            while food_overlap {
                self.food_loc = Location::random_location(self.width, self.height);
                food_overlap = self
                    .snake
                    .iter()
                    .any(|&snake_loc| {
                        snake_loc == self.food_loc
                    });
            }

            self.snake.insert(0, self.snake[0]);
        }

        let distance_to_food = f64::sqrt(
            f64::powi((new_front.x - self.food_loc.x).into(), 2)
                + f64::powi((new_front.y - self.food_loc.y).into(), 2)
        );

        let distance_to_wall: f64 = vec![
            new_front.x,
            i16::try_from(self.width).unwrap() - new_front.x,
            new_front.y,
            i16::try_from(self.height).unwrap() - new_front.y
        ]
            .into_iter()
            .min()
            .unwrap()
            .into();

        GameStatus::InProgress(
            GameStats {
                distance_to_food,
                distance_to_wall,
                score: self.snake.len() as u32
            }
        )
    }

    pub fn turn_snake(&mut self, direction: Direction) {
        self.snake_direction = direction;
    }
}
