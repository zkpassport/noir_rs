use std::io::Read;

use acvm::acir::{native_types::{WitnessMap, WitnessStack, Witness}, FieldElement};
use flate2::read::GzDecoder;

/// Convert a vector of field elements to a witness map
/// 
/// # Arguments
/// 
/// * witness_vec: The vector of field elements to convert to a witness map
/// The actual type of the vector items can be either a FieldElement or an unsigned integer
/// 
/// # Returns
/// 
/// The witness map
pub fn from_vec_to_witness_map<T>(witness_vec: Vec<T>) -> Result<WitnessMap<FieldElement>, String>
where
    T: Copy,
    FieldElement: From<T>
{
    let mut witness_map = WitnessMap::new();

    for (i, witness) in witness_vec.iter().enumerate() {
        witness_map.insert(Witness(i as u32), FieldElement::from(*witness));
    }

    Ok(witness_map)
}

/// Convert a vector of strings to a witness map
/// 
/// # Arguments
/// 
/// * witness_vec: The vector of strings to convert to a witness map
/// Each string is expected to be a valid hexadecimal or decimal string
/// 
/// # Returns
/// 
/// The witness map
pub fn from_vec_str_to_witness_map(witness_vec: Vec<&str>) -> Result<WitnessMap<FieldElement>, String> {
    let mut witness_map = WitnessMap::new();

    for (i, witness) in witness_vec.iter().enumerate() {
        witness_map.insert(Witness(i as u32), FieldElement::try_from_str(*witness).unwrap_or_default());
    }

    Ok(witness_map)
}

/// Wrap the witness map into a witness stack
/// 
/// # Arguments
/// 
/// * witness_map: The witness map to wrap into a witness stack
/// 
/// # Returns
/// 
/// The witness stack
pub fn witness_map_to_witness_stack(witness_map: WitnessMap<FieldElement>) -> Result<WitnessStack<FieldElement>, String> {
    let witness_stack = WitnessStack::try_from(witness_map).map_err(|e| e.to_string())?;
    Ok(witness_stack)
}

/// Serialize the witness stack using the format from `NOIR_SERIALIZATION_FORMAT`.
///
/// The result is the raw (uncompressed) serialized witness suitable for passing
/// to barretenberg.
///
/// # Arguments
///
/// * witness_stack: The witness stack to serialize
///
/// # Returns
///
/// The serialized witness stack
pub fn serialize_witness(witness_stack: WitnessStack<FieldElement>) -> Result<Vec<u8>, String> {
    // WitnessStack::serialize() respects NOIR_SERIALIZATION_FORMAT and gzip-compresses
    let compressed = witness_stack.serialize().map_err(|e| e.to_string())?;
    // Decompress to get raw bytes for barretenberg
    let mut decoder = GzDecoder::new(compressed.as_slice());
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf).map_err(|e| e.to_string())?;
    Ok(buf)
}

/// Deserialize the witness stack from a bincode encoded vector
/// 
/// # Arguments
/// 
/// * serialized_witness_stack: The serialized witness stack to deserialize
/// 
/// # Returns
/// 
/// The witness stack
pub fn deserialize_witness(serialized_witness_stack: Vec<u8>) -> Result<WitnessStack<FieldElement>, String> {
    let witness_stack = bincode::deserialize(&serialized_witness_stack).map_err(|e| e.to_string())?;
    Ok(witness_stack)
}