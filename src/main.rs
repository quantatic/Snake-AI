mod agent;
mod population;
mod snake;
mod matrix;
mod network;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::snake::{Direction, Game};

use crate::network::Network;
use crate::population::Population;


const FPS: u16 = 10;

const GAME_WIDTH: u16 = 50;
const GAME_HEIGHT: u16 = 50;
const GAME_SCALE: u16 = 16;
const NETWORK_SCALE: u16 = 20;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem
        .window("Snake Game", (GAME_WIDTH * GAME_SCALE).into(), (GAME_HEIGHT * GAME_SCALE).into())
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .unwrap();
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let mut game = Game::new(GAME_WIDTH, GAME_HEIGHT, GAME_SCALE);
    let mut population = Population::new(
	(0..100_000)
	    .map(|_| {
		agent::Snake::new()
	    })
	    .collect()
    );

    let mut generation = 1;
    loop {
        population = population.breed();
	let (best, best_score) = population.get_best();
	println!("Best score of generation {}: {}", generation, best_score);
	best.render(&mut canvas);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return
                }
		_ => {}
	    };
	}

	generation += 1;
    }

    return;

    let mut i = 0;
    loop {
        println!("{:?}", game.step());
        game.render(&mut canvas);
        //network.render(&mut canvas, NETWORK_SCALE);
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return
                },
                Event::KeyDown { keycode: Some(pressed_keycode), .. } => {
                    match pressed_keycode {
                        Keycode::Up => game.turn_snake(Direction::Up),
                        Keycode::Down => game.turn_snake(Direction::Down),
                        Keycode::Left => game.turn_snake(Direction::Left),
                        Keycode::Right => game.turn_snake(Direction::Right),
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / (FPS as u32)));
    }
}
