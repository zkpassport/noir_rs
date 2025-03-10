

const BYTECODE: &str = "H4sIAAAAAAAA/62QQQqAMAwErfigpEna5OZXLLb/f4KKLZbiTQdCQg7Dsm66mc9x00O717rhG9ico5cgMOfoMxJu4C2pAEsKioqisnslysoaLVkEQ6aMRYxKFc//ZYQr29L10XfhXv4jB52E+OpMAQAA";


#[cfg(test)]
mod tests {

    use bb_rs::barretenberg_api::acir::{get_circuit_sizes, new_acir_composer};
    use bb_rs::barretenberg_api::common::example_simple_create_and_verify_proof;
    use bb_rs::barretenberg_api::srs::init_srs;
    use tracing::info;
    use crate::barretenberg::srs::netsrs::NetSrs;
    use crate::barretenberg::srs::setup_srs;
    use crate::barretenberg::tests::BYTECODE;
    use crate::{circuit, witness};
    use crate::barretenberg::prove::{prove_ultra_honk, prove_ultra_plonk};
    use crate::barretenberg::verify::{verify_ultra_honk, verify_ultra_plonk};
    use crate::circuit::get_acir_buffer_uncompressed;

    #[test]
    fn test_common_example() {
        let hui = unsafe {
            // The group size required to run the example from Barretenberg
            let subgroup_size = 524289;
            let srs = NetSrs::new(subgroup_size + 1);
            init_srs(&srs.g1_data, srs.num_points, &srs.g2_data);
            example_simple_create_and_verify_proof()
        };

        assert!(hui);
    }

    #[test]
    fn test_acir_get_circuit_size() {
        let (_, constraint_system_buf) = circuit::decode_circuit(BYTECODE).unwrap();
        let circuit_sizes = unsafe {
            get_circuit_sizes(&constraint_system_buf, false)
        };
        assert_eq!(circuit_sizes.total, 22);
        assert_eq!(circuit_sizes.subgroup, 32);
    }

    #[test]
    fn test_prove_and_verify_ultra_plonk() {
        tracing_subscriber::fmt::init();

        // Setup SRS
        setup_srs(BYTECODE, None, false).unwrap();

        // Ultra Plonk
        let initial_witness = witness::from_vec_to_witness_map(vec![5u128, 6u128, 30u128]).unwrap();

        let start = std::time::Instant::now();
        let (proof, vk) = prove_ultra_plonk(BYTECODE, initial_witness, false).unwrap();
        info!("ultra plonk proof generation time: {:?}", start.elapsed());
        let acir_buffer_uncompressed = get_acir_buffer_uncompressed(BYTECODE).unwrap();

        let acir_buffer_uncompressed = get_acir_buffer_uncompressed(BYTECODE).unwrap();


        let circuit_size = unsafe {
            get_circuit_sizes(&acir_buffer_uncompressed, false)
        };


        let mut composer = unsafe{
            new_acir_composer(circuit_size.total)
        };


        let verdict = verify_ultra_plonk(proof, &mut composer).unwrap();
        info!("Plonk verification verdict: {}", verdict);
    }

    #[test]
    fn test_prove_and_verify_ultra_honk() {
        tracing_subscriber::fmt::init();
    
        // Setup SRS
        setup_srs(BYTECODE, None, false).unwrap();
    
        // Ultra Honk
    
        // Get the witness map from the vector of field elements
        // The vector items can be either a FieldElement, an unsigned integer
        // For hex or decimal strings, use from_vec_str_to_witness_map
        let initial_witness = witness::from_vec_to_witness_map(vec![5 as u128, 6 as u128, 30 as u128]).unwrap();
    
        let start = std::time::Instant::now();
        let (proof, vk) = prove_ultra_honk(BYTECODE, initial_witness, false).unwrap();
        info!("ultra honk proof generation time: {:?}", start.elapsed());
    
        let verdict = verify_ultra_honk(proof, vk).unwrap();
        info!("honk proof verification verdict: {}", verdict);
    }

    //The bytecode fails to be interpreted correctly by Barretenberg
    #[test]
    fn test_ultra_honk_keccak() {
        tracing_subscriber::fmt::init();
    
        // Read the JSON manifest of the circuit
        let keccak_circuit_txt = std::fs::read_to_string("circuits/target/keccak.json").unwrap();
        // Parse the JSON manifest into a dictionary
        let keccak_circuit: serde_json::Value = serde_json::from_str(&keccak_circuit_txt).unwrap();
        // Get the bytecode from the dictionary
        let keccak_circuit_bytecode = keccak_circuit["bytecode"].as_str().unwrap();
    
        // Setup SRS
        setup_srs(keccak_circuit_bytecode, None, false).unwrap();
    
        // Ultra Honk
    
        // Get the witness map from the vector of field elements
        // The vector items can be either a FieldElement, an unsigned integer
        // For hex or decimal strings, use from_vec_str_to_witness_map
        let initial_witness = witness::from_vec_to_witness_map(vec![2 as u128, 5 as u128, 10 as u128, 15 as u128, 20 as u128]).unwrap();
    
        let start = std::time::Instant::now();
        let (proof, vk) = prove_ultra_honk(keccak_circuit_bytecode, initial_witness, false).unwrap();
        info!("ultra honk proof generation time: {:?}", start.elapsed());
    
        let verdict = verify_ultra_honk(proof, vk).unwrap();
        info!("honk proof verification verdict: {}", verdict);
    }

    #[test]
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

        let (recursed_proof, recursed_vk) = prove_ultra_honk(recursed_circuit_bytecode, initial_witness, true).unwrap();

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

        let (proof, vk) = prove_ultra_honk(recursive_circuit_bytecode, initial_witness_recursive, true).unwrap();

        let verdict = verify_ultra_honk(proof, vk).unwrap();
        assert!(verdict);
    }

}
