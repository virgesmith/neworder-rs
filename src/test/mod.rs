

#[cfg(test)]
mod test {

  //use super::*;
  use crate::timeline::Timeline;
  use crate::neworder as no;
  use crate::callback::Callback;
  use crate::env;
  
  use pyo3::prelude::*;
  use pyo3::{Python};//, PyResult};
  use pyo3::types::*; 

  use mpi::point_to_point as p2p;
  use mpi::point_to_point::{Source, Destination};
  use mpi::topology::Rank;
  use mpi::traits::*;
  use mpi::collective::CommunicatorCollectives;

  use numpy::{PyArray, PyArray1};

  #[test]
  fn timeline_statics() {
    assert_eq!(Timeline::DISTANT_PAST, Timeline::DISTANT_PAST);
    assert_eq!(Timeline::FAR_FUTURE, Timeline::FAR_FUTURE);
    assert!(Timeline::DISTANT_PAST < 0.0);
    assert!(0.0 < Timeline::FAR_FUTURE);
    assert!(!(Timeline::DISTANT_PAST < Timeline::NEVER));
    assert!(!(Timeline::DISTANT_PAST >= Timeline::NEVER));
    assert!(!(Timeline::FAR_FUTURE <= Timeline::NEVER));
    assert!(!(Timeline::FAR_FUTURE > Timeline::NEVER));

    let x = -1e10;
    assert!(Timeline::DISTANT_PAST < x);
    assert!(Timeline::FAR_FUTURE > x);
    let x = 1e10;
    assert!(Timeline::DISTANT_PAST < x);
    assert!(Timeline::FAR_FUTURE > x);

    // dreams never end
    assert_ne!(Timeline::NEVER, x);
    assert_ne!(Timeline::NEVER, Timeline::NEVER);
    assert!(!(Timeline::NEVER < x));
    assert!(!(Timeline::NEVER == x));
    assert!(!(Timeline::NEVER >= x));
    // no nay never
    assert!(!Timeline::isnever(x)); 
    // no nay never no more
    assert!(Timeline::isnever(Timeline::NEVER))  
  }

  #[test]
  fn timeline() {
    let mut timeline = Timeline::new(2020.0, 2050.0, vec![10,20,30]);

    assert_eq!(timeline.idx(), 0);
    assert_eq!(timeline.dt(), 1.0);
    assert_eq!(timeline.at_checkpoint(), false);
    assert_eq!(timeline.at_end(), false);

    let r = timeline.next().unwrap();
    assert_eq!(r.0, 1);
    assert_eq!(r.1, 2021.0); 

    assert_eq!(timeline.idx(), 1);
    assert_eq!(timeline.at_checkpoint(), false);
    assert_eq!(timeline.at_end(), false);

    let r = timeline.nth(8).unwrap();
    assert_eq!(r.0, 10);
    assert_eq!(r.1, 2030.0);

    assert_eq!(timeline.idx(), 10);
    assert_eq!(timeline.at_checkpoint(), true);
    assert_eq!(timeline.at_end(), false);

    let r = timeline.nth(19).unwrap();
    assert_eq!(r.0, 30);
    assert_eq!(r.1, 2050.0);

    assert_eq!(timeline.idx(), 30);
    assert_eq!(timeline.at_checkpoint(), true);
    assert_eq!(timeline.at_end(), true);

    assert_eq!(timeline.next(), None);

    timeline.reset();
    let mut step = 1;
    let mut year = 2021.0;
    for (i, t) in timeline { //.collect::<Vec<(u32, f64)>>() {
      //no::log(&format!("{} {}", i, t));
      assert_eq!(step, i);
      assert_eq!(year, t);
      step += 1;
      year += 1.0;
    }
  }

  #[test]
  fn test_no() {
    // test logging - use {, true} to make it look like function returning bool. If a problem, there will be an exception or worse
    assert!({no::log("neworder module test"); true}, "log message");
    assert!({no::log(&format!("test logging types: {} {} {} {} {:?} {}", false, 0, 0.0, "", vec![0; 10], true)); true}, "log data");

    // // test formatting
    // CHECK(format::decimal(3.14, 6, 6) == "     3.140000");
    // // ignores the 1 LHS padding as there are 6 digits
    // CHECK(format::decimal(1000000.0 / 7, 1, 6) == "142857.142857");
    // CHECK(format::pad(3, 4) == "   3");
    // CHECK(format::pad(3, 5, '0') == "00003");
    // // ignores 3 as number requires 4 chars
    // CHECK(format::pad(5000, 3, '0') == "5000");
    // CHECK(format::hex<int32_t>(24233) == "0x00005ea9");
    // CHECK(format::hex<size_t>(133, false) == "0000000000000085");
    // CHECK(format::boolean(false) == "false");
    
    // /*no::Environment& env =*/ no::getenv();
    let gil = Python::acquire_gil();
    let py = gil.python();
  
    let neworder = no::init_embedded(py).unwrap();
    let locals = [("neworder", neworder)].into_py_dict(py);

    // Check required (but defaulted) attrs visible from both rust and python
    let attrs = ["rank", "size"];

    //Callback::exec("neworder.log(dir(neworder))".to_string(), None, Some(locals)).run(py).unwrap();
    // This isn't quite the same as below 
    for a in &attrs {
      assert!(match neworder.get(a) {
        Ok(_) => true,
        Err(_) => false
      });
      assert!(Callback::eval(format!("'{}' in dir(neworder)", a), None, Some(locals)).run(py).unwrap().is_true(py).unwrap(), "attr seen by python");
    }
    // for (size_t i = 0; i < sizeof(attrs)/sizeof(attrs[0]); ++i)
    // {
    //   CHECK(pycpp::has_attr(module, attrs[i]));
    //   CHECK(no::Callback::eval("'%%' in locals()"_s % attrs[i])().cast<bool>());
    // }

    // check string conversion
    assert_eq!("<built-in function never>", format!("{}", neworder.get("never").unwrap()));
    assert_eq!(format!("{}", neworder.call0("never").unwrap()), "nan");

    // Check diagnostics consistent
    assert_eq!(
      format!("{}", Callback::eval("neworder.name()".to_string(), None, Some(locals)).run(py).unwrap().as_ref(py).str().unwrap()),
      no::name());
    // CHECK(no::Callback::eval("version() == '%%'"_s % no::module_version())().cast<bool>());
    // CHECK(no::Callback::eval("python() == '%%'"_s % no::python_version()/*.c_str()*/)().cast<bool>());

    let _pyarray = PyArray1::from_vec(gil.python(), (0..10).map(|i| 0.0 / (i as f64)).collect());
    //no::log(&format!("{:?}", pyarray));
    //let res = Timeline::array_isnever(gil.python(), pyarray);
    //no::log(&format!("{:?}", res.as_ref(py)));
    //assert!(res.as_ref(py)[0]);
  }

  //fn send0_recv1<T: PartialEq + mpi::datatype::Equivalence>(x: T) -> bool {
  fn send0_recv1<T: PartialEq + mpi::datatype::Equivalence + std::fmt::Debug>(x: T) -> bool {

    if env::rank() == 0 {
      //p2p::send(x, 1);
      // NB sends ***TO** 1
      env::world().process_at_rank(1).send(&x);
      //no::log(&format!("sent {:?} to 1", x));
    }
    if env::rank() == 1 {
      // NB receives ***FROM** 0
      let (y, _): (T, _) = env::world().process_at_rank(0).receive();
      //no::log(&format!("got {:?} from 0", y));
      return y == x;
    }
    true
  }

  // fn send0_recv1_vec<T: PartialEq + std::fmt::Debug>(x: [T]) -> bool {
  //   if env::rank() == 0 {
  //     //p2p::send(x, 1);
  //     env::world().process_at_rank(1).send(&x);
  //     no::log(&format!("sent {:?} to 1", x));
  //   }
  //   // if env::rank() == 1 {
  //   //   let (y, _): ([T], _) = env::world().process_at_rank(0).receive();
  //   //   //no::log(&format!("got {:?} from 0", y));
  //   //   return y.iter().zip(&x).filter(|(&a,&b)| a == b).count() == x.len();
  //   // }

  //   true
  // }
  // // template<typename T>
  // // bool send_recv(const T& x, no::Environment& env)
  // {
  //   if (env.rank() == 0)
  //   {
  //     no::mpi::send(x, 1);
  //   }
  //   if (env.rank() == 1)
  //   {
  //     T y;
  //     no::mpi::receive(y, 0);
  //     //no::log("MPI: 0 sent x=%% 1 recd y=%%"_s % x % y);
  //     if (y != x)
  //     return false;
  //   }
  //   return true;
  // }

  #[test]
  fn test_mpi()
  {
    assert!(env::size() > 1, "mpi is not enabled");

    assert!(send0_recv1(false));
    assert!(send0_recv1(19937));
    assert!(send0_recv1(-1i64));
    assert!(send0_recv1(71.25));

    // how to send chars, strings and vectors 
    assert!(send0_recv1('c' as u8)); // char Equivalence not implemented 

    let a = vec![0;12];
    if env::rank() == 0 {
      //p2p::send(x, 1);
      env::world().process_at_rank(1).send(&a[..]);
      no::log(&format!("sent {} elements to 1", a.len()));
    }
    if env::rank() == 1 {
      let (b, status) = env::world().process_at_rank(0).receive_vec::<i32>();
      no::log(&format!("recd {} elements from 0", b.len()));
    }

    // env::world().process_at_rank(1).send(&a[..]);
    let a = "dhgsjdfg";
    if env::rank() == 0 {
      //p2p::send(x, 1);
      env::world().process_at_rank(1).send(&a.as_bytes()[..]);
      no::log(&format!("sent {:?} to 1", a));
    }
    if env::rank() == 1 {
      let (b, status) = env::world().process_at_rank(0).receive_vec::<u8>();
      no::log(&format!("recd {:?} from 0", std::str::from_utf8(&b).unwrap()));
    }
    // env::world().process_at_rank(1).send(&a.as_bytes()[..]);
    // env::world().process_at_rank(1).send(&(a as u8));

    let mut x = 7122 + env::rank();
    // sends the value to next process and receives from previous (wrapped)
    x = env::rotate(x).unwrap();  
    assert_eq!(x, 7122 + (env::size() + env::rank() - 1) % env::size());

    //env::sendrecv("const char*").unwrap();
    // //CHECK(send_recv("const char*", env));
    //send0_recv1(String::from("String").as_bytes());
    // //CHECK(send_recv("std::string"_s, env));

    let mut i = match env::rank() {
      0 => 12345,
      _ => 0
    };
    env::broadcast_from(0, &mut i).unwrap();
    // should now be 12345 on all 
    assert_eq!(i, 12345);

    let x = 10 * env::rank() + env::size();

    let a = match env::gather_into(0, &x) {
      Some(a) => {
        assert_eq!(env::rank(), 0);
        assert!(a.iter().enumerate().all(|(r, &x)|  x == 10 * (r as i32) + env::size()));
      },
      None => () 
    };

    // scatter
    // a contains different values in each process
    let a = vec![1 + env::rank() * env::rank(); env::size() as usize];

    let x = env::scatter_from(1, &a);
    env::sync(); // required
    // x contains the value 1+1*1
    assert_eq!(x, 2);

    let i = 2_u64.pow(env::rank() as u32 + 1);
    let mut a = vec![0u64; env::size() as usize];
    env::world().all_gather_into(&i, &mut a[..]);
    assert!(a.iter().enumerate().all(|(a, &b)| b == 2u64.pow(a as u32 + 1)));
    //no::log(&format!("allgather: {:?}", a));

    // std::vector<double> agv(env.size(), -1.0);
    // // give agv one positive element
    // agv[env.rank()] = 10.0 * env.rank() + env.size();
    // agv = no::mpi::allgather(agv);
    // // agv now all positive
    // for (size_t i = 0; i < agv.size(); ++i)
    // {
    //   CHECK(agv[i] == 10.0 * i + env.size());
    //   //no::log("allgather element %%=%%"_s % i % agv[i]);
    // }
  }
}