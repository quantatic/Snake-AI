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
    pub distance_to_obstacle_up: f64,
    pub distance_to_obstacle_right: f64,
    pub distance_to_obstacle_down: f64,
    pub distance_to_obstacle_left: f64,
    pub distance_to_food_x: f64,
    pub distance_to_food_y: f64,
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
                x: 0,
                y: 0
            }
        );

        snake.push(
            Location{
                x: 1,
                y: 0
            }
        );
	
        snake.push(
            Location{
                x: 2,
                y: 0
            }
        );
	
        Self {
            width,
            height,
            tile_size,
            snake,
            snake_direction: Direction::Right,
	    food_loc: Location { x: 10, y: 15 },
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
		/*self.food_loc.x += 13;
		self.food_loc.x %= (self.width as i16);
		self.food_loc.y += 19;
		self.food_loc.y %= (self.height as i16);
		food_overlap = false;*/
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

	let distance_to_food_x = new_front.x - self.food_loc.x;
	let distance_to_food_y = new_front.y - self.food_loc.y;

	let mut distance_to_obstacle_up = new_front.y;
	let mut distance_to_obstacle_right = i16::try_from(self.width).unwrap() - new_front.x;
	let mut distance_to_obstacle_down = i16::try_from(self.height).unwrap() - new_front.y;
	let mut distance_to_obstacle_left = new_front.x;

	for snake_location in self.snake.iter().rev().skip(1).rev() {
	    let snake_bit_difference_x = new_front.x - snake_location.x; // positive if head to right, negative if left
	    let snake_bit_difference_y = snake_location.y - new_front.y; // positive if head above, negative if below

	    if snake_bit_difference_x == 0 {
		if snake_bit_difference_y >= 0 {
		    distance_to_obstacle_down = i16::min(distance_to_obstacle_down, snake_bit_difference_y);
		}

		if snake_bit_difference_y <= 0 {
		    distance_to_obstacle_up = i16::min(distance_to_obstacle_up, -snake_bit_difference_y);
		}
	    }

	    if snake_bit_difference_y == 0 {
		if snake_bit_difference_x >= 0 {
		    distance_to_obstacle_left = i16::min(distance_to_obstacle_left, snake_bit_difference_x);
		}

		if snake_bit_difference_x <= 0 {
		    distance_to_obstacle_right = i16::min(distance_to_obstacle_right, -snake_bit_difference_x);
		}
	    }
	}

        GameStatus::InProgress(
            GameStats {
		distance_to_food_x: distance_to_food_x.into(),
		distance_to_food_y: distance_to_food_y.into(),
		distance_to_obstacle_up: distance_to_obstacle_up.into(),
		distance_to_obstacle_right: distance_to_obstacle_right.into(),
		distance_to_obstacle_down: distance_to_obstacle_down.into(),
		distance_to_obstacle_left: distance_to_obstacle_left.into(),
                score: self.snake.len() as u32
            }
        )
    }

    pub fn turn_snake(&mut self, direction: Direction) {
        self.snake_direction = direction;
    }

    pub fn get_width(&self) -> u16 {
	self.width
    }

    pub fn get_height(&self) -> u16 {
	self.height
    }
}
