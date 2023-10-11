#![cfg_attr(not(feature = "kani"), no_std)]

#[cfg(not(feature = "kani"))]
pub use sea;

use cfg_if::cfg_if;

#[inline(always)]
pub fn nd_i32() -> i32 {
    cfg_if!{
        if #[cfg(feature = "kani")] {
            kani::any()
        } else {
            sea::nd_i32()
        }
    }
}

// #[macro_export]
// macro_rules! nd_i32 {
//     () => {
//         cfg_if!{
//             if #[cfg(feature = "kani")] {
//                 kani::any()
//             } else {
//                 sea::nd_i32()
//             }
//         }
//     }
// }

#[inline(always)]
pub fn assume(cond : bool) {
    cfg_if!{
        if #[cfg(feature = "kani")] {
            kani::assume(cond)
        } else {
            sea::assume(cond);
        }
    }
}

#[inline(always)]
pub fn assert(cond : bool) {
    cfg_if!{
        if #[cfg(feature = "kani")] {
            assert!(cond)
        } else {
            sea::sassert!(cond)
        }
    }
}


// #[cfg(feature = "kani")]
// #[macro_export]
// macro_rules! nd_i32 {
//     () => {
//         kani::any()
//     }
// }

// #[cfg(not(feature = "kani"))]
// #[macro_export]
// macro_rules! nd_i32 {
//     () => {
//         sea::nd_i32()
//     }
// }

