// Entry point for the proof

use v_macro;
use verifier;

#[no_mangle]
pub extern "C" fn entrypt() {
    test_test1();
}

// #[cfg(kani)]
// #[kani::proof]
#[v_macro::select()]
fn test_test1() {
    // let mut x: i32 = sea::nd_i32();

    // let mut x: i32 = kani::any();
    
    let mut x: i32 = verifier::nd_i32();
    // kani::assume(x < i32::MAX);
    // x += 4;

    // assert!(x < 14);
}
