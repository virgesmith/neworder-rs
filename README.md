[![Travis Build Status](https://travis-ci.org/virgesmith/neworder-rs.svg?branch=master)](https://travis-ci.org/virgesmith/neworder-rs)

# neworder-rs

An attempt at a rust implementation of my [neworder](https://github.com/virgesmith/neworder) embedded python microsimulation package, which was implemented in C++.

## Build & Test

As per usual:
```
$ cargo build --release
```
The binary will run happily as a single process but the tests require an MPI environment to run, otherwise you'll get
```
$ cargo test --release
...
---- test::test::test_mpi stdout ----
thread 'test::test::test_mpi' panicked at 'mpi is not enabled', src/test/mod.rs:143:5
```
so do this:
```
mpirun -n N cargo test -- --nocapture
```
where `N` is >=2 and the `-- --nocapture` shows the output you'd get from the tests in the C++ implementation.

## Notes

### Dependencies
Some of these require nightly rust so assuming liable to API changes.

### pyo3 
Could be improved with some simplification and more documentation/examples to make it easier to use, but it does work. Overlapping but functionally different types (e.g. `PyObject` and `PyAny`) and difficult type conversions caused some headaches.

### mpi
The `mpi` module has external dependencies and initially failed to install, this fixed it for me (on ubuntu 19.04): 

```bash
$ sudo apt install autoconf autogen libtool texinfo
```
So far I've found no problems with it, but the package hasn't been updated in a couple of years which is of concern.

### numpy 
The `numpy` (rust) package fails to build using nightly:
```
error: array lengths can't depend on generic parameters
   --> /home/az/.cargo/registry/src/github.com-1ecc6299db9ec823/matrixmultiply-0.2.3/src/dgemm_kernel.rs:786:39
    |
786 |     let mut ab: [[T; NR]; MR] = [[0.; NR]; MR];
```
Will revisit this when it becomes a blocker...
