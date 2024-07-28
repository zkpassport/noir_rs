use std::io::Read;
use base64::{engine::general_purpose, Engine};
use flate2::bufread::GzDecoder;
use bb_rs::barretenberg_api::acir::get_circuit_sizes;

pub fn decode_circuit(circuit_bytecode: String) -> Result<(Vec<u8>, Vec<u8>), String> {
    let acir_buffer = general_purpose::STANDARD
        .decode(circuit_bytecode)
        .map_err(|e| e.to_string())?;

    let mut decoder = GzDecoder::new(acir_buffer.as_slice());
    let mut acir_buffer_uncompressed = Vec::<u8>::new();
    decoder
        .read_to_end(&mut acir_buffer_uncompressed)
        .map_err(|e| e.to_string())?;

    Ok((acir_buffer, acir_buffer_uncompressed))
}

pub fn get_subgroup_size(circuit_bytecode: String) -> u32 {
    let (_, acir_buffer_uncompressed) = if let Ok(acir_buffer_uncompressed) = decode_circuit(circuit_bytecode) {
        acir_buffer_uncompressed
    } else {
        return 0;
    };  

    let circuit_size = unsafe { get_circuit_sizes(&acir_buffer_uncompressed) };
    let log_value = (circuit_size.total as f64).log2().ceil() as u32;
    let subgroup_size = 2u32.pow(log_value);
    subgroup_size
}