

pub fn version() -> &'static str {
  // TODO how to get version *at compile time*
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



