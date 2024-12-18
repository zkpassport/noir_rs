use bb_rs::barretenberg_api::acir::{
    acir_get_honk_verification_key, acir_vk_as_fields_ultra_honk, get_circuit_sizes,
};

use crate::circuit::decode_circuit;

pub fn get_honk_vkey_hash(vk_bytes: Vec<u8>) -> Result<String, String> {
    Ok(unsafe {
        let (_, key_hash) = acir_vk_as_fields_ultra_honk(&vk_bytes);
        key_hash
    })
}

pub fn get_honk_verification_key(circuit_bytecode: &str, recursive: bool) -> Result<Vec<u8>, String> {
    let (_, acir_buffer_uncompressed) = decode_circuit(circuit_bytecode)
        .map_err(|e| format!("Failed to decode circuit: {}", e))?;

    let result = unsafe {
        acir_get_honk_verification_key(&acir_buffer_uncompressed, recursive)
    };
    Ok(result)
}

pub fn get_subgroup_size(circuit_bytecode: &str, recursion: bool) -> u32 {
    let (_, acir_buffer_uncompressed) = if let Ok(acir_buffer_uncompressed) = decode_circuit(circuit_bytecode) {
        acir_buffer_uncompressed
    } else {
        return 0;
    };

    let circuit_size = unsafe { get_circuit_sizes(&acir_buffer_uncompressed, recursion) };
    let log_value = (circuit_size.total as f64).log2().ceil() as u32;
    let subgroup_size = 2u32.pow(log_value);
    subgroup_size
}
