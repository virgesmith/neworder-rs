# neworder-rs

An attempt at a rust implementation of my [neworder](https://github.com/virgesmith/neworder) embedded python microsimulation package, which was implemented in C++.

## Notes

The `mpi` module has external dependencies and initially failed to install, this fixed it for me (on ubuntu 19.04): 

```bash
$ sudo apt install autoconf autogen libtool texinfo
```
The `numpy` (rust) package fails to build using nightly (which is needed for pyo3):

```
error: array lengths can't depend on generic parameters
   --> /home/az/.cargo/registry/src/github.com-1ecc6299db9ec823/matrixmultiply-0.2.3/src/dgemm_kernel.rs:786:39
    |
786 |     let mut ab: [[T; NR]; MR] = [[0.; NR]; MR];
```

