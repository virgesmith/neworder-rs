

// use num::traits::Zero;

// use std::error::Error;

// use crate::neworder as no;

pub fn version() -> &'static str {
  "?"
}

pub fn verbose(v: bool) {
  // TODO set global
}

pub fn checked(v: bool) {
  // TODO set global
}

pub fn rank() -> i32 {
  0 //MPI_ENV.world.rank()
}

pub fn size() -> i32 {
  1 //MPI_ENV.world.size()
}



