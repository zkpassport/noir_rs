use acir::{native_types::WitnessMap, FieldElement};
use bb_rs::barretenberg_api::acir::{ acir_get_honk_verification_key, acir_prove_ultra_honk};

use crate::execute::execute;
use crate::circuit::get_acir_buffer_uncompressed;
use crate::witness::serialize_witness;

/// Generate an Ultra Honk proof for the given circuit bytecode and initial witness
/// Will execute the circuit to make sure it is solved
/// 
/// # Arguments
/// 
/// * circuit_bytecode: The circuit bytecode to prove
/// * initial_witness: The initial witness to use for the proof
/// * recursive: Whether the circuit is recursive
/// 
/// # Returns
/// * The proof and the verification key
pub fn prove_ultra_honk(
    circuit_bytecode: &str,
    initial_witness: WitnessMap<FieldElement>,
    recursive: bool,
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let witness_stack = execute(circuit_bytecode, initial_witness)?;
    let serialized_solved_witness = serialize_witness(witness_stack)?;
    let acir_buffer_uncompressed = get_acir_buffer_uncompressed(circuit_bytecode)?;

    Ok(unsafe {
        let result = (
            acir_prove_ultra_honk(
                &acir_buffer_uncompressed,
                &serialized_solved_witness,
                recursive,
            ),
            acir_get_honk_verification_key(&acir_buffer_uncompressed, recursive),
        );
        result
    })
}