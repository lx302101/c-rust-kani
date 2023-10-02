// Entry point for the proof
fn main() {
    test_test1();
}

#[kani::proof]
#[cfg(kani)]
fn test_test1() {
    // let mut x: i32 = sea::nd_i32();
    let mut x: i32 = kani::any();
    kani::assume(x < i32::MAX);
    x += 4;

    assert!(x < 14);
}
