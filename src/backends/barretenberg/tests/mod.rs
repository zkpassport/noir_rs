use tracing::info;
use crate::backends::barretenberg::{
    api::{self, settings_ultra_honk_poseidon2},
    srs::{setup_srs_from_bytecode, setup_srs},
    verify::{
        verify_ultra_honk, verify_ultra_honk_keccak,
        get_ultra_honk_verification_key, get_ultra_honk_keccak_verification_key,
    },
    prove::{prove_ultra_honk, prove_ultra_honk_keccak},
    utils::compute_subgroup_size,
};
use crate::{witness, circuit};

#[test]
fn test_circuit_stats() {
    // Read the JSON manifest of the circuit
    let product_circuit_txt = std::fs::read_to_string("circuits/target/product.json").unwrap();
    let product_circuit: serde_json::Value = serde_json::from_str(&product_circuit_txt).unwrap();
    let product_circuit_bytecode = product_circuit["bytecode"].as_str().unwrap();

    let (_, constraint_system_buf) = circuit::decode_circuit(product_circuit_bytecode).unwrap();
    let settings = settings_ultra_honk_poseidon2();
    let info = api::circuit_stats(&constraint_system_buf, &settings).unwrap();
    assert_eq!(info.num_gates, 36);
    assert_eq!(info.num_gates_dyadic, 64);
}

#[test]
fn test_prove_and_verify_ultra_honk() {
    let _ = tracing_subscriber::fmt::try_init();

    // Read the JSON manifest of the circuit
    let product_circuit_txt = std::fs::read_to_string("circuits/target/product.json").unwrap();
    let product_circuit: serde_json::Value = serde_json::from_str(&product_circuit_txt).unwrap();
    let product_circuit_bytecode = product_circuit["bytecode"].as_str().unwrap();

    // Setup SRS
    setup_srs(512, None).unwrap();

    // Get the witness map from the vector of field elements
    let initial_witness = witness::from_vec_to_witness_map(vec![5_u128, 6_u128, 30_u128]).unwrap();

    let start = std::time::Instant::now();
    let vk = get_ultra_honk_verification_key(product_circuit_bytecode, false, None).unwrap();

    let proof = prove_ultra_honk(product_circuit_bytecode, initial_witness, vk.clone(), false, None).unwrap();
    info!("ultra honk proof generation time: {:?}", start.elapsed());

    let verdict = verify_ultra_honk(proof, vk).unwrap();
    info!("honk proof verification verdict: {}", verdict);
    assert!(verdict);
}

#[test]
fn test_ultra_honk_keccak() {
    let _ = tracing_subscriber::fmt::try_init();

    // Read the JSON manifest of the circuit
    let keccak_circuit_txt = std::fs::read_to_string("circuits/target/keccak.json").unwrap();
    let keccak_circuit: serde_json::Value = serde_json::from_str(&keccak_circuit_txt).unwrap();
    let keccak_circuit_bytecode = keccak_circuit["bytecode"].as_str().unwrap();

    // Setup SRS
    setup_srs_from_bytecode(keccak_circuit_bytecode, None, false).unwrap();

    // Get the witness map from the vector of field elements
    let initial_witness = witness::from_vec_to_witness_map(vec![2_u128, 5_u128, 10_u128, 15_u128, 20_u128]).unwrap();

    let start = std::time::Instant::now();
    let vk = get_ultra_honk_keccak_verification_key(keccak_circuit_bytecode, false, false, None).unwrap();

    let proof = prove_ultra_honk_keccak(keccak_circuit_bytecode, initial_witness, vk.clone(), false, false, None).unwrap();
    info!("ultra honk keccak proof generation time: {:?}", start.elapsed());

    let verdict = verify_ultra_honk_keccak(proof, vk, false).unwrap();
    info!("honk keccak proof verification verdict: {}", verdict);
    assert!(verdict);
}

#[test]
#[ignore]
fn test_ultra_honk_low_memory() {
    let _ = tracing_subscriber::fmt::try_init();

    // Read the JSON manifest of the circuit
    let circuit_txt = std::fs::read_to_string("circuits/target/keccak_large.json").unwrap();
    let circuit: serde_json::Value = serde_json::from_str(&circuit_txt).unwrap();
    let circuit_bytecode = circuit["bytecode"].as_str().unwrap();

    // Setup SRS
    setup_srs_from_bytecode(circuit_bytecode, None, false).unwrap();

    // Get the witness map from the vector of field elements
    let initial_witness = witness::from_vec_to_witness_map(vec![2_u128, 5_u128, 10_u128, 15_u128, 20_u128]).unwrap();

    let start = std::time::Instant::now();
    let vk = get_ultra_honk_verification_key(circuit_bytecode, true, None).unwrap();

    // Low memory mode with a limit of 5GB of storage use (falls back to RAM for the rest)
    let proof = prove_ultra_honk(circuit_bytecode, initial_witness, vk.clone(), true, Some(5 * 1024 * 1024 * 1024)).unwrap();
    info!("ultra honk low memory proof generation time: {:?}", start.elapsed());

    let verdict = verify_ultra_honk(proof, vk).unwrap();
    info!("honk low memory proof verification verdict: {}", verdict);
    assert!(verdict);
}

#[test]
fn test_srs_setup_from_bytecode() {
    let _ = tracing_subscriber::fmt::try_init();
    // Read the JSON manifest of the circuit
    let product_circuit_txt = std::fs::read_to_string("circuits/target/product.json").unwrap();
    let product_circuit: serde_json::Value = serde_json::from_str(&product_circuit_txt).unwrap();
    let product_circuit_bytecode = product_circuit["bytecode"].as_str().unwrap();

    let start = std::time::Instant::now();
    let srs = setup_srs_from_bytecode(product_circuit_bytecode, None, false).unwrap();
    info!("srs setup time: {:?}", start.elapsed());
    // 2^6 + 1 = 65
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
