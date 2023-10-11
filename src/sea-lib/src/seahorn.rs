use crate::bindings::*;

#[no_mangle]
pub fn verifier_error() { unsafe { __VERIFIER_error(); } }

#[no_mangle]
pub fn assume(v: bool) { unsafe { __VERIFIER_assume(v.into()); } }

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

generate_impl!(i8, sea_nd_i8);
generate_impl!(u8, sea_nd_u8);
generate_impl!(i16, sea_nd_i16);
generate_impl!(u16, sea_nd_u16);
generate_impl!(i32, sea_nd_i32);
generate_impl!(u32, sea_nd_u32);
generate_impl!(i64, sea_nd_i64);
generate_impl!(u64, sea_nd_u64);

generate_impl!(bool, sea_nd_bool);


#[no_mangle]
pub fn nd_usize() -> usize { unsafe { sea_nd_usize() } }
#[no_mangle]
pub fn nd_isize() -> isize { unsafe { sea_nd_isize() } }
#[no_mangle]
pub fn nd_uintptr() -> usize { unsafe { sea_nd_uintptr() } }
#[no_mangle]
pub fn nd_intptr() -> isize { unsafe { sea_nd_intptr() } }



#[inline(always)]
pub fn any<T: Arbitrary>() -> T {
    T::any()
}
