use bb_rs::barretenberg_api::acir::{
    acir_load_verification_key, acir_verify_proof, acir_verify_ultra_honk, delete_acir_composer, new_acir_composer
};


pub fn verify(
    proof: Vec<u8>,
    verification_key: Vec<u8>,
    num_points: u32,
) -> Result<bool, String> {
    Ok(unsafe {
        let mut acir_ptr = new_acir_composer(num_points - 1);
        acir_load_verification_key(&mut acir_ptr, &verification_key);
        let result = acir_verify_proof(&mut acir_ptr, &proof);
        delete_acir_composer(acir_ptr);
        result
    })
}

pub fn verify_honk(
    proof: Vec<u8>,
    verification_key: Vec<u8>,
) -> Result<bool, String> {
    Ok(unsafe {
        let result = acir_verify_ultra_honk( &proof, &verification_key);
        result
    })
}