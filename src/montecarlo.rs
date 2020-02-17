
use pyo3::{Python, Py, PyResult};
use pyo3::prelude::*;
use numpy::PyArray1;
use rand::gen::pseudo::MT19937;
use rand::gen::RandomStream;
use rand::gen::Resettable;
use mpi::topology::Rank;


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

//   #[pyfunction]
// fn ustream(py: Python, n: usize) -> Py<PyArray1::<f64>> {

//   //PyArray1::from_vec(py, vec![0.0;n]).to_owned()
//   let res = PyArray1::from_vec(py, vec![0.0;n]);
//   res.to_owned()

// }


  // // simple hazard constant probability 
  // fn hazard(&mut self, prob: f64, n: usize) -> Vec<f64> {
  //   self.rng.uniforms01(n).iter().map(|x| match x < &prob { true => 1.0, false => 0.0 }).collect()
  // }

  // fn hazard_a(&self, probs: &Vec<f64>) -> Vec<f64> {
  //   self.rng.uniforms01(probs.len()).iter().zip(probs.iter()).map(|(x, p)| match x < p { true => 1.0, false => 0.0 }).collect()
  // }

  // fn stopping(&mut self, prob: f64, n: usize) -> Vec<f64> {
  //   let rp = 1.0 / prob;
  //   self.rng.uniforms01(n).iter().map(|x| -(x.ln() * rp)).collect()
  // }

  // // computes stopping times 
  // NEWORDER_EXPORT np::array no::MonteCarlo::stopping(double prob, size_t n)
  // {
  //   std::uniform_real_distribution<> dist(0.0, 1.0);
  //   double rprob = 1.0 / prob;

  //   return np::make_array<double>(n, [&]() { return -::log(dist(m_prng)) * rprob; });
  // }

  // np::array no::MonteCarlo::stopping(const np::array& prob)
  // {
  //   std::uniform_real_distribution<> dist(0.0, 1.0);

  //   return np::unary_op<double, double>(prob, [&](double p) { return -::log(dist(m_prng)) / p; });
  // }
}

