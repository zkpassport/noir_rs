use core::num;
use std::io::Read;
use acir::{circuit::Program, native_types::{WitnessMap, WitnessStack}, FieldElement};
use bb_rs::barretenberg_api::acir::{
        acir_vk_as_fields_ultra_honk, acir_proof_as_fields_ultra_honk
};

pub fn generate_recursive_honk_proof_artifacts(    
    proof_bytes: Vec<u8>,
    vk_bytes: Vec<u8>
) -> Result<(Vec<String>, Vec<String>, String), String> {
    Ok(unsafe {
        let proof = acir_proof_as_fields_ultra_honk(&proof_bytes);
        let (vk, key_hash) = acir_vk_as_fields_ultra_honk(&vk_bytes);
        // Add the first 3 fields of the proof to the verification key
        // Temporary solution until a proper fix is found for the first 3 fields of the verification key
        let mut complete_vk = Vec::from(&proof[..3]);
        complete_vk.extend_from_slice(&vk);
        // Get the number of public inputs from the third field of the proof
        // by parsing from hex to usize
        let num_public_inputs = usize::from_str_radix(proof[2].trim_start_matches("0x"), 16).unwrap();
        let end_index_for_proof_without_public_inputs = 3 + num_public_inputs;
        // We keep the first 3 fields but remove the following public inputs and keep the rest
        let mut proof_without_public_inputs: Vec<String> = Vec::from(&proof[..3]);
        proof_without_public_inputs.extend_from_slice(&proof[end_index_for_proof_without_public_inputs..]);

        (proof_without_public_inputs, complete_vk, key_hash)
    })
}