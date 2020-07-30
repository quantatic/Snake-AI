use crate::agent::Agent;

use rand::thread_rng;
use rand::seq::SliceRandom;

use rayon::prelude::*;

#[derive(Debug)]
pub struct Population<T> where
    T: Agent
{
    agents: Vec<T>
}

impl<T> Population<T> where
    T: Agent + Sync
{
    pub fn new(agents: Vec<T>) -> Self {
	Population {
	    agents
	}
    }

    pub fn get_best(&self) -> (&T, f64) {
	let (best, score) = self.agents.par_iter()
	    .map(|agent_ref: &T| {
		(agent_ref, agent_ref.fitness())
	    })
	    .max_by(|&(_, score1): &(&T, f64), &(_, score2): &(&T, f64)| {
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
	let mut agent_scores: Vec<(&T, f64)> = self.agents.par_iter()
	    .map(|agent_ref: &T| {
		(agent_ref, agent_ref.fitness())
	    })
	    .collect();

	agent_scores.par_sort_unstable_by(|&(_, val1): &(&T, f64), &(_, val2): &(&T, f64)| {
	    val1.partial_cmp(&val2).unwrap()
	});

        let percentage_top_agents = 0.1;
        let num_top_agents = (percentage_top_agents * (agent_scores.len() as f64)) as usize;
	let agents_to_breed_with = agent_scores.iter()
	    .rev()
	    .take(num_top_agents)
	    .map(|&(agent_ref, val): &(&T, f64)| {
		agent_ref
	    })
	    .collect::<Vec<_>>();

        new_agents.push(agents_to_breed_with[0].clone());
	while new_agents.len() < self.agents.len() {
	    let agent1 = *agents_to_breed_with.choose(&mut rng).unwrap();
/*
            let mut agent2 = agent1;
	    while (agent1 as *const T) == (agent2 as *const T) {
		agent2 = agents_to_breed_with.choose(&mut rng).unwrap();
	}
*/

            new_agents.push(agent1.mutate());
	}

	Self {
            agents: new_agents
	}
    }
}
