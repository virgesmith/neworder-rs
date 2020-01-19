
use mpi::topology::Communicator;

struct MPIInfo {
  rank: i32,
  size: i32
}

impl MPIInfo {
  fn new() -> MPIInfo {
    let u = mpi::initialize().unwrap();
    let w = u.world();
    MPIInfo{ rank: w.rank(), size: w.size()}
  }
}

lazy_static! {
  static ref MPI: MPIInfo = { MPIInfo::new() };

  //static ref BASE_SEED: i64 = { 19937 };
}

// static mut g_init: bool = false;

static /*mut*/ INDEP: bool = false;
static /*mut*/ BASE_SEED: i64 = 19937;

pub fn seed() -> i64 {
  if INDEP {
    BASE_SEED * (size() as i64) + (rank() as i64)
  } else {
    BASE_SEED
  }
}

pub fn indep() -> &'static bool {
  &INDEP
}

pub fn rank() -> i32 {
  MPI.rank
}

pub fn size() -> i32 {
  MPI.size
}

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