pub mod localsrs;
pub mod netsrs;
use serde::{Deserialize, Serialize};
use flate2::bufread::GzDecoder;
use base64::{engine::general_purpose, Engine};
use std::io::Read;

use crate::utils::{decode_circuit, get_subgroup_size};

// G2 is a small fixed group, so we can hardcode it here
const G2: [u8; 128] = [126, 35, 31, 236, 147, 136, 131, 176, 159, 89, 68, 7, 59, 50, 7, 139, 188, 137, 181, 179, 152, 181, 151, 78, 1, 24, 196, 213, 184, 55, 188, 194, 78, 254, 48, 250, 192, 147, 131, 193, 234, 81, 216, 122, 53, 142, 3, 139, 231, 255, 78, 88, 7, 145, 222, 232, 38, 14, 1, 178, 81, 246, 241, 199, 133, 74, 135, 212, 218, 204, 94, 85, 17, 230, 221, 63, 150, 230, 206, 162, 86, 71, 91, 66, 20, 229, 97, 94, 34, 254, 189, 163, 192, 192, 99, 42, 238, 65, 60, 128, 218, 106, 95, 228, 156, 242, 160, 70, 65, 249, 155, 164, 210, 81, 86, 193, 187, 154, 114, 133, 4, 252, 99, 105, 247, 17, 15, 227];

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
                g1_data: self.g1_data[..(num_points * 64 - 1) as usize].to_vec(),
                g2_data: self.g2_data,
                num_points: num_points,
            },
        }
    }
}


pub fn get_srs(circuit_bytecode: String, srs_path: Option<&str>) -> Srs {
    let subgroup_size = get_subgroup_size(circuit_bytecode);

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

pub fn setup_srs(circuit_bytecode: String, srs_path: Option<&str>) ->  Result<u32, String> {
    let srs = get_srs(circuit_bytecode, srs_path);
    unsafe {
        bb_rs::barretenberg_api::srs::init_srs(&srs.g1_data, srs.num_points, &srs.g2_data);
    }
    Ok(srs.num_points)
}