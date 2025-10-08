use tracing::info;
use bb_rs::barretenberg_api::{acir::{get_circuit_sizes, acir_get_slow_low_memory}, srs::init_srs};
use crate::backends::barretenberg::{srs::{setup_srs_from_bytecode, setup_srs, netsrs::NetSrs}, verify::{verify_ultra_honk, verify_ultra_honk_keccak, get_ultra_honk_verification_key, get_ultra_honk_keccak_verification_key}, prove::{prove_ultra_honk, prove_ultra_honk_keccak}, utils::compute_subgroup_size};
use acvm::acir::{FieldElement, native_types::{Witness, WitnessMap}};
use crate::{witness, circuit};
use serde_json;

const BYTECODE: &str = "H4sIAAAAAAAA/42NsQmAMBBF74KDWGqnOIIIVmJpYyHYWChiZ5kRxAWcQnScdJY29gZNSAgp8or7x93/fIQfT2jfdAPhiqCQuw9OoBxmLmqLicVbeJTZTmlVB8mVz+e4pOxZb/4n7h2fVy9Ey93kBZmTjiLsAAAA";

#[test]
fn test_acir_get_circuit_size() {
    let (_, constraint_system_buf) = circuit::decode_circuit(BYTECODE).unwrap();
    let circuit_sizes = unsafe { 
        get_circuit_sizes(&constraint_system_buf, false) 
    }; 
    assert_eq!(circuit_sizes.total, 56);
    assert_eq!(circuit_sizes.subgroup, 64);
}

#[test]
fn test_prove_and_verify_ultra_honk() {
    let _ = tracing_subscriber::fmt::try_init();

    // Setup SRS
    //setup_srs_from_bytecode(BYTECODE, None, false).unwrap();
    setup_srs(500, None).unwrap();

    // Ultra Honk

    // Get the witness map from the vector of field elements
    // The vector items can be either a FieldElement, an unsigned integer
    // For hex or decimal strings, use from_vec_str_to_witness_map
    let initial_witness = witness::from_vec_to_witness_map(vec![5 as u128, 6 as u128, 30 as u128]).unwrap();

    let start = std::time::Instant::now();
    let vk = get_ultra_honk_verification_key(BYTECODE, false, None).unwrap();
    assert_eq!(acir_get_slow_low_memory(), false);

    let proof = prove_ultra_honk(BYTECODE, initial_witness, vk.clone(), false, None).unwrap();
    info!("ultra honk proof generation time: {:?}", start.elapsed());
    assert_eq!(acir_get_slow_low_memory(), false);

    let verdict = verify_ultra_honk(proof, vk).unwrap();
    info!("honk proof verification verdict: {}", verdict);
}

#[test]
fn test_ultra_honk_keccak() {
    let _ = tracing_subscriber::fmt::try_init();

    // Read the JSON manifest of the circuit 
    let keccak_circuit_txt = std::fs::read_to_string("circuits/target/keccak.json").unwrap();
    // Parse the JSON manifest into a dictionary
    let keccak_circuit: serde_json::Value = serde_json::from_str(&keccak_circuit_txt).unwrap();
    // Get the bytecode from the dictionary
    let keccak_circuit_bytecode = keccak_circuit["bytecode"].as_str().unwrap();
    
    // Setup SRS
    setup_srs_from_bytecode(keccak_circuit_bytecode, None, false).unwrap();

    // Ultra Honk

    // Get the witness map from the vector of field elements
    // The vector items can be either a FieldElement, an unsigned integer
    // For hex or decimal strings, use from_vec_str_to_witness_map
    let initial_witness = witness::from_vec_to_witness_map(vec![2 as u128, 5 as u128, 10 as u128, 15 as u128, 20 as u128]).unwrap();

    let start = std::time::Instant::now();
    let vk = get_ultra_honk_keccak_verification_key(keccak_circuit_bytecode, false, false, None).unwrap();
    assert_eq!(acir_get_slow_low_memory(), false);
    
    let proof = prove_ultra_honk_keccak(keccak_circuit_bytecode, initial_witness, vk.clone(), false, false, None).unwrap();
    info!("ultra honk proof generation time: {:?}", start.elapsed());
    assert_eq!(acir_get_slow_low_memory(), false);

    let verdict = verify_ultra_honk_keccak(proof, vk, false).unwrap();
    info!("honk proof verification verdict: {}", verdict);
}

#[test]
fn test_ultra_honk_low_memory() {
    let _ = tracing_subscriber::fmt::try_init();

    // Read the JSON manifest of the circuit 
    let circuit_txt = std::fs::read_to_string("circuits/target/keccak_large.json").unwrap();
    // Parse the JSON manifest into a dictionary
    let circuit: serde_json::Value = serde_json::from_str(&circuit_txt).unwrap();
    // Get the bytecode from the dictionary
    let circuit_bytecode = circuit["bytecode"].as_str().unwrap();
    
    // Setup SRS
    setup_srs_from_bytecode(circuit_bytecode, None, false).unwrap();

    // Ultra Honk

    // Get the witness map from the vector of field elements
    // The vector items can be either a FieldElement, an unsigned integer
    // For hex or decimal strings, use from_vec_str_to_witness_map
    let initial_witness = witness::from_vec_to_witness_map(vec![2 as u128, 5 as u128, 10 as u128, 15 as u128, 20 as u128]).unwrap();

    let start = std::time::Instant::now();
    let vk = get_ultra_honk_verification_key(circuit_bytecode, true, None).unwrap();
    assert_eq!(acir_get_slow_low_memory(), true);
    
    // Low memory mode with a limit of 5GB of storage use (fallbacks on using the RAM for the rest)
    let proof = prove_ultra_honk(circuit_bytecode, initial_witness, vk.clone(), true, Some(5 * 1024 * 1024 * 1024)).unwrap();
    info!("ultra honk proof generation time: {:?}", start.elapsed());
    assert_eq!(acir_get_slow_low_memory(), true);

    let verdict = verify_ultra_honk(proof, vk).unwrap();
    info!("honk proof verification verdict: {}", verdict);
}

#[test]
fn test_srs_setup_from_bytecode() {
    let _ = tracing_subscriber::fmt::try_init();

    let start = std::time::Instant::now();
    let srs = setup_srs_from_bytecode(BYTECODE, None, false).unwrap();
    info!("srs setup time: {:?}", start.elapsed());
    // 2^6 + 1 = 33
    assert_eq!(srs, 65);
}

#[test]
fn test_srs_setup_from_circuit_size() {
    let _ = tracing_subscriber::fmt::try_init();

    let start = std::time::Instant::now();
    let circuit_size = 22;
    let srs = setup_srs(circuit_size, None).unwrap();
    info!("srs setup time: {:?}", start.elapsed());
    // 2^5 + 1 = 33
    assert_eq!(srs, 33);
}

#[test]
fn test_compute_subgroup_size() {
    let mut subgroup_size = compute_subgroup_size(22);
    assert_eq!(subgroup_size, 32);

    subgroup_size = compute_subgroup_size(50);
    assert_eq!(subgroup_size, 64);

    subgroup_size = compute_subgroup_size(100);
    assert_eq!(subgroup_size, 128);

    subgroup_size = compute_subgroup_size(1000);
    assert_eq!(subgroup_size, 1024);

    subgroup_size = compute_subgroup_size(10000);
    assert_eq!(subgroup_size, 16384);

    subgroup_size = compute_subgroup_size(100000);
    assert_eq!(subgroup_size, 131072);
    
    subgroup_size = compute_subgroup_size(200000);
    assert_eq!(subgroup_size, 262144);

    subgroup_size = compute_subgroup_size(500000);
    assert_eq!(subgroup_size, 524288);

    subgroup_size = compute_subgroup_size(1000000);
    assert_eq!(subgroup_size, 1048576);    
}

/*#[test]
fn test_ultra_honk_recursive_proving() {
    // Read the JSON manifest of the circuit 
    let recursed_circuit_txt = std::fs::read_to_string("circuits/target/recursed.json").unwrap();
    // Parse the JSON manifest into a dictionary
    let recursed_circuit: serde_json::Value = serde_json::from_str(&recursed_circuit_txt).unwrap();
    // Get the bytecode from the dictionary
    let recursed_circuit_bytecode = recursed_circuit["bytecode"].as_str().unwrap();

    setup_srs(String::from(recursed_circuit_bytecode), None, true).unwrap();

    let mut initial_witness = WitnessMap::new();
    // x
    initial_witness.insert(Witness(0), FieldElement::from(5u128));
    // y
    initial_witness.insert(Witness(1), FieldElement::from(25u128));

    let (recursed_proof, recursed_vk) = prove_ultra_honk(recursed_circuit_bytecode, initial_witness, true, None).unwrap();

    let (proof_as_fields, vk_as_fields, key_hash) = recursion::generate_recursive_honk_proof_artifacts(recursed_proof, recursed_vk).unwrap();

    //println!("proof: {:?}", proof_as_fields);
    //println!("vk: {:?}", vk_as_fields);
    //println!("key_hash: {:?}", key_hash);
    
    assert_eq!(proof_as_fields.len(), 463);
    assert_eq!(vk_as_fields.len(), 128);
    //assert_eq!(key_hash, "0x25240793a378438025d0dbe8a4e197c93ec663864a5c9b01699199423dab1008");

    // Read the JSON manifest of the circuit 
    let recursive_circuit_txt = std::fs::read_to_string("circuits/target/recursive.json").unwrap();
    // Parse the JSON manifest into a dictionary
    let recursive_circuit: serde_json::Value = serde_json::from_str(&recursive_circuit_txt).unwrap();
    // Get the bytecode from the dictionary
    let recursive_circuit_bytecode = recursive_circuit["bytecode"].as_str().unwrap();
    println!("recursive_circuit_bytecode: {:?}", recursive_circuit_bytecode);

    // IMPORTANT: Likely to run into a timeout for the net srs, replace None with a path to a local srs file
    // before running this test
    setup_srs(String::from(recursive_circuit_bytecode), None, true).unwrap();

    let mut initial_witness_recursive = WitnessMap::new();
    let mut index = 0;
    // Verification key
    vk_as_fields.iter().for_each(|v| {
        initial_witness_recursive.insert(Witness(index), FieldElement::try_from_str(v).unwrap());
        index += 1;
    });
    // Proof
    proof_as_fields.iter().for_each(|v| {
        initial_witness_recursive.insert(Witness(index), FieldElement::try_from_str(v).unwrap());
        index += 1;
    });
    // Public inputs
    initial_witness_recursive.insert(Witness(index), FieldElement::from(25u128));
    index += 1;
    // Key hash
    initial_witness_recursive.insert(Witness(index), FieldElement::try_from_str(&key_hash).unwrap());

    let (proof, vk) = prove_ultra_honk(recursive_circuit_bytecode, initial_witness_recursive, true, None).unwrap();

    let verdict = verify_ultra_honk(proof, vk).unwrap();
    assert!(verdict);
}*/
