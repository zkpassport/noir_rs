use bb_rs::barretenberg_api::acir::{
    acir_get_honk_verification_key, acir_vk_as_fields_ultra_honk, get_circuit_sizes,
};

use crate::circuit::decode_circuit;

pub fn get_honk_verification_key(circuit_bytecode: &str) -> Result<Vec<u8>, String> {
    let (_, acir_buffer_uncompressed) = decode_circuit(circuit_bytecode)
        .map_err(|e| format!("Failed to decode circuit: {}", e))?;

    let result = unsafe {
        acir_get_honk_verification_key(&acir_buffer_uncompressed)
    };
    Ok(result)
}


pub fn compute_subgroup_size(circuit_size: u32) -> u32 {
    let log_value = (circuit_size as f64).log2().ceil() as u32;
    let subgroup_size = 2u32.pow(log_value);
    subgroup_size
}

pub fn get_circuit_size(circuit_bytecode: &str, recursion: bool) -> u32 {
    let (_, acir_buffer_uncompressed) = if let Ok(acir_buffer_uncompressed) = decode_circuit(circuit_bytecode) {
        acir_buffer_uncompressed
    } else {
        return 0;
    };

    let circuit_size = unsafe { get_circuit_sizes(&acir_buffer_uncompressed, recursion) };
    circuit_size.total
}

pub fn get_subgroup_size(circuit_bytecode: &str, recursion: bool) -> u32 {
    let circuit_size = get_circuit_size(circuit_bytecode, recursion);
    compute_subgroup_size(circuit_size)
}