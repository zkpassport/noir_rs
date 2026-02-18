use crate::backends::barretenberg::api::{self, settings_ultra_honk_poseidon2};
use crate::circuit::decode_circuit;

/// Compute the next power of two that is >= `circuit_size`.
pub fn compute_subgroup_size(circuit_size: u32) -> u32 {
    let log_value = (circuit_size as f64).log2().ceil() as u32;
    2u32.pow(log_value)
}

/// Get the total gate count (circuit size) for the given bytecode.
pub fn get_circuit_size(circuit_bytecode: &str, _recursion: bool) -> u32 {
    let (_, acir_buffer_uncompressed) = if let Ok(decoded) = decode_circuit(circuit_bytecode) {
        decoded
    } else {
        return 0;
    };

    let settings = settings_ultra_honk_poseidon2();

    match api::circuit_stats(&acir_buffer_uncompressed, &settings) {
        Ok(info) => info.num_gates,
        Err(_) => 0,
    }
}

/// Get the dyadic (next power-of-two) circuit size for the given bytecode.
pub fn get_circuit_size_dyadic(circuit_bytecode: &str) -> u32 {
    let (_, acir_buffer_uncompressed) = if let Ok(decoded) = decode_circuit(circuit_bytecode) {
        decoded
    } else {
        return 0;
    };

    let settings = settings_ultra_honk_poseidon2();

    match api::circuit_stats(&acir_buffer_uncompressed, &settings) {
        Ok(info) => info.num_gates_dyadic,
        Err(_) => 0,
    }
}

/// Get the subgroup size (next power of two >= circuit size) for the given bytecode.
pub fn get_subgroup_size(circuit_bytecode: &str, recursion: bool) -> u32 {
    let circuit_size = get_circuit_size(circuit_bytecode, recursion);
    compute_subgroup_size(circuit_size)
}
