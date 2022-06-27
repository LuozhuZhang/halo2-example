use std::marker::PhantomData;

// 我们主要使用的module
use halo2_proofs::{
    // finite field
    arithmetic::FieldExt,
    // circuit和对table的操作
    circuit::{AssignedCell, Chip, Layouter, Region, SimpleFloorPlanner, Value},
    // 主要是lookup table
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Fixed, Instance, Selector},
    // polynomial
    poly::Rotation,
};

fn main() {
    println!("Hello, world! {:?}", );
}
