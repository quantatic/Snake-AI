use rand::{Rng, thread_rng};
use rand::distributions::Standard;

use sdl2::video::Window;
use sdl2::render::Canvas;

use crate::network::Network;
use crate::snake;

use std::time::Duration;

pub trait Agent {
    fn fitness(&self) -> f64;
    fn crossover(&self, other: &Self) -> Self;
    fn mutate(&self) -> Self;
}

pub struct Binary {
    vals: Vec<bool>,
    mutation_prob: f64
}

impl Binary {
    pub fn new(len: usize, mutation_prob: f64) -> Self {
	let rng = thread_rng();
	let vals = rng
	    .sample_iter(Standard)
	    .take(len)
	    .collect();

	Self {
	    vals,
            mutation_prob
	}
    }
}

impl Agent for Binary {
    fn fitness(&self) -> f64 {
	let prefitness = self.vals
	    .iter()
	    .filter(|&&val| {
		val
	    })
	    .count() as f64;

	f64::powi(prefitness, 40)
    }

    fn crossover(&self, other: &Self) -> Self {
	let mut rng = thread_rng();

	// Choose either the first or second value with equal probability, then
	// mutate each element with a given probability.
	let new_vals: Vec<_> = self.vals.iter()
	    .zip(other.vals.iter())
	    .map(move |(&val1, &val2)| {
		match rng.gen_bool(0.5) {
		    true => val1,
		    false => val2
		}
	    })
	    .collect();

	Self {
	    vals: new_vals,
            mutation_prob: self.mutation_prob
	}
    }

    fn mutate(&self) -> Self {
	let mut rng = thread_rng();
	
	let new_vals: Vec<_> = self.vals.iter()
	    .map(move |val| {
		val ^ rng.gen_bool(self.mutation_prob)
	    })
	    .collect();

	Self {
	    vals: new_vals,
	    mutation_prob: self.mutation_prob
	}

    }
}

pub struct Snake {
    rules: Network
}

impl Snake {
    const SNAKE_STEPS: usize = 500;
    const NETWORK_INNER_LAYERS: [usize; 2] = [25, 25];

    pub fn new() -> Self{
        let mut network_size = Snake::NETWORK_INNER_LAYERS.to_vec();
        network_size.insert(0, 8); // 8 input values
        network_size.push(4);      // 4 output values
        
        Self {
            rules: Network::new(network_size)
        }
    }

    fn get_next_press(&self, stats: snake::GameStats) -> snake::Direction {
        let network_inputs = vec![
	    stats.distance_to_food_x,
	    stats.distance_to_food_y,
	    stats.distance_to_wall_up,
	    stats.distance_to_wall_right,
	    stats.distance_to_wall_down,
	    stats.distance_to_wall_left,
	    stats.distance_to_tail_x,
	    stats.distance_to_tail_y
        ];

        let network_result = self.rules.evaluate(network_inputs);
        let (selected_choice, selected_value) = network_result.iter().copied()
            .enumerate()
            .max_by(|&(_, val_one), &(_, val_two)| {
                val_one.partial_cmp(&val_two).unwrap()
            })
            .unwrap();

        match selected_choice {
            0 => snake::Direction::Right,
            1 => snake::Direction::Up,
            2 => snake::Direction::Down,
            3 => snake::Direction::Left,
            other => panic!("Unhandled network output: {}", other)
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
	let mut game = snake::Game::new(50, 50, 16);
        for _ in 0..Snake::SNAKE_STEPS {
            if let snake::GameStatus::InProgress(stats) = game.step() {
		game.render(canvas);
                let button_press = self.get_next_press(stats);
                game.turn_snake(button_press);
		canvas.present();
		std::thread::sleep(Duration::new(0, 1_000_000_000u32 / (100 as u32)));
            } else {
		break;
            }
        }
    }

}

impl Agent for Snake {
    fn fitness(&self) -> f64 {
        let mut game = snake::Game::new(50, 50, 0);
	let mut score = 0;
	let mut maybe_last_stats: Option<snake::GameStats> = None;
	let mut steps_taken = 0;
        for step in 0..Snake::SNAKE_STEPS {
            if let snake::GameStatus::InProgress(stats) = game.step() {
		score = u32::max(score, stats.score);
                let button_press = self.get_next_press(stats);
                game.turn_snake(button_press);
		maybe_last_stats = Some(stats);
		steps_taken = step;
            } else {
		break;
		steps_taken = step;
            }
        }

	let last_stats = maybe_last_stats.unwrap();
	let distance_to_food = f64::sqrt((last_stats.distance_to_food_x * last_stats.distance_to_food_x) + (last_stats.distance_to_food_y * last_stats.distance_to_food_y));
	let max_distance_to_food = f64::sqrt(((game.get_width() * game.get_width()) + (game.get_height() * game.get_height())) as f64);
	
	f64::powi((score as f64) - (distance_to_food / max_distance_to_food) - ((steps_taken / Snake::SNAKE_STEPS) as f64), 5)
    }

    fn crossover(&self, other: &Self) -> Self {
        Self {
            rules: self.rules.merge(&other.rules)
        }
    }

    fn mutate(&self) -> Self {
	Self {
	    rules: self.rules.mutate(0.1, 2.0)
	}
    }
}
