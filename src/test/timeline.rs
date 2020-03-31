

#[cfg(test)]
mod test {

  use crate::timeline::{Timeline, isnever, NEVER, DISTANT_PAST, FAR_FUTURE};

  #[test]
  fn timeline_statics() {
    assert_eq!(DISTANT_PAST, DISTANT_PAST);
    assert_eq!(FAR_FUTURE, FAR_FUTURE);
    assert!(DISTANT_PAST < 0.0);
    assert!(0.0 < FAR_FUTURE);
    assert!(!(DISTANT_PAST < NEVER));
    assert!(!(DISTANT_PAST >= NEVER));
    assert!(!(FAR_FUTURE <= NEVER));
    assert!(!(FAR_FUTURE > NEVER));

    let x = -1e10;
    assert!(DISTANT_PAST < x);
    assert!(FAR_FUTURE > x);
    let x = 1e10;
    assert!(DISTANT_PAST < x);
    assert!(FAR_FUTURE > x);

    // dreams never end
    assert_ne!(NEVER, x);
    assert_ne!(NEVER, NEVER);
    assert!(!(NEVER < x));
    assert!(!(NEVER == x));
    assert!(!(NEVER >= x));
    // no nay never
    assert!(!isnever(x)); 
    // no nay never no more
    assert!(isnever(NEVER))  
  }

  #[test]
  fn timeline() {
    let mut timeline = Timeline::new(2020.0, 2050.0, vec![10,20,30]);

    assert_eq!(timeline.index(), 0);
    assert_eq!(timeline.dt(), 1.0);
    assert_eq!(timeline.at_checkpoint(), false);
    assert_eq!(timeline.at_end(), false);
    assert_eq!(timeline.nsteps(), 30);

    // test Iterator impl
    let r = timeline.next().unwrap();
    assert_eq!(r.0, 1);
    assert_eq!(r.1, 2021.0); 

    assert_eq!(timeline.index(), 1);
    assert_eq!(timeline.time(), 2021.0);
    assert_eq!(timeline.at_checkpoint(), false);
    assert_eq!(timeline.at_end(), false);

    let r = timeline.nth(8).unwrap();
    assert_eq!(r.0, 10);
    assert_eq!(r.1, 2030.0);

    assert_eq!(timeline.index(), 10);
    assert_eq!(timeline.at_checkpoint(), true);
    assert_eq!(timeline.at_end(), false);

    let r = timeline.nth(19).unwrap();
    assert_eq!(r.0, 30);
    assert_eq!(r.1, 2050.0);

    assert_eq!(timeline.index(), 30);
    assert_eq!(timeline.at_checkpoint(), true);
    assert_eq!(timeline.at_end(), true);

    assert_eq!(timeline.next(), None);

    timeline.reset();
    
    let mut step = 1;
    let mut year = 2021.0;
    for (i, t) in timeline { 
      //no::log(&format!("{} {}", i, t));
      assert_eq!(step, i);
      assert_eq!(year, t);
      step += 1;
      year += 1.0;
    }
    //assert!(timeline.at_checkpoint(), "should be at checkpoint");

    // null timeline
    let mut notimeline = Timeline::null();

    assert_eq!(notimeline.index(), 0);
    assert_eq!(notimeline.dt(), 0.0);
    assert_eq!(notimeline.at_checkpoint(), false);
    assert_eq!(notimeline.at_end(), false);
    assert_eq!(notimeline.nsteps(), 1);

    let r = notimeline.next().unwrap();
    assert_eq!(r.0, 1);
    assert_eq!(r.1, 0.0); 
    assert_eq!(notimeline.index(), 1);
    assert_eq!(notimeline.at_checkpoint(), true);
    assert_eq!(notimeline.at_end(), true);

    // TODO test Timeline in sync in python
  }
}