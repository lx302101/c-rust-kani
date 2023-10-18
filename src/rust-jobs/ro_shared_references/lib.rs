#![cfg_attr(not(feature = "kani"), no_std)]

use verifier;
// use std::mem;

// example taken from page 14 of 
// https://plv.mpi-sws.org/rustbelt/stacked-borrows/
// need to figure out how to port std

fn example2 (x: &i32 , f: impl FnOnce (& i32 )) -> i32 {
    let _val = *x / 3;
    f(x);
    return *x / 3; // We want to optimize this to return val .
}
    

#[no_mangle]
#[cfg_attr(feature = "kani", kani::proof)]
pub extern "C" fn entrypt() {

    // let mut local = 6;
    // let x = & local;
    // let result = example2 (x, | inner_x | {
    //     let raw_pointer : * mut i32 = unsafe { mem :: transmute ( inner_x ) };
    //     unsafe { * raw_pointer = 15; }
    //     });
    // println !(" {} ", result ); // Prints "5" ( aka 15/3).

        
}
