use crate::matrix::Matrix;

use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Clone)]
pub struct Network {
    weights: Vec<Matrix<f64>>,
    biases: Vec<Matrix<f64>>,
    shape: Vec<usize>,
}

impl Network {
    const NODE_COLOR: Color = Color::RGB(255, 0, 0);
    const CONNECTION_COLOR: Color = Color::RGB(0, 255, 0);

    pub fn new(shape: Vec<usize>) -> Self {
        let mut rng = thread_rng();

        let weights = shape
            .iter()
            .zip(shape[1..].iter())
            .map(|(&input_size, &output_size): (&usize, &usize)| {
                Matrix::new_map(input_size, output_size, |_row: usize, _col: usize| {
                    rng.sample(StandardNormal)
                })
            })
            .collect::<Vec<Matrix<f64>>>();

        let biases = shape[1..]
            .iter()
            .map(|&layer_size: &usize| {
                Matrix::new_map(1, layer_size, |_row: usize, _col: usize| {
                    rng.sample(StandardNormal)
                })
            })
            .collect::<Vec<Matrix<f64>>>();

        Self {
            weights,
            biases,
            shape,
        }
    }

    pub fn evaluate(&self, values: Vec<f64>) -> Vec<f64> {
        let input: Matrix<f64> =
            Matrix::new_map(1, values.len(), |_row: usize, col: usize| values[col]);

        let result_matrix =
            self.weights
                .iter()
                .zip(self.biases.iter())
                .fold(input, |total, (weight, bias)| {
                    let multiplied = &total * weight;
                    let biased = &multiplied + bias;
                    biased.map(|val: f64| Network::sigmoid(val))
                });

        (0..result_matrix.get_width())
            .map(|i| result_matrix[0][i])
            .collect::<Vec<f64>>()
    }

    pub fn get_shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn merge(&self, other: &Self) -> Self {
        let mut rng = thread_rng();
        let new_weights = self
            .weights
            .iter()
            .zip(other.weights.iter())
            .map(|(layer_one, layer_two)| {
                Matrix::new_map(layer_one.get_height(), layer_one.get_width(), |row, col| {
                    if rng.gen_bool(0.5) {
                        layer_one[row][col]
                    } else {
                        layer_two[row][col]
                    }
                })
            })
            .collect();

        let new_biases = self
            .biases
            .iter()
            .zip(other.weights.iter())
            .map(|(bias_one, bias_two)| {
                Matrix::new_map(bias_one.get_height(), bias_one.get_width(), |row, col| {
                    if rng.gen_bool(0.5) {
                        bias_one[row][col]
                    } else {
                        bias_two[row][col]
                    }
                })
            })
            .collect();

        Self {
            weights: new_weights,
            biases: new_biases,
            shape: self.shape.clone(),
        }
    }

    pub fn mutate(&self, mutation_prob: f64, mutation_amount: f64) -> Self {
        let mut rng = thread_rng();
        let new_weights = self
            .weights
            .iter()
            .map(|layer| {
                Matrix::new_map(layer.get_height(), layer.get_width(), |row, col| {
                    let mut res = layer[row][col];
                    if rng.gen_bool(mutation_prob) {
                        res += rng.gen_range(-mutation_amount..mutation_amount)
                    };

                    res
                })
            })
            .collect();

        let new_biases = self
            .biases
            .iter()
            .map(|bias| {
                Matrix::new_map(bias.get_height(), bias.get_width(), |row, col| {
                    let mut res = bias[row][col];
                    if rng.gen_bool(mutation_prob) {
                        res += rng.gen_range(-mutation_amount..mutation_amount);
                    }

                    res
                })
            })
            .collect();

        Self {
            weights: new_weights,
            biases: new_biases,
            shape: self.shape.clone(),
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, scale: u16) {
        let x_offset = i32::from(4 * scale);
        let y_offset = i32::from(4 * scale);
        let space_between = 5;

        for (start_layer, (&start_layer_size, &end_layer_size)) in
            self.shape.iter().zip(self.shape[1..].iter()).enumerate()
        {
            for start_node in 0..start_layer_size {
                let start_point = Point::new(
                    ((start_layer as i32) * i32::from(scale) * space_between)
                        + i32::from(scale / 2)
                        + x_offset,
                    ((start_node as i32) * i32::from(scale) * space_between)
                        + i32::from(scale / 2)
                        + y_offset,
                );
                for end_node in 0..end_layer_size {
                    let end_point = Point::new(
                        (((start_layer + 1) as i32) * i32::from(scale) * space_between)
                            + i32::from(scale / 2)
                            + x_offset,
                        ((end_node as i32) * i32::from(scale) * space_between)
                            + i32::from(scale / 2)
                            + y_offset,
                    );
                    canvas.set_draw_color(Self::CONNECTION_COLOR);
                    canvas.draw_line(start_point, end_point).unwrap();
                }
            }
        }

        for (layer, &layer_size) in self.shape.iter().enumerate() {
            for node in 0..layer_size {
                canvas.set_draw_color(Self::NODE_COLOR);
                canvas
                    .fill_rect(Rect::new(
                        ((layer as i32) * i32::from(scale) * space_between) + x_offset,
                        ((node as i32) * i32::from(scale) * space_between) + y_offset,
                        scale.into(),
                        scale.into(),
                    ))
                    .unwrap();
            }
        }
    }

    pub fn sigmoid(val: f64) -> f64 {
        1.0 / (1.0 + f64::exp(-val))
    }
}
