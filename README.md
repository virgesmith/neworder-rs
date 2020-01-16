# neworder-rs

A rust implementation of my [neworder](https://github.com/virgesmith/neworder) embedded python microsimulation package, which was implemented in C++.

## Notes

The `mpi` module has external dependencies and initially failed to install, this fixed it for me (on ubuntu 19.04): 

```bash
$ sudo apt install autoconf autogen libtool texinfo
```
