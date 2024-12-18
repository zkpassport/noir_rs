use bb_rs::barretenberg_api::acir::acir_verify_ultra_honk;

pub fn verify_ultra_honk(
    proof: Vec<u8>,
    verification_key: Vec<u8>,
) -> Result<bool, String> {
    Ok(unsafe {
        let result = acir_verify_ultra_honk( &proof, &verification_key);
        result
    })
}