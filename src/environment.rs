
//use std::sync::Mutex;
use mpi::topology::Communicator;

// lazy_static! {
//   static ref ENV: Mutex<Environment> = Mutex::new(Environment::new());
// }

// static mut g_init: bool = false;
// static mut g_rank: u32 = 0;
// static mut g_size: u32 = 1;

static /*mut*/ g_indep: bool = false;
static /*mut*/ g_seed: i64 = 19937;

// struct Environment {

//   // flag to check whether init has been called
//   init : bool,

//   // RNG sequence index
//   //size_t m_seqno; use python version for now
//   //np::array m_sequence;

//   // MPI rank/size
//   rank: u32,
//   size: u32,
//   // set to false to make all processes use the same seed
//   indep: bool,

//   // seed not directly visible to python
//   seed: i64,

//   // TODO work out why this segfaults if the dtor is called (even on exit)
//   //m_self: &PyModule
//   // thread/process-safe seeding strategy deferred until config loaded
//   // std::mt19937 m_prng;

//   // no::Timeline m_timeline;
// }

// impl Environment {
//   fn new() -> Environment {
//     Environment{ init: false, rank: 0, size: 1, indep: false, seed: 19937 }
//   }
// }

pub fn seed() -> &'static i64 {
  &g_seed
}

pub fn indep() -> &'static bool {
  &g_indep
}

pub fn mpi() -> (i32, i32) {
  match mpi::initialize() {
    Some(u) => (u.world().rank(), u.world().size()), 
    None => (-1, -1)
  }
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