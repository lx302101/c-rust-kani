# Kani C-Rust
Repository to compare kani and seahorn on similar rust code.

## How to run Kani Proofs
Not sure how to run individual tests in kani just yet.

### To run kani tests:
To run all the tests - run in root folder:
```
export RUSTFLAGS='--cfg feature="kani"'
cargo kani
``` 
If seahorn is built in the build folder, source files can also be found in the build folder. So, the same commands will work in the build folder.

To find exceptions: 

### To run seahorn tests:
```
export RUSTFLAGS='--cfg feature=""'
rm -Rf build && mkdir build && cd build && cmake -DCMAKE_C_COMPILER=clang-14 -DCMAKE_CXX_COMPILER=clang++-14 -DSEAHORN_ROOT=$HOME/seahorn/build-dbg/run/ ../ -GNinja && cmake --build .
// in build folder
./verify src/rust-jobs/add/
``` 

`RUSTFLAGS` environment variable need to be set so that `kani` is not a feature before every build.

## How to make Kani Proofs
Create a folder under src/rust-jobs. In this folder, there will be 4 files:

`<test>.c` : The entry point for seahorn. It calls `entrypt()` from the rust library.
`lib.rs` : rust static library where the proof harnesses go.
`CMakeLists.txt` : To let seahorn build the library.
`Cargo.toml` : solve rust dependencies.

Important changes from c-rust to kani-c-rust:
In `Cargo.toml`:
- dependencies must include verifier and cfg-if
```
[dependencies]
verifier = { path = "../../verifier/src"}
cfg-if = "0.1.10"
```

In `lib.rs` :
- instead of `#![no_std]`, use `#![cfg_attr(not(feature = "kani"), no_std)]`
- must include `use verifier`
- must include `#[cfg_attr(feature = "kani", kani::proof)]` on top of test that should be run by seahorn/kani.
- use `verifier::any!()` , `verifier::assume!(cond)`, and `verifier::vassert!(cond)` instead of respective sea versions.

There is a script (not fully tested) to add jobs, similar to c-rust.
