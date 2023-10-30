#![cfg_attr(not(feature = "std"), no_std)]

use verifier;


#[no_mangle]
#[cfg_attr(feature = "kani", kani::proof)]
pub extern "C" fn entrypt() {

    let mut v: i32  = verifier::any!();
    verifier::assume!(v > 0);
    let original: i32 = v;

    let n: *mut i32 = &mut v;

    unsafe {
        *n = *n + 1;
        *n = *n + 1;
    }

    verifier::vassert!(v == original + 2);
}
