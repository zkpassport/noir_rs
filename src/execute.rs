use acir::{native_types::{WitnessMap, WitnessStack}, FieldElement};
use bn254_blackbox_solver::Bn254BlackBoxSolver;
use nargo::ops::execute_program;
use nargo::foreign_calls::DefaultForeignCallExecutor;

use crate::circuit::get_program;

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
    let program = get_program(circuit_bytecode)?;

    let blackbox_solver = Bn254BlackBoxSolver::default();
    let mut foreign_call_executor = DefaultForeignCallExecutor::default();

    let solved_witness =
        execute_program(&program, initial_witness, &blackbox_solver, &mut foreign_call_executor).map_err(|e| e.to_string())?;

    Ok(solved_witness)
}