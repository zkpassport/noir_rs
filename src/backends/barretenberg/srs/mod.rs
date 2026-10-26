pub mod localsrs;
pub mod netsrs;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::backends::barretenberg::api;
use crate::backends::barretenberg::api::settings_ultra_honk_poseidon2;
use crate::backends::barretenberg::utils::compute_subgroup_size;
use crate::circuit::decode_circuit;

// Number of points the process-global barretenberg SRS was initialized with.
// Since barretenberg 5.x, only the first SRS initialization of the process takes
// effect: later `srs_init` calls are silently ignored by the backend. The lock is
// held across download + init so the recorded size always matches the backend state.
static SRS_NUM_POINTS: Mutex<Option<u32>> = Mutex::new(None);

// G2 is a small fixed group, so we can hardcode it here
const G2: [u8; 128] = [
    1, 24, 196, 213, 184, 55, 188, 194, 188, 137, 181, 179, 152, 181, 151, 78, 159, 89, 68, 7, 59,
    50, 7, 139, 126, 35, 31, 236, 147, 136, 131, 176, 38, 14, 1, 178, 81, 246, 241, 199, 231, 255,
    78, 88, 7, 145, 222, 232, 234, 81, 216, 122, 53, 142, 3, 139, 78, 254, 48, 250, 192, 147, 131,
    193, 34, 254, 189, 163, 192, 192, 99, 42, 86, 71, 91, 66, 20, 229, 97, 94, 17, 230, 221, 63,
    150, 230, 206, 162, 133, 74, 135, 212, 218, 204, 94, 85, 4, 252, 99, 105, 247, 17, 15, 227,
    210, 81, 86, 193, 187, 154, 114, 133, 156, 242, 160, 70, 65, 249, 155, 164, 238, 65, 60, 128,
    218, 106, 95, 228,
];

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Srs {
    pub g1_data: Vec<u8>,
    pub g2_data: Vec<u8>,
    pub num_points: u32,
}

impl Srs {
    pub fn get(self, num_points: u32) -> Srs {
        match self.num_points.cmp(&num_points) {
            std::cmp::Ordering::Equal => self,
            _ => Srs {
                g1_data: self.g1_data[..=(num_points * 64 - 1) as usize].to_vec(),
                g2_data: self.g2_data,
                num_points,
            },
        }
    }
}

pub fn get_srs(subgroup_size: u32, srs_path: Option<&str>) -> Srs {
    match srs_path {
        Some(path) => {
            if path.ends_with(".dat") {
                // Interpret as a .dat file
                let local_srs = localsrs::LocalSrs::from_dat_file(subgroup_size + 1, srs_path);
                local_srs.to_srs()
            } else {
                // Otherwise interpret as a .local file (i.e. a serialized SRS struct)
                let local_srs = localsrs::LocalSrs::new(subgroup_size + 1, srs_path);
                local_srs.to_srs()
            }
        }
        None => {
            let net_srs = netsrs::NetSrs::new(subgroup_size + 1);
            net_srs.to_srs()
        }
    }
}

pub fn setup_srs(circuit_size: u32, srs_path: Option<&str>) -> Result<u32, String> {
    let subgroup_size = compute_subgroup_size(circuit_size);
    let num_points = subgroup_size + 1;

    let mut initialized = SRS_NUM_POINTS.lock().unwrap_or_else(|e| e.into_inner());
    match *initialized {
        // The SRS already covers this circuit size: nothing to do.
        Some(n) if n >= num_points => return Ok(num_points),
        // The backend ignores re-initialization, so a larger SRS is unobtainable
        // at this point: fail loudly instead of letting proving fail later with
        // an obscure backend error.
        Some(n) => {
            return Err(format!(
                "SRS already initialized with {} points, cannot re-initialize with {}: \
                 barretenberg only honors the first SRS initialization of the process. \
                 Set up the SRS once with the largest circuit size you need.",
                n, num_points
            ))
        }
        None => {}
    }

    let srs = get_srs(subgroup_size, srs_path);
    api::srs_init(&srs.g1_data, srs.num_points, &srs.g2_data)?;
    *initialized = Some(srs.num_points);

    Ok(srs.num_points)
}

pub fn setup_srs_from_bytecode(
    circuit_bytecode: &str,
    srs_path: Option<&str>,
    _recursive: bool,
) -> Result<u32, String> {
    let (_, acir_buffer_uncompressed) = decode_circuit(circuit_bytecode)?;
    let settings = settings_ultra_honk_poseidon2();
    let info = api::circuit_stats(&acir_buffer_uncompressed, &settings)?;
    setup_srs(info.num_gates, srs_path)
}
