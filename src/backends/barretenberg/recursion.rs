// Recursion support is currently disabled pending updates to the barretenberg API.
//
// The barretenberg_rs API provides VK-as-fields through the CircuitComputeVk response,
// which returns both `bytes` and `fields` representations. VK conversion to field
// elements is also available via the VkAsFields command.
//
// To re-enable recursive proving, use:
//   - api::circuit_compute_vk() for VK fields
//   - api::vk_as_fields() for converting VK bytes to field elements
//   - The proof response already contains field-element representations

/*
use crate::backends::barretenberg::api;

pub fn generate_recursive_honk_proof_artifacts(
    proof_bytes: Vec<u8>,
    vk_bytes: Vec<u8>
) -> Result<(Vec<Vec<u8>>, Vec<Vec<u8>>), String> {
    let vk_fields = api::vk_as_fields(&vk_bytes)?;

    // The proof is already in field-element format from circuit_prove response
    let proof_fields = api::proof_bytes_to_fields(&proof_bytes);

    Ok((proof_fields, vk_fields))
}
*/
