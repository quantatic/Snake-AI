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
	let agent_scores: Vec<(&T, f64)> = self.agents.iter()
	    .zip(self.evaluate())
	    .collect();

	for _ in self.agents.iter() {
	    let &(agent1, _) = agent_scores
		.choose_weighted(&mut rng, |&(_, score): &(&T, f64)| {
		    score
		}).unwrap();
	    let mut agent2 = agent1;
	    while (agent1 as *const T) == (agent2 as *const T) {
		let &(chosen_agent, _) = agent_scores
		    .choose_weighted(&mut rng, |&(_, score): &(&T, f64)| {
			score
		    }).unwrap();

		agent2 = chosen_agent;
	    }

	    //new_agents.push(agent1.crossover(&agent2).mutate());
	    new_agents.push(agent1.mutate());
	}

	Self {
            agents: new_agents
	}
    }
}
