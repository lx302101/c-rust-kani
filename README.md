# Kani C-Rust
Repository to compare kani and seahorn on similar rust code.

## How to run Kani Proofs
The current build system is still in the early stages. There are a number of [pain points](#build-system-pain-points) which need to be addressed before it fully matures. For now, this is a brief description of how to run the tests.

### To run all tests:
To run all the tests, run `RUSTFLAGS='--cfg feature="kani"' cargo kani` in any directory.
- This will generate a `target/` file in the root directory which contains kani debug logs.
To run individual tests: go to source directory (ex. `src/rust-jobs/add/`) and run kani lib.rs

To find exceptions: 


## How to make Kani Proofs




## Build System Pain Points
Kani verifies proofs through the compilation process rather than creating an executable that can be run. This deviates from how the build system is set up in c-rust. This also means that it is not necessary to link with C codes Current investigating ways to run only one kani proof in the whole compilation unit, and to find ways to integrate with seahorn.

Another pain point is that Kani requires the attribute `#[kani::proof]` on top of the functions. When building normaly (for instance, `cargo build`), the compilation will fail due to dependencies. Although `#[cfg(kani)]` can be used, this will hide the function when compiling normally, and so `cargo build` will not include that function. This causes issues if we want to reuse the function for both kani/seahorn runs.

