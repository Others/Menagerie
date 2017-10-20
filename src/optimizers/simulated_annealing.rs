use rand::Rng;

use super::{Optimizer, OptimizerState};
use super::super::{NeighborCandidate, Problem};

pub struct Annealer<P: Problem> {
    current_candidate: P::CandidateType,
    current_score: u32
}

// TODO: Consider allowing alternative acceptance_probability functions
fn acceptance_probability(current_score: u32, new_score: u32, t: f64) -> f64 {
    if new_score < current_score {
        1.0
    } else {
        ((current_score - new_score) as f64 / t).exp()
    }
}

fn choose<T>(mut values: Vec<T>, rng: &mut Rng) -> Option<T> {
    if values.len() == 0 {
        None
    } else {
        // TODO: Make this fairly distributed
        let index = rng.next_u64() as usize % values.len();
        Some(values.swap_remove(index))
    }
}

impl<P: Problem> Optimizer<P> for Annealer<P> where P::CandidateType: NeighborCandidate {
    fn advance_state(&mut self, completion_hint: f64, p: &P, rng: &mut Rng) -> OptimizerState {
        let new_candidate = choose(self.current_candidate.neighbors(), rng).unwrap();
        let new_score = p.score(&new_candidate);

        if acceptance_probability(self.current_score, new_score, completion_hint) >= rng.next_f64() {
            self.current_candidate = new_candidate;
            self.current_score = new_score;
        }

        // Simulated annealing never will exit early
        OptimizerState::Continue
    }

    fn extract_solutions(&mut self) -> Vec<P::CandidateType> {
        vec![self.current_candidate.clone()]
    }
}