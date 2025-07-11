use bb_rs::barretenberg_api::acir::{
    acir_verify_ultra_honk, acir_verify_ultra_keccak_honk, acir_verify_ultra_keccak_zk_honk, 
    acir_get_ultra_honk_verification_key, acir_get_ultra_honk_keccak_verification_key, 
    acir_get_ultra_honk_keccak_zk_verification_key
};
use crate::circuit::decode_circuit;

pub fn get_ultra_honk_verification_key(circuit_bytecode: &str) -> Result<Vec<u8>, String> {
    let (_, acir_buffer_uncompressed) = decode_circuit(circuit_bytecode)
        .map_err(|e| format!("Failed to decode circuit: {}", e))?;

    let result = unsafe {
        acir_get_ultra_honk_verification_key(&acir_buffer_uncompressed)
    };
    Ok(result)
}

pub fn verify_ultra_honk(
    proof: Vec<u8>,
    verification_key: Vec<u8>,
) -> Result<bool, String> {
    Ok(unsafe {
        let result = acir_verify_ultra_honk(&proof, &verification_key);
        result
    })
}

pub fn get_ultra_honk_keccak_verification_key(circuit_bytecode: &str, disable_zk: bool) -> Result<Vec<u8>, String> {
    let (_, acir_buffer_uncompressed) = decode_circuit(circuit_bytecode)
        .map_err(|e| format!("Failed to decode circuit: {}", e))?;
    
    let result = unsafe {
        if disable_zk {
            acir_get_ultra_honk_keccak_verification_key(&acir_buffer_uncompressed)
        } else {
            acir_get_ultra_honk_keccak_zk_verification_key(&acir_buffer_uncompressed)
        }
    };

    Ok(result)
}

pub fn verify_ultra_honk_keccak(
    proof: Vec<u8>,
    verification_key: Vec<u8>,
    disable_zk: bool,
) -> Result<bool, String> {
    Ok(unsafe {
        if disable_zk {
            acir_verify_ultra_keccak_honk(&proof, &verification_key)
        } else {
            acir_verify_ultra_keccak_zk_honk(&proof, &verification_key)
        }
    })
}