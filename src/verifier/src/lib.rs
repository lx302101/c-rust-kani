#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "kani"), feature = "std"))]
#[cfg(feature = "std")]
pub use sea_std as sea;

#[cfg(all(not(feature = "kani"), not(feature = "std")))]
pub use sea_panic_error as sea;

#[macro_export]
macro_rules! any {
    () => {{
        use cfg_if::cfg_if;
        
        cfg_if!{
            if #[cfg(feature = "kani")] {
                kani::any()
            } else {
                sea::any()
            }
        }
    }};
}

#[macro_export]
macro_rules! assume {
    ($cond: expr) => {{
        use cfg_if::cfg_if;
        
        cfg_if!{
            if #[cfg(feature = "kani")] {
                kani::assume($cond)
            } else {
                sea::assume($cond);
            }
        }
    }};
}

#[macro_export]
macro_rules! vassert {
    ($cond: expr) => {{
        use cfg_if::cfg_if;
        
        cfg_if!{
            if #[cfg(feature = "kani")] {
                assert!($cond)
            } else {
                sea::sassert!($cond)
            }
        }
    }};
}
