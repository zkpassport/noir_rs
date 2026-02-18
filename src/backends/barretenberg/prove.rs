use acvm::acir::{native_types::WitnessMap, FieldElement};

use crate::backends::barretenberg::api::{
    self, configure_memory, proof_fields_to_bytes, settings_ultra_honk_poseidon2,
    settings_ultra_honk_keccak,
};
use crate::circuit::get_acir_buffer_uncompressed;
use crate::execute::execute;
use crate::witness::serialize_witness;

/// Generate an Ultra Honk proof for the given circuit bytecode and initial witness.
/// Will execute the circuit to make sure it is solved.
///
/// Uses poseidon2 as the oracle hash function (suitable for recursive verification in Noir).
///
/// # Arguments
///
/// * `circuit_bytecode` - The base64-encoded circuit bytecode
/// * `initial_witness` - The initial witness to use for the proof
/// * `verification_key` - The verification key (pass empty vec for first-time proving)
/// * `low_memory_mode` - Whether to use file-backed memory for polynomials (slower but uses less RAM)
/// * `max_storage_usage` - Optional storage budget in bytes for file-backed memory
///
/// # Returns
/// * The proof as a flat byte vector
pub fn prove_ultra_honk(
    circuit_bytecode: &str,
    initial_witness: WitnessMap<FieldElement>,
    verification_key: Vec<u8>,
    low_memory_mode: bool,
    max_storage_usage: Option<u64>,
) -> Result<Vec<u8>, String> {
    configure_memory(low_memory_mode, max_storage_usage);
    let witness_stack = execute(circuit_bytecode, initial_witness)?;
    let serialized_solved_witness = serialize_witness(witness_stack)?;
    let acir_buffer_uncompressed = get_acir_buffer_uncompressed(circuit_bytecode)?;

    let settings = settings_ultra_honk_poseidon2();
    let response = api::circuit_prove(
        &acir_buffer_uncompressed,
        &serialized_solved_witness,
        &verification_key,
        &settings,
    )?;

    // Encode as: [num_public_inputs (4 bytes BE)] [public_inputs] [proof]
    let num_pub = response.public_inputs.len() as u32;
    let mut result = num_pub.to_be_bytes().to_vec();
    result.extend(proof_fields_to_bytes(&response.public_inputs));
    result.extend(proof_fields_to_bytes(&response.proof));
    Ok(result)
}

/// Generate an Ultra Honk proof using Keccak as the oracle hash function.
/// This is suitable for on-chain (EVM/Solidity) verification.
///
/// # Arguments
///
/// * `circuit_bytecode` - The base64-encoded circuit bytecode
/// * `initial_witness` - The initial witness to use for the proof
/// * `verification_key` - The verification key (pass empty vec for first-time proving)
/// * `disable_zk` - Whether to disable zero-knowledge (set true for public-input-only circuits)
/// * `low_memory_mode` - Whether to use file-backed memory for polynomials (slower but uses less RAM)
/// * `max_storage_usage` - Optional storage budget in bytes for file-backed memory
///
/// # Returns
/// * The proof as a flat byte vector
pub fn prove_ultra_honk_keccak(
    circuit_bytecode: &str,
    initial_witness: WitnessMap<FieldElement>,
    verification_key: Vec<u8>,
    disable_zk: bool,
    low_memory_mode: bool,
    max_storage_usage: Option<u64>,
) -> Result<Vec<u8>, String> {
    configure_memory(low_memory_mode, max_storage_usage);
    let witness_stack = execute(circuit_bytecode, initial_witness)?;
    let serialized_solved_witness = serialize_witness(witness_stack)?;
    let acir_buffer_uncompressed = get_acir_buffer_uncompressed(circuit_bytecode)?;

    let settings = settings_ultra_honk_keccak(disable_zk);
    let response = api::circuit_prove(
        &acir_buffer_uncompressed,
        &serialized_solved_witness,
        &verification_key,
        &settings,
    )?;

    // Encode as: [num_public_inputs (4 bytes BE)] [public_inputs] [proof]
    let num_pub = response.public_inputs.len() as u32;
    let mut result = num_pub.to_be_bytes().to_vec();
    result.extend(proof_fields_to_bytes(&response.public_inputs));
    result.extend(proof_fields_to_bytes(&response.proof));
    Ok(result)
}
