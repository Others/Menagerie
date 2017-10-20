use std::time::{Duration, Instant};

use rand::Rng;

use super::Problem;

fn microseconds(d: Duration) -> f64 {
    (d.as_secs() * 1000000) as f64 + (d.subsec_nanos() as f64 / 1000.0)
}

#[derive(Debug, Eq, PartialEq)]
pub enum OptimizerState {
    Continue,
    Done
}

pub trait Optimizer<P: Problem> {
    fn optimize_for(&mut self, rng: &mut Rng,  p: &P, duration: Duration) {
        self.optimize_until(rng, p, Instant::now() + duration)
    }

    fn optimize_until(&mut self, rng: &mut Rng, p: &P, deadline: Instant) {
        let total_microseconds = microseconds(deadline - Instant::now());
        let mut state = OptimizerState::Continue;
        while state == OptimizerState::Continue && Instant::now() < deadline {
            let remaining_microseconds = microseconds(deadline - Instant::now());
            let completion_hint = remaining_microseconds / total_microseconds;
            state = self.advance_state(completion_hint, p, rng);
        }
    }

    fn advance_state(&mut self, completion_hint: f64, p: &P, rng: &mut Rng) -> OptimizerState;

    fn extract_solutions(&mut self) -> Vec<P::CandidateType>;
}

pub mod cuckoo_search;
pub mod hill_climbing;
pub mod simulated_annealing;