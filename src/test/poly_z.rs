use poly_z::*;
use std::ops::*;
//
// #[test]
// fn test_poly_z_add() {
//     let a = Poly::<u64> {
//         degree: 4,
//         coeff: vec![1, 2, 3, 4],
//     };
//     let b = a.clone();
//     let c = a + b;
//     assert_eq!(
//         c,
//         Poly::<u64> {
//             degree: 4,
//             coeff: vec![2, 4, 6, 8],
//         }
//     );
//
//     let a = Poly::<u64> {
//         degree: 4,
//         coeff: vec![1, 2, 3, 4],
//     };
//     let b = Poly::<u64> {
//         degree: 3,
//         coeff: vec![1, 2, 3],
//     };
//     let c = a + b;
//     assert_eq!(
//         c,
//         Poly::<u64> {
//             degree: 4,
//             coeff: vec![2, 4, 6, 4],
//         }
//     );
// }

fn test_poly_z_add<T: Add>() {
    let a = Poly::<T> {
        degree: 4,
        coeff: vec![1, 2, 3, 4],
    };
    let b = Poly::<T> {
        degree: 3,
        coeff: vec![1, 2, 3],
    };
    let c = a + b;
    assert_eq!(
        c,
        Poly::<u64> {
            degree: 4,
            coeff: vec![2, 4, 6, 4],
        }
    );
}

// test_poly_z_add_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128  }
