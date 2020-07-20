use rand::{Rng, thread_rng};
use rand::distributions::Standard;

use crate::network::Network;
use crate::snake;

pub trait Agent {
    fn fitness(&self) -> f64;
    fn crossover(&self, other: &Self) -> Self;
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
    const SNAKE_STEPS: usize = 1000;
    const NETWORK_INNER_LAYERS: [usize; 4] = [50, 50, 50, 50];

    pub fn new() -> Self{
        let mut network_size = Snake::NETWORK_INNER_LAYERS.to_vec();
        network_size.insert(0, 1); // 1 input value
        network_size.push(4);      // 4 output values
        
        Self {
            rules: Network::new(network_size)
        }
    }

    fn get_next_press(&self, stats: snake::GameStats) -> snake::Direction {
        let network_inputs = vec![
            stats.distance_to_wall
        ];

        let network_result = self.rules.evaluate(network_inputs);
        let (selected_choice, selected_value) = network_result.iter().copied()
            .enumerate()
            .max_by(|&(_, val_one), &(_, val_two)| {
                val_one.partial_cmp(&val_two).unwrap()
            })
            .unwrap();

        match selected_choice {
            0 => snake::Direction::Up,
            1 => snake::Direction::Right,
            2 => snake::Direction::Down,
            3 => snake::Direction::Left,
            other => panic!("Unhandled network output: {}", other)
        }
    }
}

impl Agent for Snake {
    fn fitness(&self) -> f64 {
        let mut game = snake::Game::new(50, 50, 0);
        for steps_lasted in 0..Snake::SNAKE_STEPS {
            if let snake::GameStatus::InProgress(stats) = game.step() {
                let button_press = self.get_next_press(stats);
                game.turn_snake(button_press);
            } else {
                if steps_lasted >= 50 {
                    println!("{}", steps_lasted);
                }
                return steps_lasted as f64;
            }
        }

        println!("lasted entire time!");
        Snake::SNAKE_STEPS as f64
    }

    fn crossover(&self, other: &Self) -> Self {
        Self {
            rules: self.rules.merge(&other.rules)
        }
    }
}
