//! implements the polynomial arithmetics

use rand::Rng;
use zeroize::Zeroize;


pub(crate) const N: usize = 8;
pub(crate) const Q: u64 = 64;

// #[cfg(feature = "degree512")]
// pub(crate) const N: usize = 256;
// #[cfg(feature = "degree512")]
// pub(crate) const N: usize = 512;
// #[cfg(feature = "degree1024")]
// pub(crate) const N: usize = 1024;
// #[cfg(feature = "degree65536")]
// pub(crate) const N: usize = 65536;

// #[derive(Debug, PartialEq, Clone, Zeroize)]
// pub struct Unsigned64Poly<T: Sized + Add + Sub + Rem + Mul> {
//     pub(crate) degree: usize,
//     pub(crate) coeff: Vec<T>,
//     pub(crate) modulus: u64,
// }

#[derive(Clone, Zeroize)]
pub struct RingElement(pub(crate) [u64; N]);

impl std::fmt::Debug for RingElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for i in 0..N / 8 {
            for j in 0..8 {
                write!(f, "{:04}, ", self.0[i * 8 + j])?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

impl std::cmp::PartialEq for RingElement {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if self.0[i] != other.0[i] {
                return false;
            }
        }
        true
    }
}

pub trait PolyArith {
    // arith
    fn add(a: &Self, b: &Self) -> Self;
    fn add_assign(&mut self, b: &Self);

    fn sub(a: &Self, b: &Self) -> Self;
    fn sub_assign(&mut self, b: &Self);

    fn mul(a: &Self, b: &Self) -> Self;
    fn mul_assign(&mut self, b: &Self);

    // assign
    fn zero() -> Self;


    // random polynomials
    fn rand<R: Rng + ?Sized>(rng: &mut R) -> Self;
    fn gaussian<R: Rng + ?Sized>(rng: &mut R) -> Self;

    // getters
    fn degree(&self) -> usize;
    fn modulus(&self) -> u64;
}

impl Default for RingElement{
    fn default() -> Self {
        RingElement([0; N])
    }
}

impl PolyArith for RingElement {
    fn rand<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut v = [0u64; N];
        for i in 0..N {
            v[i] = random_u64_coeff(rng, Q);
        }
        RingElement(v)
    }

    fn gaussian<R: Rng + ?Sized>(_rng: &mut R) -> Self{
        // unimplemented!
        Self::zero()
    }


    fn zero() -> Self {
        RingElement([0; N])
    }

    fn add(a: &Self, b: &Self) -> Self {
        let mut v = a.0;
        for i in 0..N {
            v[i] = (v[i] + b.0[i]) % Q;
        }
        RingElement(v)
    }

    fn add_assign(&mut self, b: &Self) {
        *self = Self::add(self, b);
    }

    fn sub(a: &Self, b: &Self) -> Self {
        let mut v = a.0;
        for i in 0..N {
            v[i] = (v[i] + Q - b.0[i]) % Q;
        }
        RingElement(v)
    }

    fn sub_assign(&mut self, b: &Self) {
        *self = Self::sub(self, b);
    }

    /// currently implements school book multiplication
    /// TODO: use NTT
    fn mul(a: &Self, b: &Self) -> Self {
        //  for(j=0; j<N; j++)
        //  {
        //      res[j] = a[0]*b[j];
        //  }
        //  for(i=1; i<N; i++)
        //  {
        //      res[i+N-1] = 0;
        //      for(j=0; j<N; j++)
        //      {
        //          res[i+j] += a[i]*b[j];
        //      }
        //  }
        //  res[2*N-1] = 0;
        let mut res: Vec<u64> = vec![0; N * 2];
        let mut array = [0; N];
        for i in 0..N {
            for j in 0..N {
                res[i + j] += a.0[i] * b.0[j];
            }
        }
        for i in 0..N {
            array[i] = (res[i] + Q - res[i + N]) % Q;
        }

        RingElement(array)
    }

    fn mul_assign(&mut self, b: &Self) {
        *self = Self::mul(self, b);
    }

    fn degree(&self) -> usize {
        N
    }
    fn modulus(&self) -> u64 {
        Q
    }
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
