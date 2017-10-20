use rand::Rng;

use super::{Optimizer, OptimizerState};
use super::super::{NeighborCandidate, Problem};

pub struct HillClimber<P: Problem> {
    current_candidate: P::CandidateType,
    current_score: u32
}

impl<P: Problem> Optimizer<P> for HillClimber<P> where P::CandidateType: NeighborCandidate {
    fn advance_state(&mut self, _: f64, p: &P, rng: &mut Rng) -> OptimizerState {
        let mut found_improvement = false;

        for n in self.current_candidate.neighbors() {
            let n_score = p.score(&n);
            if n_score < self.current_score {
                self.current_candidate = n;
                self.current_score = n_score;
                found_improvement = true;
            }
        }

        if found_improvement {
            OptimizerState::Continue
        } else {
            OptimizerState::Done
        }
    }

    fn extract_solutions(&mut self) -> Vec<P::CandidateType> {
        vec![self.current_candidate.clone()]
    }
}