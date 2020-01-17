use crate::ring_element::*;

#[test]
fn basic_test() {
    let coeff = [1u64; N];
    let mut a = RingElement(coeff);
    println!("{:?}", a);

    let mut rng = rand::thread_rng();
    let b: RingElement = PolyArith::rand(&mut rng);
    println!("{:?}", b);

    let c = PolyArith::add(&a, &b);
    let d = PolyArith::add(&b, &a);
    a.add_assign(&b);
    assert_eq!(c, d);
    assert_eq!(a, c);

    let c = PolyArith::sub(&a, &b);
    a.sub_assign(&b);
    assert_eq!(c.0, coeff);
    assert_eq!(a, c);

    // a = x^7 + 1;
    let mut a = RingElement([0; N]);
    a.0[0] = 1;
    a.0[7] = 1;
    // b = x^3 + 4
    let mut b = RingElement([0; N]);
    b.0[0] = 4;
    b.0[3] = 1;

    let c = PolyArith::mul(&a, &b);
    a.mul_assign(&b);
    assert_eq!(a, c);
    println!("{:?}", a);

    assert!(false);
}
