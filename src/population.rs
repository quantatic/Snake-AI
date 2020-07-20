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

    pub fn breed(&self) -> Self {
	if self.agents.len() < 2 {
	    panic!("Cannot breed with less than 2 agents");
	}

	let mut rng = thread_rng();

	let mut new_agents: Vec<T> = Vec::new();

	for _ in self.agents.iter() {
	    let agent1 = self.agents.choose_weighted(&mut rng, Agent::fitness).unwrap();
	    let mut agent2 = agent1;
	    while (agent1 as *const T) == (agent2 as *const T) {
                agent2 = self.agents.choose_weighted(&mut rng, Agent::fitness).unwrap();
	    }

	    new_agents.push(agent1.crossover(&agent2));
	}

	Self {
            agents: new_agents
	}
    }
}
