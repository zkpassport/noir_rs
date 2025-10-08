use acvm::acir::{native_types::WitnessMap, FieldElement};
use bb_rs::barretenberg_api::acir::{ acir_prove_ultra_honk, acir_prove_ultra_keccak_honk, acir_prove_ultra_keccak_zk_honk};

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
/// * verification_key: The verification key to use for the proof
/// 
/// # Returns
/// * The proof
pub fn prove_ultra_honk(
    circuit_bytecode: &str,
    initial_witness: WitnessMap<FieldElement>,
    verification_key: Vec<u8>,
    low_memory_mode: bool,
    max_storage_usage: Option<u64>,
) -> Result<Vec<u8>, String> {
    let witness_stack = execute(circuit_bytecode, initial_witness)?;
    let serialized_solved_witness = serialize_witness(witness_stack)?;
    let acir_buffer_uncompressed = get_acir_buffer_uncompressed(circuit_bytecode)?;

    Ok(unsafe {
        acir_prove_ultra_honk(&acir_buffer_uncompressed, &serialized_solved_witness, &verification_key, low_memory_mode, max_storage_usage)
    })
}

/// Generate an Ultra Honk proof for the given circuit bytecode and initial witness
/// Will execute the circuit to make sure it is solved
/// Unlike the standard Ultra Honk proof, this proof uses the Keccak hash function
/// instead of Poseidon hash function for the random oracle
/// 
/// If zk is true, the proof will be fully zero-knowledge
/// 
/// # Arguments
/// 
/// * circuit_bytecode: The circuit bytecode to prove
/// * initial_witness: The initial witness to use for the proof
/// * verification_key: The verification key to use for the proof
/// * disable_zk: Whether to disable the zero-knowledge property of the proof
/// 
/// # Returns
/// * The proof
pub fn prove_ultra_honk_keccak(
    circuit_bytecode: &str,
    initial_witness: WitnessMap<FieldElement>,
    verification_key: Vec<u8>,
    disable_zk: bool,
    low_memory_mode: bool,
    max_storage_usage: Option<u64>,
) -> Result<Vec<u8>, String> {
    let witness_stack = execute(circuit_bytecode, initial_witness)?;
    let serialized_solved_witness = serialize_witness(witness_stack)?;
    let acir_buffer_uncompressed = get_acir_buffer_uncompressed(circuit_bytecode)?;

    Ok(unsafe {
        if disable_zk {
            acir_prove_ultra_keccak_honk(&acir_buffer_uncompressed, &serialized_solved_witness, &verification_key, low_memory_mode, max_storage_usage)
        } else {
            acir_prove_ultra_keccak_zk_honk(&acir_buffer_uncompressed, &serialized_solved_witness, &verification_key, low_memory_mode, max_storage_usage)
        }
    })
}