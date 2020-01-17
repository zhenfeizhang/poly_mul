extern crate rand;
extern crate zeroize;

mod err;

use err::*;
use rand::Rng;
use std::ops::*;
use zeroize::Zeroize;

#[cfg(feature = "degree512")]
pub(crate) const N: usize = 512;
#[cfg(feature = "degree1024")]
pub(crate) const N: usize = 1024;
#[cfg(feature = "degree65536")]
pub(crate) const N: usize = 65536;

// #[derive(Debug, PartialEq, Clone, Zeroize)]
// pub struct Unsigned64Poly<T: Sized + Add + Sub + Rem + Mul> {
//     pub(crate) degree: usize,
//     pub(crate) coeff: Vec<T>,
//     pub(crate) modulus: u64,
// }

#[derive(Clone, Zeroize)]
pub struct Poly<T: Sized + Add + Sub + Rem + Mul> {
    pub(crate) degree: usize,
    pub(crate) coeff: [T; N],
    pub(crate) modulus: u64,
}

impl std::fmt::Debug for Poly<u64> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        // todo
        unimplemented!()
    }
}

impl std::cmp::PartialEq for Poly<u64> {
    fn eq(&self, _: &Self) -> bool {
        unimplemented!()
    }
}

// pub struct Signed64Poly {
//     pub(crate) degree: usize,
//     pub(crate) coeff: Vec<i64>,
//     pub(crate) modulus: u64,
// }

pub trait PolyArith {
    // arith
    fn add(a: &Self, b: &Self) -> Result<Self, String>
    where
        Self: std::marker::Sized;
    fn sub(a: &Self, b: &Self) -> Result<Self, String>
    where
        Self: std::marker::Sized;
    fn mul(a: &Self, b: &Self) -> Self;

    // assign
    fn zero() -> Self;
    fn rand<R: Rng + ?Sized>(rng: &mut R, modulus: u64) -> Self;

    // getters
    fn degree(&self) -> usize;
    fn modulus(&self) -> u64;
}

impl PolyArith for Poly<u64> {
    fn rand<R: Rng + ?Sized>(rng: &mut R, modulus: u64) -> Self {
        let mut v = [0u64; N];
        for i in 0..N {
            v[i] = random_u64_coeff(rng, modulus);
        }
        Poly {
            degree: N,
            coeff: v,
            modulus,
        }
    }

    fn zero() -> Self {
        Poly {
            degree: N,
            coeff: [0; N],
            modulus: 0,
        }
    }

    fn add(a: &Self, b: &Self) -> Result<Self, String> {
        if a.degree != b.degree {
            return Err(ERR_DEGREE_MISMATCH.to_string());
        }
        if a.modulus != b.modulus {
            return Err(ERR_MODULUS_MISMATCH.to_string());
        }
        let mut v = a.coeff.clone();
        for i in 0..N {
            v[i] = (v[i] + b.coeff[i]) % a.modulus;
        }

        Ok(Poly {
            degree: a.degree,
            coeff: v,
            modulus: a.modulus,
        })
    }

    fn sub(a: &Self, b: &Self) -> Result<Self, String> {
        if a.degree != b.degree {
            return Err(ERR_DEGREE_MISMATCH.to_string());
        }
        if a.modulus != b.modulus {
            return Err(ERR_MODULUS_MISMATCH.to_string());
        }

        let mut v = a.coeff.clone();
        for i in 0..N {
            v[i] = (v[i] - b.coeff[i]) % a.modulus;
        }

        Ok(Poly {
            degree: a.degree,
            coeff: v,
            modulus: a.modulus,
        })
    }

    fn mul(a: &Self, b: &Self) -> Self {
        Poly {
            degree: N,
            coeff: [0; N],
            modulus: 0,
        }
    }

    fn degree(&self) -> usize {
        self.degree
    }
    fn modulus(&self) -> u64 {
        self.modulus
    }
}

impl Poly<u64> {
    pub fn construct(coeff: [u64; N], modulus: u64) -> Self {
        Self {
            degree: N,
            coeff,
            modulus,
        }
    }
}

#[test]
fn basic_test() {
    let degree = 10;
    let coeff = vec![1u64, 2, 3, 4];
    let modulus = 12289u64;
    let mut a = Poly::construct(coeff, modulus);
    println!("{:?}", a);

    let mut rng = rand::thread_rng();
    let b: Poly<u64> = PolyArith::rand(&mut rng, modulus);
    println!("{:?}", b);

    let c = PolyArith::add(&a, &b);
    let d = PolyArith::add(&b, &a);
    assert_eq!(c, d);
    println!("{:?}", c);
    assert!(false);
}

fn random_u64_coeff<R: Rng + ?Sized>(rng: &mut R, modulus: u64) -> u64 {
    if is_pow_of_2(&modulus) {
        return rng.next_u64() % modulus;
    } else {
        // todo
        return rng.next_u64() % modulus;
    }
}

fn random_i64_coeff<R: Rng + ?Sized>(rng: &mut R, modulus: u64) -> u64 {
    if is_pow_of_2(&modulus) {
        return rng.next_u64() % modulus;
    } else {
        // todo
        return rng.next_u64() % modulus;
    }
}

fn is_pow_of_2(a: &u64) -> bool {
    [
        1,
        2,
        4,
        8,
        16,
        32,
        64,
        128,
        256,
        512,
        1024,
        2048,
        4096,
        8192,
        16384,
        32768,
        65536,
        131072,
        262144,
        524288,
        1048576,
        2097152,
        4194304,
        8388608,
        16777216,
        33554432,
        67108864,
        134217728,
        268435456,
        536870912,
        1073741824,
        2147483648,
        4294967296,
        8589934592,
        17179869184,
        34359738368,
        68719476736,
        137438953472,
        274877906944,
        549755813888,
        1099511627776,
        2199023255552,
        4398046511104,
        8796093022208,
        17592186044416,
        35184372088832,
        70368744177664,
        140737488355328,
        281474976710656,
        562949953421312,
        1125899906842624,
        2251799813685248,
        4503599627370496,
        9007199254740992,
        18014398509481984,
        36028797018963968,
        72057594037927936,
        144115188075855872,
        288230376151711744,
        576460752303423488,
        1152921504606846976,
        2305843009213693952,
        4611686018427387904,
        9223372036854775808,
    ]
    .contains(a)
}
