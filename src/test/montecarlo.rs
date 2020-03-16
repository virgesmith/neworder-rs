
#[cfg(test)]
mod test {

  use crate::neworder as no;
  use crate::python;
  use crate::env;
  use crate::montecarlo::MonteCarlo;

  use pyo3::prelude::*;
  use pyo3::{Python};//, PyResult};
  use pyo3::types::*; 
  
  #[test]
  fn test_mt19937() {
    let mut mc = MonteCarlo::new(0, 1, true); 
    assert!(mc.seed() == 19937);
    let r = mc.ustream(5); 

    let scale = 0.5 / (1u32 << 31) as f64;
    assert!(r[0] == 1450791966u32 as f64 * scale); // 0.3377888272516429 
    assert!(r[1] ==  204743920u32 as f64 * scale); // 0.04767065867781639 
    assert!(r[2] == 3492290356u32 as f64 * scale); // 0.8131122114136815 
    assert!(r[3] == 1071801876u32 as f64 * scale); // 0.249548320658505 
    assert!(r[4] == 1454088227u32 as f64 * scale); // 0.3385562978219241

    assert!((r[0] - 0.3377888272516429).abs() < 1e-15);
    assert!((r[1] - 0.04767065867781639).abs() < 1e-15);
    assert!((r[2] - 0.8131122114136815 ).abs() < 1e-15);
    assert!((r[3] - 0.249548320658505).abs() < 1e-15);
    assert!((r[4] - 0.3385562978219241).abs() < 1e-15);
  }

  // no::MonteCarlo mc(0, 1, 19937); 
  // CHECK(mc.seed() == 19937);
  // CHECK(mc.indep());
  // py::array a = mc.ustream(5);
  // no::log(a);
  // CHECK(fabs(no::at<double>(a,{0}) - 0.33778882725164294) < 1e-8);
  // CHECK(fabs(no::at<double>(a,{1}) - 0.04767065867781639) < 1e-8);
  // CHECK(fabs(no::at<double>(a,{2}) - 0.8131122114136815) < 1e-8);
  // CHECK(fabs(no::at<double>(a,{3}) - 0.24954832065850496) < 1e-8);
  // CHECK(fabs(no::at<double>(a,{4}) - 0.3385562978219241) < 1e-8);

  // mc.reset();
  // py::array h = mc.hazard(0.5, 1000000);
  // CHECK(no::sum<int>(h) == 500151)


  #[test]
  fn test_hazard() {
    let mut mc = MonteCarlo::new(0, 1, true); 
    let h = mc.hazard(0.5, 1000000);
    assert!(h.iter().sum::<i32>() == 500151); 
  }

  #[test]
  fn test_mc_py() {

    // fails with mpi
    if env::size() > 1 { return; }

    let gil = Python::acquire_gil();
    let py = gil.python();
  
    let neworder = no::init_embedded(py).unwrap();
    let locals = [("neworder", neworder)].into_py_dict(py);

    let runtime = python::Runtime::new(py, None, Some(locals));

    let mc_rs = no::init_mc(py, true, neworder);
    let mc_py = neworder.get("mc").unwrap().to_object(py); //unwrap();

    // use eval to directly exec python...

    //const py::object& mc = neworder.attr("mc"); 
    //assert!(mc_py.getattr(py, "indep").unwrap().call0(py).unwrap().extract::<bool>(py).unwrap());
    assert!(runtime.run(&("neworder.mc.indep()", python::CommandType::Eval)).unwrap().extract::<bool>(py).unwrap());
    assert_eq!(runtime.run(&("neworder.mc.seed()", python::CommandType::Eval)).unwrap().extract::<u32>(py).unwrap(), 19937);
    assert_eq!(mc_py.getattr(py, "seed").unwrap().call0(py).unwrap().extract::<u32>(py).unwrap(), 19937);
    assert!(mc_rs.indep(), true);
    assert_eq!(mc_rs.seed(), (19937 * env::size() + env::rank()) as u32);
    
    // check MC object state is shared between C++ and python
    let h01_rs = mc_rs.ustream(2); 
    // py::array_t<double> h23_py = mc.attr("ustream")(2);
    // let h23_py_ffs/*: &PyArray::<f64>*/ = mc_py.getattr(py, "ustream").unwrap().call(py, PyTuple::new(py, &[2]), None).unwrap();
    // let h23_py = h23_py_ffs.extract::<&PyArray1::<f64>>(py).unwrap();
    let h23_py = runtime.run(&("neworder.mc.ustream(2)", python::CommandType::Eval)).unwrap().extract::<Vec<f64>>(py).unwrap();
    // values should not match (0,1) != (2,3)
    assert!(h01_rs[0] != h23_py[0]);
    assert!(h01_rs[1] != h23_py[1]);
    // reset from rust
    mc_rs.reset();
    // sample from python
    let h01_py = runtime.run(&("neworder.mc.ustream(2)", python::CommandType::Eval)).unwrap().extract::<Vec<f64>>(py).unwrap();
    assert_eq!(h01_rs[0], h01_py[0]);
    assert_eq!(h01_rs[1], h01_py[1]); 

    // sample from rust
    let h23_rs = mc_rs.ustream(2);
    // values should match  
    assert_eq!(h23_rs[0], h23_py[0]);
    assert_eq!(h23_rs[1], h23_py[1]);
    // reset from python
    runtime.run(&("neworder.mc.reset()", python::CommandType::Exec)).unwrap();
    // sample from rust
    let h01_rs = mc_rs.ustream(2);
    // // values should still match (0,1) == (0,1)
    assert_eq!(h01_rs[0], h01_py[0]);
    assert_eq!(h01_rs[1], h01_py[1]); 
  }

}