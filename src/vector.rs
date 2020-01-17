//! implements the vector operations of ring elements

use crate::ring_element::*;
use zeroize::Zeroize;

pub(crate) const K: usize = 3;
pub(crate) const M: usize = 4;

#[derive(Debug, Clone, PartialEq, Zeroize)]
pub struct VectorOfRingElement(pub(crate) Vec<RingElement>);

#[derive(Debug, Clone, PartialEq, Zeroize)]
pub struct MatrixOfRingElement(pub(crate) Vec<Vec<RingElement>>);

impl VectorOfRingElement {
    pub fn inner_product(a: &Self, b: &Self) -> RingElement {
        assert_eq!(a.0.len(), b.0.len());
        let mut res: RingElement = PolyArith::zero();
        for i in 0..K {
            res.add_assign(&PolyArith::mul(&a.0[i], &b.0[i]));
        }
        res
    }

    pub fn left_mul_matrix(a: &Self, b: &MatrixOfRingElement) {
        assert_eq!(a.0.len(), b.0.len());
        let c = MatrixOfRingElement::transpose(b);
        let mut res = Self(vec![]);
        for e in c.0 {
            res.0.push(Self::inner_product(&a, &Self(e)))
        }
    }
}

impl MatrixOfRingElement {
    pub fn dimension(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }

    pub fn transpose(&self) -> Self {
        let (row, column) = self.dimension();

        let mut res: Vec<Vec<RingElement>> = vec![];
        for i in 0..column {
            let mut tmp: Vec<RingElement> = vec![];
            for j in 0..row {
                tmp.push(self.0[i][j].clone());
            }
            res.push(tmp);
        }
        Self(res)
    }

    pub fn transpose_mut(&mut self) {
        *self = self.transpose();
    }
}
