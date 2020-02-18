
use pyo3::prelude::*;
use rand::gen::pseudo::MT19937;
use rand::gen::RandomStream;
use rand::gen::Resettable;
use mpi::topology::Rank;

use numpy::PyArray1;

const BASE_SEED: u32 = 19937;


// compute the RNG seed
fn compute_seed(rank: Rank, size: Rank, indep: bool) -> u32
{
  // ensure stream (in)dependence w.r.t. sequence and MPI rank/sizes
  77027473 * 0 + BASE_SEED * (size as u32) + (rank as u32) * (indep as u32)
}


#[pyclass]
pub struct MonteCarlo {
  indep: bool,
  // seed
  seed: u32,
  rng: MT19937
}

// not visible to python
impl MonteCarlo {
  pub fn new(rank: Rank, size: Rank, indep: bool) -> MonteCarlo {
    let seed = compute_seed(rank, size, indep);
    MonteCarlo{ indep: indep, seed: seed, rng: MT19937::new(Some(seed)) }
  }

  // [arg] the trait `pyo3::type_object::PyTypeInfo` is not implemented for `std::vec::Vec<f64>
  fn hazard_a(&mut self, probs: &[f64]) -> Vec<f64> {
    self.rng.uniforms01(probs.len()).iter().zip(probs).map(|(x, p)| match x < p { true => 1.0, false => 0.0 }).collect()
  }

  fn stopping_a(&mut self, probs: &[f64]) -> Vec<f64> {
    //return np::unary_op<double, double>(prob, [&](double p) { return -::log(dist(m_prng)) / p; });
    self.rng.uniforms01(probs.len()).iter().zip(probs).map(|(x, p)| -(x.ln() / p) ).collect()
  }
}

#[pymethods]
impl MonteCarlo {

  pub fn indep(&self) -> bool {
    self.indep
  }

  pub fn seed(&self) -> u32 {
    self.seed
  }

  pub fn reset(&mut self) {
    self.rng.reset();
  }

  pub fn ustream(&mut self, n: usize) -> Vec::<f64> {
    self.rng.uniforms01(n)
  } 

  // simple hazard constant probability 
  fn hazard(&mut self, prob: f64, n: usize) -> Vec<f64> {
    self.rng.uniforms01(n).iter().map(|x| match x < &prob { true => 1.0, false => 0.0 }).collect()
  }

  // [arg] the trait `pyo3::type_object::PyTypeInfo` is not implemented for `std::vec::Vec<f64>
  #[name="hazard_a"]
  fn hazard_a_py(&mut self, probs: &PyArray1::<f64>) -> Vec<f64> {
    self.hazard_a(probs.as_slice().unwrap())
  }

  fn stopping(&mut self, prob: f64, n: usize) -> Vec<f64> {
    let rp = 1.0 / prob;
    self.rng.uniforms01(n).iter().map(|x| -(x.ln() * rp)).collect()
  }

  // [arg] the trait `pyo3::type_object::PyTypeInfo` is not implemented for `std::vec::Vec<f64>
  #[name="stopping_a"]
  fn stopping_a_py(&mut self, probs: &PyArray1::<f64>) -> Vec<f64> {
    self.stopping_a(probs.as_slice().unwrap())
  }
}

