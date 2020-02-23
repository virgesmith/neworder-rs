
use mpi;
use mpi::topology::{Rank, Communicator};
use mpi::collective::CommunicatorCollectives;
use mpi::collective::Root;

use num::traits::Zero;

use std::error::Error;

use crate::neworder as no;

struct MPIEnv {
  _universe: mpi::environment::Universe,
  world: mpi::topology::SystemCommunicator,
}


impl MPIEnv {
  fn new() -> MPIEnv {
    let u = mpi::initialize().unwrap();
    let w = u.world();
    MPIEnv{ _universe: u, world: w }
  }
}

// TODO this data may need to be stored in the python module
lazy_static! {
  static ref MPI_ENV: MPIEnv = { MPIEnv::new() };

  //static ref PY_ENV: GILGuard = Arc::new(Python::acquire_gil());
}

pub fn rank() -> Rank {
  MPI_ENV.world.rank()
}

pub fn size() -> Rank {
  MPI_ENV.world.size()
}

pub fn world() -> &'static mpi::topology::SystemCommunicator {
  &MPI_ENV.world
}

#[allow(dead_code)]
pub fn rotate<T: mpi::datatype::Equivalence>(data: T) -> Result<T, Box<dyn Error>> {
  
  let rank = rank();
  let size = size();

  let (prev, next) = (world().process_at_rank((rank + size - 1) % size), 
                      world().process_at_rank((rank + 1) % size));

  let (data, _status) = mpi::point_to_point::send_receive(&data, &next, &prev);
  // TODO match status check for error
  Ok(data)
}

pub fn broadcast_from<T: mpi::datatype::Equivalence>(from: Rank, data: &mut T) -> Result<(), Box<dyn Error>> {
  let root_process = world().process_at_rank(from);  
  root_process.broadcast_into(data);
  Ok(())
}

pub fn scatter_from<T: Zero + mpi::datatype::Equivalence>(from: Rank, data: &Vec<T>) -> T { 

  let src = world().process_at_rank(from);

  let mut x = T::zero();
  match rank() == from {
    true => src.scatter_into_root(&data[..], &mut x),
    false => src.scatter_into(&mut x)
  };
  x
}

// Returns an Option containing an array in rank() == to
pub fn gather_into<T: mpi::datatype::Equivalence + Clone + Zero>(into: Rank, data: &T) -> Option<Vec<T>> { 

  let dst = world().process_at_rank(into);

  match rank() == into {
    true => {
      //let mut a = Vec::with_capacity(size() as usize);
      let mut a = vec![T::zero(); size() as usize];
      dst.gather_into_root(data, &mut a[..]);
      Some(a)
    },
    false => { 
      dst.gather_into(data);
      None
    }
  }
}


pub fn sync() {
  no::log("waiting for sync...");
  world().barrier();
  no::log("...synced, resuming");
}

