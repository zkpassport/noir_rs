use bb_rs::barretenberg_api::acir::{acir_verify_ultra_honk, acir_verify_ultra_keccak_honk, acir_verify_ultra_keccak_zk_honk};

pub fn verify_ultra_honk(
    proof: Vec<u8>,
    verification_key: Vec<u8>,
) -> Result<bool, String> {
    Ok(unsafe {
        let result = acir_verify_ultra_honk( &proof, &verification_key);
        result
    })
}

pub fn verify_ultra_keccak_honk(
    proof: Vec<u8>,
    verification_key: Vec<u8>,
    zk: bool,
) -> Result<bool, String> {
    Ok(unsafe {
        if zk {
            acir_verify_ultra_keccak_zk_honk( &proof, &verification_key)
        } else {
            acir_verify_ultra_keccak_honk( &proof, &verification_key)
        }
    })
}