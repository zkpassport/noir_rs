use std::io::Read;

use base64::{engine::general_purpose, Engine};
use bb_rs::barretenberg_api::{
    acir::{
        acir_load_verification_key, acir_verify_proof, acir_verify_ultra_honk, delete_acir_composer, get_circuit_sizes, new_acir_composer
    },
    srs::init_srs,
};
use flate2::bufread::GzDecoder;

use crate::srs::{netsrs::NetSrs, Srs, get_srs};

fn decode_circuit(circuit_bytecode: String) -> Result<Vec<u8>, String> {
    let acir_buffer = general_purpose::STANDARD
        .decode(circuit_bytecode)
        .map_err(|e| e.to_string())?;

    let mut decoder = GzDecoder::new(acir_buffer.as_slice());
    let mut acir_buffer_uncompressed = Vec::<u8>::new();
    decoder
        .read_to_end(&mut acir_buffer_uncompressed)
        .map_err(|e| e.to_string())?;

    Ok(acir_buffer_uncompressed)
}

pub fn verify(
    circuit_bytecode: String,
    proof: Vec<u8>,
    verification_key: Vec<u8>,
    srs_path: Option<&str>,
) -> Result<bool, String> {
    let acir_buffer_uncompressed = decode_circuit(circuit_bytecode)?;

    let srs: Srs = get_srs(&acir_buffer_uncompressed, srs_path);

    Ok(unsafe {
        init_srs(&srs.g1_data, srs.num_points, &srs.g2_data);
        let mut acir_ptr = new_acir_composer(srs.num_points - 1);
        acir_load_verification_key(&mut acir_ptr, &verification_key);
        let result = acir_verify_proof(&mut acir_ptr, &proof);
        delete_acir_composer(acir_ptr);
        result
    })
}

pub fn verify_honk(
    circuit_bytecode: String,
    proof: Vec<u8>,
    verification_key: Vec<u8>,
    srs_path: Option<&str>,
) -> Result<bool, String> {
    let acir_buffer_uncompressed = decode_circuit(circuit_bytecode)?;

    let srs: Srs = get_srs(&acir_buffer_uncompressed, srs_path);

    Ok(unsafe {
        init_srs(&srs.g1_data, srs.num_points, &srs.g2_data);
        let result = acir_verify_ultra_honk( &proof, &verification_key);
        result
    })
}