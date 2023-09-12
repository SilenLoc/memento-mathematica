use bigdecimal::BigDecimal;
use std::ops::{Add, Sub};

pub fn addition(left: BigDecimal, right: BigDecimal) -> BigDecimal {
    left.add(right)
}

pub fn subtraction(left: BigDecimal, right: BigDecimal) -> BigDecimal {
    left.sub(right)
}
