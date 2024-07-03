use std::io::Read;

use acir::{circuit::{self, Circuit, Program}, native_types::{WitnessMap, WitnessStack}};
use base64::{engine::general_purpose, Engine};
use bb_rs::barretenberg_api::{
    acir::{
        acir_create_proof, acir_get_verification_key, acir_init_proving_key, delete_acir_composer,
        get_circuit_sizes, new_acir_composer, CircuitSizes, acir_create_circuit
    },
    srs::init_srs,
};
use bn254_blackbox_solver::Bn254BlackBoxSolver;
use flate2::bufread::GzDecoder;
use nargo::ops::execute::execute_circuit;
use serde::{Deserialize, Serialize};

use crate::srs::netsrs::NetSrs;

pub fn prove(
    circuit_bytecode: String,
    initial_witness: WitnessMap,
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let acir_buffer = general_purpose::STANDARD
        .decode(circuit_bytecode)
        .map_err(|e| e.to_string())?;
    
    let program = Program::deserialize_program(&acir_buffer).map_err(|e| e.to_string())?;

    let mut decoder = GzDecoder::new(acir_buffer.as_slice());
    let mut acir_buffer_uncompressed = Vec::<u8>::new();
    decoder
        .read_to_end(&mut acir_buffer_uncompressed)
        .map_err(|e| e.to_string())?;

    let blackbox_solver = Bn254BlackBoxSolver::new();

    let solved_witness =
        execute_circuit(&program, initial_witness, &blackbox_solver).map_err(|e| e.to_string())?;
    let witness_stack = WitnessStack::try_from(solved_witness).map_err(|e| e.to_string())?;
    let serialized_solved_witness =
        bincode::serialize(&witness_stack).map_err(|e| e.to_string())?;
        
    let circuit_size = unsafe { get_circuit_sizes(&acir_buffer_uncompressed) };
    let log_value = (circuit_size.total as f64).log2().ceil() as u32;
    let subgroup_size = 2u32.pow(log_value);

    let srs = NetSrs::new(subgroup_size + 1);

    Ok(unsafe {
        init_srs(&srs.g1_data, srs.num_points, &srs.g2_data);
        let mut acir_ptr = new_acir_composer(subgroup_size);
        acir_init_proving_key(&mut acir_ptr, &acir_buffer_uncompressed);
        let result = (
            acir_create_proof(
                &mut acir_ptr,
                &acir_buffer_uncompressed,
                &serialized_solved_witness,
            ),
            acir_get_verification_key(&mut acir_ptr),
        );
        delete_acir_composer(acir_ptr);
        result
    })
}
