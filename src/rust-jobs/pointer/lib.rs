// Entry point for the proof
pub extern "C" fn entrypt() {
    test();
}

#[cfg(kani)]
#[kani::proof]
fn test() {
    // let mut v: i32  = kani::any();
    // kani::assume(v > 0);
    let mut v = 10;
    let original: i32 = v;

    let n: *mut i32 = &mut v;

    unsafe {
        *n = *n + 1;
        *n = *n + 1;
    }

    assert!(v == original + 2);
}
