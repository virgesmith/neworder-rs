pub struct Timeline {
  checkpoints: (f64),
  steps: usize,
}

impl Timeline {
  fn new(checkpoints: (f64), steps: usize) -> Timeline {
    Timeline {
      checkpoints: checkpoints,
      steps: steps,
    }
  }

  // unequal to any other value 
  const NEVER: f64 = std::f64::NAN;
  // less than any other value
  const DISTANT_PAST: f64 = -std::f64::INFINITY;
  // greater than any other value
  const FAR_FUTURE: f64 = std::f64::INFINITY;

}


#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn statics() {
    assert_ne!(Timeline::NEVER, 0.0);
    assert_ne!(Timeline::NEVER, Timeline::NEVER);
    assert_eq!(Timeline::DISTANT_PAST, Timeline::DISTANT_PAST);
    assert_eq!(Timeline::FAR_FUTURE, Timeline::FAR_FUTURE);
    assert!(Timeline::DISTANT_PAST < 0.0);
    assert!(0.0 < Timeline::FAR_FUTURE);
    assert!(!(Timeline::DISTANT_PAST < Timeline::NEVER));
    assert!(!(Timeline::DISTANT_PAST >= Timeline::NEVER));
    assert!(!(Timeline::FAR_FUTURE <= Timeline::NEVER));
    assert!(!(Timeline::FAR_FUTURE > Timeline::NEVER));
  }



}