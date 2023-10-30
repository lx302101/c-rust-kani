#![cfg_attr(not(feature = "std"), no_std)]

// example taken from page 2 of 
// https://plv.mpi-sws.org/rustbelt/stacked-borrows/

use verifier;

#[no_mangle]
fn example1 (x: & mut i32 , y: & mut i32, x_value: i32, y_value: i32 ) -> i32 {
    *x = x_value;
    *y = y_value;
    return *x;
}

#[no_mangle]
#[cfg_attr(feature = "kani", kani::proof)]
pub extern "C" fn entrypt() {
    let x_value:i32 = verifier::any!();
    let y_value:i32 = verifier::any!();

    // verifier::assume!(x_value != y_value);

    let b:bool = verifier::any!();
    verifier::assume!(b == true);

    let mut local = verifier::any!();

    let raw_pointer = & mut local as * mut i32;
    let result:i32 = unsafe { example1 (& mut * raw_pointer , & mut * raw_pointer , x_value, y_value) };

    // this will pass seahorn but not pass kani
    verifier::vassert!((result == x_value) && b);

    // this will pass kani but not pass seahorn
    verifier::vassert!((result == y_value) && b);
}
