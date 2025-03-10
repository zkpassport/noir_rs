use bb_rs::barretenberg_api::acir::{acir_verify_proof, acir_verify_ultra_honk};
use bb_rs::barretenberg_api::models::Ptr;

pub fn verify_ultra_honk(
    proof: Vec<u8>,
    verification_key: Vec<u8>,
) -> Result<bool, String> {
    Ok(unsafe {
        let result = acir_verify_ultra_honk( &proof, &verification_key);
        result
    })
}


pub fn verify_ultra_plonk(
    proof: Vec<u8>,
    acir_composer_ptr: &mut Ptr
) -> Result<bool, String> {
    Ok(unsafe {
        let result = acir_verify_proof(acir_composer_ptr ,&proof);
        result
    })
}