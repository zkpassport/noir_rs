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

use crate::circuit::{get_acir_buffer_uncompressed, get_program};

/// Execute the circuit and return the serialized solved witness stack
/// 
/// # Arguments
/// 
/// * circuit_bytecode: The circuit bytecode to execute
/// * initial_witness: The initial witness to use for the execution
/// 
/// # Returns
/// 
/// The Witness Stack
pub fn execute(circuit_bytecode: &str, initial_witness: WitnessMap<FieldElement>) -> Result<WitnessStack<FieldElement>, String> {
    let acir_buffer_uncompressed: Vec<u8> = get_acir_buffer_uncompressed(circuit_bytecode)?;

    let program = get_program(circuit_bytecode)?;

    let blackbox_solver = Bn254BlackBoxSolver::default();
    let mut foreign_call_executor = DefaultForeignCallExecutor::default();

    let solved_witness =
        execute_program(&program, initial_witness, &blackbox_solver, &mut foreign_call_executor).map_err(|e| e.to_string())?;

    Ok(solved_witness)
}