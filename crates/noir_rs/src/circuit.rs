use core::num;
use std::io::Read;

use acir::{circuit::Program, native_types::{WitnessMap, WitnessStack}, FieldElement};
use base64::{engine::general_purpose, Engine};
use bb_rs::barretenberg_api::{acir::{
        acir_create_proof, acir_get_honk_verification_key, acir_get_verification_key, acir_prove_ultra_honk, delete_acir_composer, new_acir_composer, CircuitSizes
}, Buffer};
use bn254_blackbox_solver::Bn254BlackBoxSolver;
use flate2::bufread::GzDecoder;
use nargo::ops::execute_program;
use nargo::foreign_calls::DefaultForeignCallExecutor;

/// Get the acir buffer (compressed) from the circuit bytecode
/// 
/// # Arguments
/// 
/// * circuit_bytecode: The circuit bytecode to get the acir buffer from
/// 
/// # Returns
/// 
/// The acir buffer (compressed)
pub fn get_acir_buffer(circuit_bytecode: &str) -> Result<Vec<u8>, String> {
    let acir_buffer = general_purpose::STANDARD
        .decode(circuit_bytecode)
        .map_err(|e| e.to_string())?;
    
    Ok(acir_buffer)
}

/// Uncompress the acir buffer
/// 
/// # Arguments
/// 
/// * acir_buffer: The acir buffer to uncompress
/// 
/// # Returns
/// 
/// The uncompressed acir buffer
pub fn uncompress_acir_buffer(acir_buffer: Vec<u8>) -> Result<Vec<u8>, String> {
    let mut decoder = GzDecoder::new(acir_buffer.as_slice());
    let mut acir_buffer_uncompressed = Vec::<u8>::new();
    decoder
        .read_to_end(&mut acir_buffer_uncompressed)
        .map_err(|e| e.to_string())?;

    Ok(acir_buffer_uncompressed)
}

/// Get the acir buffer (uncompressed) from the circuit bytecode
/// 
/// # Arguments
/// 
/// * circuit_bytecode: The circuit bytecode to get the acir buffer from
/// 
/// # Returns
/// 
/// The acir buffer (uncompressed)
pub fn get_acir_buffer_uncompressed(circuit_bytecode: &str) -> Result<Vec<u8>, String> {
    let acir_buffer = get_acir_buffer(circuit_bytecode)?;
    uncompress_acir_buffer(acir_buffer)
}

/// Get the program from the circuit bytecode
/// 
/// # Arguments
/// 
/// * circuit_bytecode: The circuit bytecode to get the program from
/// 
/// # Returns
/// 
/// The program
pub fn get_program(circuit_bytecode: &str) -> Result<Program<FieldElement>, String> {
    let acir_buffer: Vec<u8> = get_acir_buffer(circuit_bytecode)?;
    Program::deserialize_program(&acir_buffer).map_err(|e| e.to_string())
}