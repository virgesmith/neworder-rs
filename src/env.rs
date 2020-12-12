
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, Ordering};

static VERBOSE: AtomicBool = AtomicBool::new(false);
static CHECKED: AtomicBool = AtomicBool::new(true);
static HALT: AtomicBool = AtomicBool::new(false);

static RANK: AtomicI32 = AtomicI32::new(-1);
static SIZE: AtomicI32 = AtomicI32::new(-1);

static UNIQUE_INDEX: AtomicI64 = AtomicI64::new(-1);

pub fn version() -> &'static str {
  // TODO how to get version from VERSION, not Cargo.toml
  env!("CARGO_PKG_VERSION")
}

pub fn verbose(v: bool) {
  VERBOSE.store(v, Ordering::Relaxed);
}

pub fn checked(c: bool) {
  CHECKED.store(c, Ordering::Relaxed);
}

// halt() exposed to python calls this with h=true
pub fn set_halt(h: bool) {
  HALT.store(h, Ordering::Relaxed);
}

// these should only be called on module init NB also (re)sets unique index
pub fn set_rank(r: i32) {
  RANK.store(r, Ordering::Relaxed);
  UNIQUE_INDEX.store(r as i64, Ordering::Relaxed);
}

pub fn set_size(s: i32) {
  SIZE.store(s, Ordering::Relaxed);
}


// getters

pub fn halted() -> bool {
  HALT.load(Ordering::Relaxed)
}

pub fn rank() -> i32 {
  RANK.load(Ordering::Relaxed)
}

pub fn size() -> i32 {
  SIZE.load(Ordering::Relaxed)
}

pub fn unique_index(n: usize) -> Vec<i64> {
  let n = n as i64;
  // acquired the global
  let start = UNIQUE_INDEX.load(Ordering::Acquire);
  let step = size() as i64;
  // update and release the global
  UNIQUE_INDEX.store(start + n * step, Ordering::Release);
  (0..n).map(|i| start + i * step).collect()
}


