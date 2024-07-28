use core::num;

use acir::{
    native_types::{Witness, WitnessMap},
    FieldElement
};
use noir_rs::{prove::prove_honk, srs::setup_srs, verify::verify_honk};
use prove::prove;
use tracing::info;
use verify::verify;
use bb_rs::barretenberg_api::{
    srs::init_srs,
    common::example_simple_create_and_verify_proof,
    acir::get_circuit_sizes
};

pub mod prove;
pub mod srs;
pub mod verify;
pub mod utils;

const BYTECODE: &str = "H4sIAAAAAAAA/62QQQ6AMAgErfFBUKCFm1+xsf3/E9TYxka96SQEwmGyWTecjPu44aLdc93wDWzOu5cgMOfoMxIu4C2pAEsKioqisnolysoaLVkEQ6aMRYxKFY//ZYQj29T10XfhXv4PNvD4VlxNAQAA";

fn main() {
    tracing_subscriber::fmt::init();

    // Witness for a circuit with two inputs a and b and public input res = a * b
    let mut initial_witness = WitnessMap::new();
    // a
    initial_witness.insert(Witness(0), FieldElement::from(3u128));
    // b
    initial_witness.insert(Witness(1), FieldElement::from(5u128));
    // res = a * b
    initial_witness.insert(Witness(2), FieldElement::from(15u128));

    // Setup SRS
    let num_points = setup_srs(String::from(BYTECODE), None).unwrap();

    // Ultra Plonk
    let mut start = std::time::Instant::now();
    let (proof, vk) = prove(String::from(BYTECODE), initial_witness, num_points.clone()).unwrap();
    info!("ultraplonk proof generation time: {:?}", start.elapsed());

    let verdict = verify(proof, vk, num_points).unwrap();
    info!("ultraplonk proof verification verdict: {}", verdict);

    // Honk
    let mut initial_witness_honk = WitnessMap::new();
    // a
    initial_witness_honk.insert(Witness(0), FieldElement::from(5u128));
    // b
    initial_witness_honk.insert(Witness(1), FieldElement::from(6u128));
    // res = a * b
    initial_witness_honk.insert(Witness(2), FieldElement::from(30u128));

    start = std::time::Instant::now();
    let (proof, vk) = prove_honk(String::from(BYTECODE), initial_witness_honk).unwrap();
    info!("honk proof generation time: {:?}", start.elapsed());

    let verdict = verify_honk(proof, vk).unwrap();
    info!("honk proof verification verdict: {}", verdict);
}

#[test]
fn test_common_example() {
    assert!(unsafe { 
        // The group size required to run the example from Barretenberg
        let subgroup_size = 524289;
        let srs = srs::netsrs::NetSrs::new(subgroup_size + 1);
        init_srs(&srs.g1_data, srs.num_points, &srs.g2_data);
        example_simple_create_and_verify_proof() 
    });
}


#[test]
fn test_acir_get_circuit_size() {
    // The uncompressed acir buffer for the circuit represented by the bytecode above
    let constraint_system_buf: [u8; 333] = [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 49, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 51, 48, 54, 52, 52, 101, 55, 50, 101, 49, 51, 49, 97, 48, 50, 57, 98, 56, 53, 48, 52, 53, 98, 54, 56, 49, 56, 49, 53, 56, 53, 100, 50, 56, 51, 51, 101, 56, 52, 56, 55, 57, 98, 57, 55, 48, 57, 49, 52, 51, 101, 49, 102, 53, 57, 51, 102, 48, 48, 48, 48, 48, 48, 48, 2, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 1, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let circuit_sizes = unsafe { 
        get_circuit_sizes(&constraint_system_buf) 
    }; 
    assert_eq!(circuit_sizes.exact, 13);
    assert_eq!(circuit_sizes.total, 18);
    assert_eq!(circuit_sizes.subgroup, 32);
}

