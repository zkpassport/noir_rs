use acir::{native_types::WitnessMap, FieldElement};
use bb_rs::barretenberg_api::acir::{acir_create_proof, acir_get_honk_verification_key, acir_get_verification_key, acir_prove_ultra_honk, get_circuit_sizes, new_acir_composer};
use std::ptr;
use bb_rs::barretenberg_api::common::example_simple_create_and_verify_proof;
use bb_rs::barretenberg_api::srs::init_srs;
use crate::barretenberg::srs::netsrs::NetSrs;
use crate::circuit::get_acir_buffer_uncompressed;
use crate::execute::execute;
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

pub fn prove_ultra_plonk(
    circuit_bytecode: &str,
    initial_witness: WitnessMap<FieldElement>,
    recursive: bool,
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let witness_stack = execute(circuit_bytecode, initial_witness)?;
    let serialized_solved_witness = serialize_witness(witness_stack)?;
    let acir_buffer_uncompressed = get_acir_buffer_uncompressed(circuit_bytecode)?;


    let circuit_size = unsafe {
        get_circuit_sizes(&acir_buffer_uncompressed, recursive)
    };


    let mut composer = unsafe{
        new_acir_composer(circuit_size.total)
    };

    Ok(unsafe {
        let result = (
            acir_create_proof(
                &mut composer,
                &acir_buffer_uncompressed,
                &serialized_solved_witness,
                recursive,
            ),
            acir_get_verification_key(&mut composer),
        );
        result
    })
}

