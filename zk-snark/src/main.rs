//! This is an example of using the `bellman` crate to create a circuit that adds two numbers together.
//! The circuit is then used to create a proof that the two numbers were added together correctly.
//! The proof is then verified to ensure that it is valid.
//! Statement of the problem: Given two numbers `a` and `b`, prove that `a + b = 42`.

use pairing::{
    Engine,
};
use bls12_381::{Bls12, Scalar};
use rand::rngs::OsRng;
use bellman::{
    groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key,
    },
    Circuit, ConstraintSystem, SynthesisError, Variable,
};
use blake2::Blake2s;
use ff::PrimeField;

// This is a simple circuit with two values.
struct ZkSnarkCircuit {
    a: Option<Scalar>,
    b: Option<Scalar>,
}

impl<S: PrimeField> Circuit<S> for ZkSnarkCircuit {
    fn synthesize<CS: ConstraintSystem<S>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Allocate the first number as a private input to the circuit.
        // let a = cs.alloc(|| "a", || self.a.ok_or(SynthesisError::AssignmentMissing))?; // a = 1
        // println!("a is after alloc: {:?}", a);
        // let b = cs.alloc(|| "b", || self.b.ok_or(SynthesisError::AssignmentMissing))?; // b = 2
        // println!("b is after alloc: {:?}", b);
        Ok(())
    }
}



fn main() {
    let rng = &mut OsRng;

    // Create parameters for our circuit.
    // let params = {
    //     let c = ZkSnarkCircuit {
    //         a: Some(Scalar::from(30u64)),
    //         b: Some(Scalar::from(12u64)),
    //     };
    //     generate_random_parameters::<Bls12, _, _>(c, rng).unwrap()
    // };
}
