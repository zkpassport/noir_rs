use crate::backends::barretenberg::api::{
    self, configure_memory, ensure_msgpack_format, proof_bytes_to_fields, settings_ultra_honk_poseidon2, settings_ultra_honk_keccak,
    FIELD_ELEMENT_SIZE,
};
use crate::circuit::decode_circuit;

/// Split a flat proof byte vector into public_inputs and proof fields.
///
/// The proof format is: [num_public_inputs (4 bytes BE)] [public_input fields] [proof fields]
/// Each field is FIELD_ELEMENT_SIZE (32) bytes.
fn split_proof(proof: &[u8]) -> Result<(Vec<Vec<u8>>, Vec<Vec<u8>>), String> {
    if proof.len() < 4 {
        return Err("Proof too short to contain public inputs count".to_string());
    }
    let num_pub = u32::from_be_bytes(
        proof[0..4].try_into().map_err(|_| "Failed to read num_public_inputs")?
    ) as usize;
    let pub_bytes_len = num_pub * FIELD_ELEMENT_SIZE;
    if proof.len() < 4 + pub_bytes_len {
        return Err(format!(
            "Proof too short: expected at least {} bytes for {} public inputs, got {}",
            4 + pub_bytes_len, num_pub, proof.len()
        ));
    }
    let public_inputs = proof_bytes_to_fields(&proof[4..4 + pub_bytes_len]);
    let proof_fields = proof_bytes_to_fields(&proof[4 + pub_bytes_len..]);
    Ok((public_inputs, proof_fields))
}

/// Compute the Ultra Honk verification key for the given circuit.
///
/// Uses poseidon2 as the oracle hash function (matching `prove_ultra_honk`).
///
/// # Arguments
///
/// * `circuit_bytecode` - The base64-encoded circuit bytecode
/// * `low_memory_mode` - Whether to use file-backed memory for polynomials (slower but uses less RAM)
/// * `max_storage_usage` - Optional storage budget in bytes for file-backed memory
///
/// # Returns
/// * The serialized verification key bytes
pub fn get_ultra_honk_verification_key(
    circuit_bytecode: &str,
    low_memory_mode: bool,
    max_storage_usage: Option<u64>,
) -> Result<Vec<u8>, String> {
    ensure_msgpack_format();
    configure_memory(low_memory_mode, max_storage_usage);
    let (_, acir_buffer_uncompressed) =
        decode_circuit(circuit_bytecode).map_err(|e| format!("Failed to decode circuit: {}", e))?;

    let settings = settings_ultra_honk_poseidon2();
    let vk_response = api::circuit_compute_vk(&acir_buffer_uncompressed, &settings)?;
    Ok(vk_response.bytes)
}

/// Verify an Ultra Honk proof.
///
/// Uses poseidon2 as the oracle hash function (matching `prove_ultra_honk`).
///
/// # Arguments
///
/// * `proof` - The flat proof bytes (prefixed with 4-byte BE num_public_inputs)
/// * `verification_key` - The serialized verification key
///
/// # Returns
/// * Whether the proof is valid
pub fn verify_ultra_honk(proof: Vec<u8>, verification_key: Vec<u8>) -> Result<bool, String> {
    let settings = settings_ultra_honk_poseidon2();
    let (public_inputs, proof_fields) = split_proof(&proof)?;

    api::circuit_verify(&verification_key, public_inputs, proof_fields, &settings)
}

/// Compute the Keccak-variant Ultra Honk verification key for the given circuit.
///
/// Uses keccak as the oracle hash function (matching `prove_ultra_honk_keccak`).
///
/// # Arguments
///
/// * `circuit_bytecode` - The base64-encoded circuit bytecode
/// * `disable_zk` - Whether ZK is disabled (must match the proving setting)
/// * `low_memory_mode` - Whether to use file-backed memory for polynomials (slower but uses less RAM)
/// * `max_storage_usage` - Optional storage budget in bytes for file-backed memory
///
/// # Returns
/// * The serialized verification key bytes
pub fn get_ultra_honk_keccak_verification_key(
    circuit_bytecode: &str,
    disable_zk: bool,
    low_memory_mode: bool,
    max_storage_usage: Option<u64>,
) -> Result<Vec<u8>, String> {
    ensure_msgpack_format();
    configure_memory(low_memory_mode, max_storage_usage);
    let (_, acir_buffer_uncompressed) =
        decode_circuit(circuit_bytecode).map_err(|e| format!("Failed to decode circuit: {}", e))?;

    let settings = settings_ultra_honk_keccak(disable_zk);
    let vk_response = api::circuit_compute_vk(&acir_buffer_uncompressed, &settings)?;
    Ok(vk_response.bytes)
}

/// Verify a Keccak-variant Ultra Honk proof.
///
/// # Arguments
///
/// * `proof` - The flat proof bytes (prefixed with 4-byte BE num_public_inputs)
/// * `verification_key` - The serialized verification key
/// * `disable_zk` - Whether ZK was disabled during proving (must match)
///
/// # Returns
/// * Whether the proof is valid
pub fn verify_ultra_honk_keccak(
    proof: Vec<u8>,
    verification_key: Vec<u8>,
    disable_zk: bool,
) -> Result<bool, String> {
    let settings = settings_ultra_honk_keccak(disable_zk);
    let (public_inputs, proof_fields) = split_proof(&proof)?;

    api::circuit_verify(&verification_key, public_inputs, proof_fields, &settings)
}
