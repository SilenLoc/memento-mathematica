use bigdecimal::{BigDecimal, One, Zero};
use std::ops::{Add, Sub};

pub fn addition(left: BigDecimal, right: BigDecimal) -> BigDecimal {
    left.add(right)
}

pub fn subtraction(left: BigDecimal, right: BigDecimal) -> BigDecimal {
    left.sub(right)
}

pub fn ack(m: BigDecimal, n: BigDecimal) -> BigDecimal {
    if m == BigDecimal::zero() {
        return n.add(1);
    }
    if n == BigDecimal::zero() {
        return ack(m.clone().sub(BigDecimal::one()), BigDecimal::one());
    }
    ack(
        m.clone().sub(BigDecimal::one()),
        ack(m, n.sub(BigDecimal::one())),
    )
}
