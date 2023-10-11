use crate::bindings::*;

#[no_mangle]
pub fn verifier_error() { unsafe { __VERIFIER_error(); } }

#[no_mangle]
pub fn assume(v: bool) { unsafe { __VERIFIER_assume(v.into()); } }

#[no_mangle]
pub fn nd_i8() -> i8 { unsafe { sea_nd_i8() } }
#[no_mangle]
pub fn nd_u8() -> u8 { unsafe { sea_nd_u8() } }
#[no_mangle]
pub fn nd_i16() -> i16 { unsafe { sea_nd_i16() } }
#[no_mangle]
pub fn nd_u16() -> u16 { unsafe { sea_nd_u16() } }
#[no_mangle]
pub fn nd_u32() -> u32 { unsafe { sea_nd_u32() } }
#[no_mangle]
pub fn nd_i64() -> i64 { unsafe { sea_nd_i64() } }
#[no_mangle]
pub fn nd_u64() -> u64 { unsafe { sea_nd_u64() } }
#[no_mangle]
pub fn nd_usize() -> usize { unsafe { sea_nd_usize() } }
#[no_mangle]
pub fn nd_isize() -> isize { unsafe { sea_nd_isize() } }
#[no_mangle]
pub fn nd_uintptr() -> usize { unsafe { sea_nd_uintptr() } }
#[no_mangle]
pub fn nd_intptr() -> isize { unsafe { sea_nd_intptr() } }

#[no_mangle]
pub fn nd_bool() -> bool { unsafe { sea_nd_bool() } }

#[macro_export]
macro_rules! sea_printf {
    ($message:expr $(, $args:expr)*) => {{
        use crate::sea::bindings::sea_printf;
        use core::ffi::c_char;
        unsafe { sea_printf($message.as_ptr() as *const c_char, $($args),*); }
    }}
}

#[macro_export]
macro_rules! sassert {
    ($cond:expr) => {{
        if !$cond {
            sea::verifier_error();
        }
    }};
}

pub trait Arbitrary
where
    Self: Sized,
{
    fn any() -> Self;
}

macro_rules! generate_impl {
    ( $type: ty, $sea_func: expr ) => {
        impl Arbitrary for $type {
            #[inline(always)]
            fn any() -> $type {
                unsafe { $sea_func() }
            }
        }
    };
}

generate_impl!(i32, sea_nd_i32);


#[inline(always)]
pub fn any<T: Arbitrary>() -> T {
    T::any()
}
