use cfg_if::cfg_if;

#[inline(always)]
pub fn nd_i32() -> i32 {
    cfg_if!{
        if #[cfg(KANI)] {
            kani::any()
        } else {
            // replace with seahorn
            4
        }
    }
}

// #[macro_export]
// macro_rules! nd_i32 {
//     () => {
//         cfg_if!{
//             if #[cfg(KANI)] {
//                 kani::any()
//             } else {
//                 4
//             }
//         }
//     };
// }

