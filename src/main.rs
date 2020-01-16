
use pyo3::{Python, PyResult};
use pyo3::types::PyModule;
use mpi::topology::Communicator;

fn append_model_paths(paths: &[String]) {

  let pypath = match std::env::var("PYTHON_PATH") {
    Ok(val) => val + ":",
    Err(_) => "".to_string()
  } + &paths.join(":");

  std::env::set_var("PYTHON_PATH", pypath);
}

fn main() -> PyResult<()> {

  let args = std::env::args().collect::<Vec<String>>();

  if args.len() < 2 {
    println!("usage: neworder <model-path> [<extra-path>...]");
    std::process::exit(1)
  }

  append_model_paths(&args[1..]);

  let gil = Python::acquire_gil();
  run(gil.python())?;

  Ok(())
}

fn run(_py: Python) -> PyResult<()> {

  let universe = mpi::initialize().unwrap();
  let world = universe.world();
  let size = world.size();
  let rank = world.rank();

  println!("MPI {}/{}", rank, size);
  
  println!("PYTHON_PATH={}", std::env::var("PYTHON_PATH").unwrap());

  Ok(())
}