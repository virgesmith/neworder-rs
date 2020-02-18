
#[cfg(test)]
mod test {

  use crate::neworder as no;
  use crate::python;
  use crate::env;

  use pyo3::prelude::*;
  use pyo3::{Python};//, PyResult};
  use pyo3::types::*; 
  
    #[test]
  fn test_mc() {

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