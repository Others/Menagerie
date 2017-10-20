extern crate nalgebra;
extern crate rand;

use nalgebra::DVector;

use rand::Rng;

pub mod optimizers;

pub trait Problem {
    type CandidateType: Candidate;

    fn random_candidate(&self, rng: &mut Rng) -> Self::CandidateType;

    fn score(&self, candidate: &Self::CandidateType) -> u32;
}

// Candidate traits
pub trait Candidate: Sized + Eq + Clone {}

pub trait NeighborCandidate: Candidate {
    fn neighbors(&self) -> Vec<Self>;
}

pub trait VecCandidate: Candidate {
    fn to_vec(&self) -> DVector<u32>;

    fn from_vec(vec: &DVector<u32>) -> Self;
}

// Candidate trait impls for u32
impl Candidate for u32 {
}

impl NeighborCandidate for u32 {
    fn neighbors(&self) -> Vec<Self> {
        vec![self.wrapping_sub(1), self.wrapping_add(1)]
    }
}

impl VecCandidate for u32 {
    fn to_vec(&self) -> DVector<u32> {
        DVector::from_element(1, self.clone())
    }

    fn from_vec(vec: &DVector<u32>) -> Self {
        assert!(vec.len() == 1);
        vec[0]
    }
}