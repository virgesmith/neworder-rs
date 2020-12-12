use std::sync::Mutex;

lazy_static! {

  // runtime flags
  static ref VERBOSE: Mutex<bool> = Mutex::new(false);
  static ref CHECKED: Mutex<bool> = Mutex::new(true);
  static ref HALT: Mutex<bool> = Mutex::new(false);

  // mpi env
  static ref RANK: Mutex<i32> = Mutex::new(0);
  static ref SIZE: Mutex<i32> = Mutex::new(1);

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

// these should only be called on module init
pub fn set_rank(r: i32) {
  *RANK.lock().unwrap() = r;
}

pub fn set_size(s: i32) {
  *SIZE.lock().unwrap() = s;
}