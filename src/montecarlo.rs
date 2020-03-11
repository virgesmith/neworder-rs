
use crate::timeline;
use pyo3::prelude::*;
use rand::gen::pseudo::MT19937;
use rand::gen::{RandomStream, Dimensionless, Resettable};
use mpi::topology::Rank;

use numpy::PyArray1;

use std::cmp;

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

  // simple hazard constant probability 
  fn hazard(&mut self, prob: f64, n: usize) -> Vec<f64> {
    self.rng.uniforms01(n).iter().map(|x| match x < &prob { true => 1.0, false => 0.0 }).collect()
  }

  // [arg] the trait `pyo3::type_object::PyTypeInfo` is not implemented for `std::vec::Vec<f64>
  fn hazard_a(&mut self, probs: &[f64]) -> Vec<f64> {
    self.rng.uniforms01(probs.len()).iter().zip(probs).map(|(x, p)| match x < p { true => 1.0, false => 0.0 }).collect()
  }

  fn stopping(&mut self, prob: f64, n: usize) -> Vec<f64> {
    let rp = 1.0 / prob;
    self.rng.uniforms01(n).iter().map(|x| -(x.ln() * rp)).collect()
  }

  fn stopping_a(&mut self, probs: &[f64]) -> Vec<f64> {
    //return np::unary_op<double, double>(prob, [&](double p) { return -::log(dist(m_prng)) / p; });
    self.rng.uniforms01(probs.len()).iter().zip(probs).map(|(x, p)| -(x.ln() / p) ).collect()
  }
  
//   // multiple-arrival (0+) process 
// np::array no::MonteCarlo::arrivals(const np::array& lambda_t, double dt, double gap, size_t n)
// {
//   std::uniform_real_distribution<> dist(0.0, 1.0);

//   const double* pl = np::cbegin<double>(lambda_t);
//   size_t nl = lambda_t.size();

//   // validate lambdas - but what exactly is valid?
//   if (pl[nl-1] != 0.0)
//   {
//     throw std::runtime_error("Multiple-arrival Non-homogeneous Poisson process requires a zero final hazard rate");
//   }

//   // What is the optimal lambda_u? For now largest value
//   double lambda_u = *std::max_element(pl, pl + nl);
//   double lambda_i;

//   std::vector<std::vector<double>> times(n);

//   double tmax = (nl - 1) * dt;
//   size_t imax = 0;

//   for (size_t i = 0; i < n; ++i)
//   {
//     // rejection sampling
//     double pt = 0.0;
//     do 
//     {
//       do 
//       {
//         pt += -::log(dist(m_prng)) / lambda_u;
//         // final entry in lambda_t is flat extrapolated...
//         lambda_i = pl[ std::min((size_t)(pt / dt), nl-1) ];
//         if (pt > tmax && lambda_i == 0.0)
//         {
//           pt = no::Timeline::never();
//           break;
//         }
//       } while (dist(m_prng) > lambda_i / lambda_u);
//       times[i].push_back(pt);
//       pt += gap;
//     } while (pt < tmax);
//     imax = std::max(times[i].size(), imax);
//     //no::log("%%: %%"_s % i % times[i]);
//   }

//   np::array nptimes = np::empty<double>({n, imax- 1});
//   np::fill(nptimes, no::Timeline::never());
//   double* pa = np::begin<double>(nptimes);

//   for (size_t i = 0; i < times.size(); ++i)
//   {
//     for (size_t j = 0; j < times[i].size() - 1; ++j)
//     {
//       pa[j] = times[i][j];
//     }
//     pa += imax - 1;
//   }

//   return nptimes;
// }
  pub fn first_arrival(&mut self, lambda_t: &[f64], dt: f64, n: usize, minval: f64) -> Vec<f64>
  {
    let nl = lambda_t.len();
    let lambda_u = lambda_t.iter().fold(std::f64::NEG_INFINITY, |a, &b| a.max(b));
    let mut lambda_i;

    let mut times = vec![0.0; n];
    let tmax = (nl - 1) as f64 * dt;

    for i in 0..n {
      times[i] = minval;
      loop {
        times[i] += -self.rng.uniform01().ln() / lambda_u;

        lambda_i = lambda_t[cmp::min((times[i] / dt) as usize, nl-1)];
        // deal with open case (event not certain to happen)
        if times[i] > tmax && lambda_i == 0.0 {
          times[i] = timeline::NEVER;
          break;
        }
        if self.rng.uniform01() <= lambda_i / lambda_u { break; }
      } 
    }
    times
  }
// np::array no::MonteCarlo::first_arrival(const np::array& lambda_t, double dt, size_t n, double minval)
// {
//   std::uniform_real_distribution<> dist(0.0, 1.0);

//   const double* pl = np::cbegin<double>(lambda_t);
//   size_t nl = lambda_t.size();

//   // What is the optimal lambda_u? For now largest value
//   double lambda_u = *std::max_element(pl, pl + nl);
//   double lambda_i;

//   np::array times = np::empty_1d_array<double>(n);
//   double* pt = np::begin<double>(times);
//   double tmax = (nl - 1) * dt;

//   for (size_t i = 0; i < n; ++i)
//   {
//     // rejection sampling
//     pt[i] = minval;
//     do 
//     {
//       pt[i] += -::log(dist(m_prng)) / lambda_u;
//       // final entry in lambda_t is flat extrapolated...
//       lambda_i = pl[ std::min((size_t)(pt[i] / dt), nl-1) ];
//       // deal with open case (event not certain to happen)
//       if (pt[i] > tmax && lambda_i == 0.0)
//       {
//         pt[i] = no::Timeline::never();
//         break;
//       }
//     } while (dist(m_prng) > lambda_i / lambda_u);
//   }
//   return times;
// }

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
  #[name="hazard"]
  fn hazard_py(&mut self, py: Python, prob: f64, n: usize) -> Py<PyArray1::<f64>> { //Vec<f64> {
    let res = PyArray1::from_vec(py, self.hazard(prob, n));
    res.to_owned()
  
  }

  // [arg] the trait `pyo3::type_object::PyTypeInfo` is not implemented for `std::vec::Vec<f64>
  #[name="hazard_a"]
  fn hazard_a_py(&mut self, py: Python, probs: &PyArray1::<f64>) -> Py<PyArray1::<f64>> {
    let res = PyArray1::from_vec(py, self.hazard_a(probs.as_slice().unwrap()));
    res.to_owned()
  }

  #[name="stopping"]
  fn stopping_py(&mut self, py: Python, prob: f64, n: usize) -> Py<PyArray1::<f64>> {
    let res = PyArray1::from_vec(py, self.stopping(prob, n));
    res.to_owned()
  }

  // [arg] the trait `pyo3::type_object::PyTypeInfo` is not implemented for `std::vec::Vec<f64>
  #[name="stopping_a"]
  fn stopping_a_py(&mut self, py: Python, probs: &PyArray1::<f64>) -> Py<PyArray1::<f64>> {
    let res = PyArray1::from_vec(py, self.stopping_a(probs.as_slice().unwrap()));
    res.to_owned()
  }
}

