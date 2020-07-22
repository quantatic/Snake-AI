use crate::agent::Agent;

use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Population<T> where
    T: Agent
{
    agents: Vec<T>
}

impl<T> Population<T> where
    T: Agent
{
    pub fn new(agents: Vec<T>) -> Self {
	Population {
	    agents
	}
    }

    pub fn evaluate(&self) -> Vec<f64> {
        self.agents
	    .iter()
	    .map(Agent::fitness)
	    .collect()
    }

    pub fn get_best(&self) -> (&T, f64) {
	let (score, best) = self.evaluate().into_iter()
	    .zip(self.agents.iter())
	    .max_by(|&(score1, _): &(f64, &T), &(score2, _): &(f64, &T)| {
		score1.partial_cmp(&score2).unwrap()
	    })
	    .unwrap();

	(best, score)
    }

    pub fn breed(&self) -> Self {
	if self.agents.len() < 2 {
	    panic!("Cannot breed with less than 2 agents");
	}

	let mut rng = thread_rng();

	let mut new_agents: Vec<T> = Vec::new();
	let mut agent_scores: Vec<(&T, f64)> = self.agents.iter()
	    .zip(self.evaluate())
	    .collect();

	agent_scores.sort_unstable_by(|&(_, val1): &(&T, f64), &(_, val2): &(&T, f64)| {
	    val1.partial_cmp(&val2).unwrap()
	});

	let num_top_agents = 20;
	let num_bottom_agents = 3;
	let agents_to_breed_with = agent_scores.iter().take(num_bottom_agents)
	    .chain(agent_scores.iter().skip(num_top_agents).rev().take(num_top_agents))
	    .map(|&(agent_ref, val): &(&T, f64)| {
		agent_ref
	    })
	    .collect::<Vec<_>>();

	for _ in 0..self.agents.len() {
	    let agent1 = *agents_to_breed_with.choose(&mut rng).unwrap();
	    let mut agent2 = agent1;
	    while (agent1 as *const T) == (agent2 as *const T) {
		agent2 = agents_to_breed_with.choose(&mut rng).unwrap();
	    }

	    new_agents.push(agent1.crossover(&agent2).mutate());
	}

	Self {
            agents: new_agents
	}
    }
}
