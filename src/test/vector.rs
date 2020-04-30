use crate::ring_element::*;
use crate::vector::*;

#[test]
fn basic_vector_arith_test() {
    let mut coeff = [0; N];
    coeff[0] = 1;
    coeff[1] = 1;
    let a = RingElement(coeff);

    let v1 = VectorOfRingElement(vec![a.clone(); 3]);
    let v2 = v1.clone();
    let c = VectorOfRingElement::inner_product(&v1, &v2);

    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", c);

    let m1 = MatrixOfRingElement(vec![vec![a.clone(); 4]; 3]);
    let v4 = VectorOfRingElement::left_mul_matrix(&v1, &m1);
    println!("{:?}", v4);

    assert!(false)
}
