use core::num;

use acir::{
    native_types::{Witness, WitnessMap},
    FieldElement
};
use bb_rs::barretenberg_api::{acir::get_circuit_sizes, common::example_simple_create_and_verify_proof, srs::init_srs};
use noir_rs::{prove::prove_honk, srs::setup_srs, utils::decode_circuit, verify::verify_honk};
use prove::prove;
use tracing::info;
use verify::verify;
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
    let (_, constraint_system_buf) = decode_circuit(String::from(BYTECODE)).unwrap();
    let circuit_sizes = unsafe { 
        get_circuit_sizes(&constraint_system_buf, false) 
    }; 
    assert_eq!(circuit_sizes.exact, 2);
    assert_eq!(circuit_sizes.total, 7);
    assert_eq!(circuit_sizes.subgroup, 8);
}

