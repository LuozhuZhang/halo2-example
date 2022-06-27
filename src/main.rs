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

trait NumbericInstructions<F: FieldExt>: Chip<F> {
    type Num;

    // * Load a number into circuit

    // As a private input
    fn load_private(&self, layouter: impl Layouter<F>, a: Value<F>) -> Result<Self::Num, Error>;

    // As a fixed constant (fixed column)
    fn load_constant(&self, layouter: impl Layouter<F>, constant: F) -> Result<Self::Num, Error>;

    // 最简单的乘法门，c = a * b
    // 可以由`c = a * b`的basic gate组成`a^2 = a * a`
    fn mul(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error>;

    // 输出public input
    // expose a number as a public input to the circuit
    fn expose_public(
        &self,
        layouter: impl Layouter<F>,
        num: Self::Num,
        row: usize,
    ) -> Result<(), Error>;
}

fn main() {
    println!("Hello, world! ");
}
