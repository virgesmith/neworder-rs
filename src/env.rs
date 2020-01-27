
use mpi;
use mpi::topology::Communicator;
use mpi::collective::CommunicatorCollectives;
use mpi::collective::Root;

use std::error::Error;
use num::traits::Zero;

// enum DistPolicy {
//   ChainForward,
//   ChainForwardWrapped
// }

use crate::neworder as no;

struct MPIEnv {
  _universe: mpi::environment::Universe,
  world: mpi::topology::SystemCommunicator,
  seed: i64,
  indep: bool // TODO can this be removed
}

const BASE_SEED: i64 = 19937;

// generate a seed for each process
fn genseed(rank: i32, size: i32, indep: bool) -> i64 {
  if indep {
    BASE_SEED * (size as i64) + (rank as i64)
  } else {
    BASE_SEED
  }
}

impl MPIEnv {
  fn new(indep: bool) -> MPIEnv {
    let u = mpi::initialize().unwrap();
    let w = u.world();
    let r = w.rank();
    let s = w.size();
    MPIEnv{ _universe: u, world: w, seed: genseed(r, s, indep), indep: indep }
  }
}

// TODO this data may need to be stored in the python module
lazy_static! {
  static ref MPI_ENV: MPIEnv = { MPIEnv::new(true) };

  //static ref BASE_SEED: i64 = { 19937 };
}

// static mut g_init: bool = false;

// static /*mut*/ INDEP: bool = false;


pub fn indep() -> bool {
  MPI_ENV.indep
}

pub fn rank() -> i32 {
  MPI_ENV.world.rank()
}

pub fn size() -> i32 {
  MPI_ENV.world.size()
}

pub fn seed() -> i64 {
  MPI_ENV.seed
}

pub fn world() -> &'static mpi::topology::SystemCommunicator {
  &MPI_ENV.world
}

// template<typename T>
// T& sendrecv(T& data, no::mpi::DistPolicy dist_policy = no::mpi::DistPolicy::CHAIN_FWD_WRAPPED)
pub fn rotate<T: mpi::datatype::Equivalence>(data: T) -> Result<T, Box<dyn Error>> {
  
  let rank = rank();
  let size = size();

  let (prev, next) = (world().process_at_rank((rank + size - 1) % size), 
                      world().process_at_rank((rank + 1) % size));

  let (data, _status) = mpi::point_to_point::send_receive(&data, &next, &prev);
  // TODO match status check for error
  Ok(data)
}

pub fn broadcast_from<T: mpi::datatype::Equivalence>(from: i32, data: &mut T) -> Result<(), Box<dyn Error>> {
  let root_process = world().process_at_rank(from);  
  root_process.broadcast_into(data);
  Ok(())
}

pub fn scatter_from<T: Zero + mpi::datatype::Equivalence>(from: i32, data: &Vec<T>) -> T { 

  let src = world().process_at_rank(from);

  let mut x = T::zero();
  match rank() == from {
    true => src.scatter_into_root(&data[..], &mut x),
    false => src.scatter_into(&mut x)
  };
  x
}

// Returns an Option containing an array in rank() == to
pub fn gather_into<T: mpi::datatype::Equivalence>(into: i32, data: &T) -> Option<Vec<T>> { 

  let dst = world().process_at_rank(into);

  match rank() == into {
    true => {
      let mut a = Vec::with_capacity(size() as usize);
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
  no::log("waiting...");
  world().barrier();
  no::log("...resuming");
}

//pub fn gather

// {
// #ifdef NEWORDER_MPI
//   no::Environment& env = no::getenv();
//   int source, dest;
//   // CHAIN_FWD_WRAPPED
//   switch (dist_policy)
//   {
//   case CHAIN_FWD_WRAPPED:  
//     dest = (env.rank() + 1) % env.size();
//     source = (env.rank() - 1) % env.size();
//     break;
//   case CHAIN_FWD:
//     dest = env.rank() == env.size() - 1 ?  MPI_PROC_NULL: env.rank() + 1;
//     source = env.rank() == 0 ?  MPI_PROC_NULL: env.rank() - 1;
//     break;
//   default:
//     throw std::runtime_error("invalid sendrecv distribution policy");
//   }

//   MPI_Sendrecv_replace(&data, 1, mpi_type_trait<T>::type, dest, 0, source, 0, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
//   // TODO return a ref to data?
// #endif
//   return data;
// }

// struct NEWORDER_EXPORT Environment
// {
// public:

//   // Context
//   static const int CPP = 0;
//   static const int PY = 1;

//   ~Environment();

//   // Disable any copy/assignment
//   Environment(const Environment&) = delete;
//   Environment& operator=(const Environment&) = delete;
//   Environment(const Environment&&) = delete;
//   Environment& operator=(const Environment&&) = delete;

//   // Use this function to create the base environemt
//   static Environment& init(int rank, int size, bool indep = true);

//   // check for errors in the python env (use after catching py::error_already_set)
//   static std::string get_error() noexcept;

//   // returns the python version
//   static std::string python_version();

//   // MPI rank (0 if serial)
//   static int rank();

//   // MPI size (1 if serial)
//   static int size();

//   // independent streams (per rank)? 
//   static bool indep();

//   // returns "py/no rank/size"
//   std::string context(int ctx = CPP) const;

//   // reset the RNG stream sequence to the original seed 
//   static void reset();

//   // Accress the NRG stream (one per env)
//   std::mt19937& prng();

//   // returns the env as a python object 
//   //operator py::object&() { return m_self; } doesnt implicitly cast
//   py::module& operator()() { return *m_self; }

//   no::Timeline& timeline() { return m_timeline; }

// private:

//   // TODO reinstate when this is no longer static lifetime
//   //py::scoped_interpreter m_guard; // start the interpreter and keep it alive

//   // compute the RNG seed
//   int64_t compute_seed() const;

//   // flag to check whether init has been called
//   bool m_init;

//   // Singletons only
//   Environment();
//   friend Environment& Global::instance<Environment>();

//   // RNG sequence index
//   //size_t m_seqno; use python version for now
//   //np::array m_sequence;

//   // MPI rank/size
//   int m_rank;
//   int m_size;
//   // set to false to make all processes use the same seed
//   bool m_indep;

//   // seed not directly visible to python
//   int64_t m_seed;

//   // TODO work out why this segfaults if the dtor is called (even on exit)
//   py::module* m_self;
//   // thread/process-safe seeding strategy deferred until config loaded
//   std::mt19937 m_prng;

//   no::Timeline m_timeline;
// };

// // syntactic sugar
// Environment& getenv();

// }