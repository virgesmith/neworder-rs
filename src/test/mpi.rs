
#[cfg(test)]
mod test {

  //use super::*;
  use crate::neworder as no;
  use crate::env;
  
  //use mpi::point_to_point as p2p;
  use mpi::point_to_point::{Source, Destination};
  use mpi::topology::Rank;
  use mpi::traits::*;
  //use mpi::collective::CommunicatorCollectives;

  use numpy::{PyArray1};

  // TODO crashes... fix
  // //fn send0_recv1<T: PartialEq + mpi::datatype::Equivalence>(x: T) -> bool {
  // fn send0_recv1<T: PartialEq + mpi::datatype::Equivalence + std::fmt::Debug>(x: T) -> bool {

  //   if env::rank() == 0 {
  //     //p2p::send(x, 1);
  //     // NB sends ***TO** 1
  //     env::world().process_at_rank(1).send(&x);
  //     //no::log(&format!("sent {:?} to 1", x));
  //   }
  //   if env::rank() == 1 {
  //     // NB receives ***FROM** 0
  //     let (y, _): (T, _) = env::world().process_at_rank(0).receive();
  //     //no::log(&format!("got {:?} from 0", y));
  //     return y == x;
  //   }
  //   true
  // }

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
    if env::size() == 1 {
      no::log("WARNING: MPI tests not executed");
      return;
    }
    //assert!(env::size() > 1, "mpi is not enabled");

    // assert!(send0_recv1(false));
    // assert!(send0_recv1(19937));
    // assert!(send0_recv1(-1i64));
    // assert!(send0_recv1(71.25));

    // // how to send chars, strings and vectors 
    // assert!(send0_recv1('c' as u8)); // char Equivalence not implemented 

    let a = vec![0;12];
    if env::rank() == 0 {
      //p2p::send(x, 1);
      env::world().process_at_rank(1).send(&a[..]);
      no::log(&format!("sent {} elements to 1", a.len()));
    }
    if env::rank() == 1 {
      let (b, _status) = env::world().process_at_rank(0).receive_vec::<i32>();
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
      let (b, _status) = env::world().process_at_rank(0).receive_vec::<u8>();
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

    // should be e.g. [3, 13, 23] on 0
    match env::gather_into(0, &x) {
      Some(a) => {
        //no::log(&format!("gather_into {} = {:?}", env::rank(), a));
        assert_eq!(env::rank(), 0);
        assert_eq!(a.len(), env::size() as usize);
        assert!(a.iter().enumerate().all(|(r, &x)|  x == 10 * (r as Rank) + env::size()));
      },
      None => {
        assert!(env::rank() != 0);
      } 
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