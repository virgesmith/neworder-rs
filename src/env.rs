use std::sync::Mutex;
//use std::sync::atomic::AtomicBool; //clippy says to use this instead of Mutex


lazy_static! {

  // runtime flags
  static ref VERBOSE: Mutex<bool> = Mutex::new(false);
  static ref CHECKED: Mutex<bool> = Mutex::new(true);
  static ref HALT: Mutex<bool> = Mutex::new(false);

  // below initially set to invalid values (so that if module is not properly initialised its immediately apparent)

  // mpi env
  static ref RANK: Mutex<i32> = Mutex::new(-1);
  static ref SIZE: Mutex<i32> = Mutex::new(-1);

  static ref UNIQUE_INDEX: Mutex<i64> = Mutex::new(-1);

}

pub fn version() -> &'static str {
  // TODO how to get version from VERSION, not Cargo.toml
  env!("CARGO_PKG_VERSION")
}

pub fn verbose(v: bool) {
  *VERBOSE.lock().unwrap() = v;
}

pub fn checked(c: bool) {
  *CHECKED.lock().unwrap() = c;
}

// halt() exposed to python calls this with h=true
pub fn halt(h: bool) {
  *HALT.lock().unwrap() = h;
}

pub fn rank() -> i32 {
  *RANK.lock().unwrap()
}

pub fn size() -> i32 {
  *SIZE.lock().unwrap()
}

pub fn unique_index(n: usize) -> Vec<i64> {
  let n = n as i64;
  let start = *UNIQUE_INDEX.lock().unwrap();
  let step = size() as i64;
  // update the global
  *UNIQUE_INDEX.lock().unwrap() = start + n * step;
  (0..n).map(|i| start + i * step).collect()
}

// these should only be called on module init NB also (re)sets unique index
pub fn set_rank(r: i32) {
  *RANK.lock().unwrap() = r;
  *UNIQUE_INDEX.lock().unwrap() = r as i64;
}

pub fn set_size(s: i32) {
  *SIZE.lock().unwrap() = s;
}

