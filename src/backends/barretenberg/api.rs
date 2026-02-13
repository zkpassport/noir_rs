use std::sync::Once;

use barretenberg_rs::{
    BarretenbergApi,
    backends::FfiBackend,
    generated_types::{
        CircuitInput, CircuitInputNoVK, ProofSystemSettings,
        CircuitProveResponse, CircuitComputeVkResponse, CircuitInfoResponse,
    },
};

pub const FIELD_ELEMENT_SIZE: usize = 32;

static INIT_FORMAT: Once = Once::new();

/// Ensure the Noir serialization format is set to msgpack-compact,
/// which is required by the current version of barretenberg.
pub fn ensure_msgpack_format() {
    INIT_FORMAT.call_once(|| {
        std::env::set_var("NOIR_SERIALIZATION_FORMAT", "msgpack-compact");
    });
}

fn get_api() -> BarretenbergApi<FfiBackend> {
    ensure_msgpack_format();
    let backend = FfiBackend::new().expect("Failed to initialize FfiBackend");
    BarretenbergApi::new(backend)
}

pub fn settings_ultra_honk_poseidon2() -> ProofSystemSettings {
    ProofSystemSettings {
        ipa_accumulation: false,
        oracle_hash_type: "poseidon2".to_string(),
        disable_zk: false,
        optimized_solidity_verifier: false,
    }
}

pub fn settings_ultra_honk_keccak(disable_zk: bool) -> ProofSystemSettings {
    ProofSystemSettings {
        ipa_accumulation: false,
        oracle_hash_type: "keccak".to_string(),
        disable_zk,
        optimized_solidity_verifier: false,
    }
}

pub fn proof_fields_to_bytes(proof_fields: &[Vec<u8>]) -> Vec<u8> {
    proof_fields.iter().flat_map(|f| f.iter().copied()).collect()
}

pub fn proof_bytes_to_fields(proof_bytes: &[u8]) -> Vec<Vec<u8>> {
    proof_bytes
        .chunks(FIELD_ELEMENT_SIZE)
        .map(|chunk| chunk.to_vec())
        .collect()
}

pub fn circuit_prove(
    acir_buffer: &[u8],
    witness: &[u8],
    verification_key: &[u8],
    settings: &ProofSystemSettings,
) -> Result<CircuitProveResponse, String> {
    let circuit = CircuitInput {
        name: String::new(),
        bytecode: acir_buffer.to_vec(),
        verification_key: verification_key.to_vec(),
    };
    let mut api = get_api();
    api.circuit_prove(circuit, witness, settings.clone())
        .map_err(|e| format!("circuit_prove failed: {}", e))
}

pub fn circuit_compute_vk(
    acir_buffer: &[u8],
    settings: &ProofSystemSettings,
) -> Result<CircuitComputeVkResponse, String> {
    let circuit = CircuitInputNoVK {
        name: String::new(),
        bytecode: acir_buffer.to_vec(),
    };
    let mut api = get_api();
    api.circuit_compute_vk(circuit, settings.clone())
        .map_err(|e| format!("circuit_compute_vk failed: {}", e))
}

pub fn circuit_verify(
    verification_key: &[u8],
    public_inputs: Vec<Vec<u8>>,
    proof: Vec<Vec<u8>>,
    settings: &ProofSystemSettings,
) -> Result<bool, String> {
    let mut api = get_api();
    let response = api
        .circuit_verify(verification_key, public_inputs, proof, settings.clone())
        .map_err(|e| format!("circuit_verify failed: {}", e))?;
    Ok(response.verified)
}

pub fn circuit_stats(
    acir_buffer: &[u8],
    settings: &ProofSystemSettings,
) -> Result<CircuitInfoResponse, String> {
    let circuit = CircuitInput {
        name: String::new(),
        bytecode: acir_buffer.to_vec(),
        verification_key: vec![],
    };
    let mut api = get_api();
    api.circuit_stats(circuit, false, settings.clone())
        .map_err(|e| format!("circuit_stats failed: {}", e))
}

pub fn srs_init(
    g1_data: &[u8],
    num_points: u32,
    g2_data: &[u8],
) -> Result<(), String> {
    let mut api = get_api();
    api.srs_init_srs(g1_data, num_points, g2_data)
        .map_err(|e| format!("srs_init failed: {}", e))?;
    Ok(())
}

// Direct FFI access to barretenberg global memory configuration variables.
// These are file-scope C++ globals in barretenberg/polynomials/backing_memory.cpp.
extern "C" {
    static mut slow_low_memory: bool;
    static mut storage_budget: usize;
}

/// Configure barretenberg's low memory mode.
///
/// When enabled, barretenberg uses file-backed memory for polynomial storage,
/// which significantly reduces RAM usage at the cost of slower proving (~2x).
///
/// # Arguments
///
/// * `enabled` - Whether to enable low memory mode
/// * `max_storage_usage` - Optional storage budget in bytes for file-backed memory
///
/// # Safety
///
/// Sets global C++ variables via FFI. Must be called before proving/vk operations.
pub fn configure_memory(enabled: bool, max_storage_usage: Option<u64>) {
    // Set environment variables (for any code paths that re-read them)
    if enabled {
        std::env::set_var("BB_SLOW_LOW_MEMORY", "1");
    } else {
        std::env::set_var("BB_SLOW_LOW_MEMORY", "0");
    }
    if let Some(budget) = max_storage_usage {
        std::env::set_var("BB_STORAGE_BUDGET", format!("{}", budget));
    }
    // Also write the globals directly via FFI (belt-and-suspenders)
    unsafe {
        slow_low_memory = enabled;
        if let Some(budget) = max_storage_usage {
            storage_budget = budget as usize;
        }
    }
}
