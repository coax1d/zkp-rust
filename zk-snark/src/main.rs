//! This is an example of using the `bellman` crate to create a circuit that adds two numbers together.
//! The circuit is then used to create a proof that the two numbers were added together correctly.
//! The proof is then verified to ensure that it is valid.
//! Statement of the problem: Given two numbers `a` and `b`, prove that `a + b = 42`.

use pairing::{Engine, MultiMillerLoop};
use bls12_381::{Bls12, Scalar};
use rand::rngs::OsRng;
use bellman::{
    gadgets::num::AllocatedNum,
    groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, self,
    },
    Circuit, ConstraintSystem, SynthesisError, Variable,
};

use ff::PrimeField;

// Circuit y = x^2 + a
// Where:
// x, a are private and y is public
struct FirstCircuit<S> {
    pub a: Option<S>,
    pub x: Option<S>,
}

impl<S: PrimeField> Circuit<S> for FirstCircuit<S> {
    fn synthesize<CS: ConstraintSystem<S>>(self, cs: &mut CS) -> Result<(), SynthesisError> {

        // allocate private variable x
        let x = cs.alloc(|| "x", || self.x.ok_or(SynthesisError::AssignmentMissing))?;

        // Save result of x^2 for later
        let mut x_squared_scalar = S::from(0);
        let x_squared = cs.alloc(|| "x_squared", ||{
            let tmp_x = self.x.ok_or(SynthesisError::AssignmentMissing)?;
            let x_squared = tmp_x * tmp_x;
            x_squared_scalar = x_squared;
            Ok(x_squared)
        })?;

        cs.enforce(
            || "x * x = x^2",
            |lc| lc + x,
            |lc| lc + x,
            |lc| lc + x_squared
        );

        // allocate other private variable a
        let a = cs.alloc(|| "a", || self.a.ok_or(SynthesisError::AssignmentMissing))?;

        // allocate public variable y
        let y = cs.alloc_input(|| "y", || {
            let a = self.a.ok_or(SynthesisError::AssignmentMissing)?;
            let x = self.x.ok_or(SynthesisError::AssignmentMissing)?;
            let y_scalar = a + x_squared_scalar;
            Ok(y_scalar)
        })?;

        cs.enforce(
            || "y = x^2 + a",
            |lc| lc + a + x_squared,
            |lc| lc + CS::one(),
            |lc| lc + y
        );

        Ok(())
    }
}



fn main() {
    let rng = &mut OsRng;

    // Create some random params for setup
    let params = {
        let c = FirstCircuit { a: None, x: None };
        groth16::generate_random_parameters::<Bls12, _, _>(c, rng).unwrap()
    };

    // prep a verification key
    let pvk = groth16::prepare_verifying_key(&params.vk);

    let my_circuit = FirstCircuit { a: Some(Scalar::from(3)), x: Some(Scalar::from(5)) };

    // Create Groth16 proof with custom params
    let proof = groth16::create_random_proof(my_circuit, &params, rng).unwrap();

    let solution = (5 * 5) + 3;
    let y = Scalar::from(solution);

    assert!(groth16::verify_proof(&pvk, &proof, &[y]).is_ok());
}
