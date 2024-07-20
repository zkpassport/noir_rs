pub mod localsrs;
pub mod netsrs;
use serde::{Deserialize, Serialize};
use flate2::bufread::GzDecoder;
use base64::{engine::general_purpose, Engine};
use std::io::Read;

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


pub fn get_srs(acir_buffer_uncompressed: &[u8], srs_path: Option<&str>) -> Srs {
    let circuit_size = unsafe { bb_rs::barretenberg_api::acir::get_circuit_sizes(&acir_buffer_uncompressed) };
    let log_value = (circuit_size.total as f64).log2().ceil() as u32;
    let subgroup_size = 2u32.pow(log_value);

    match srs_path {
        Some(path) => {
            let local_srs = localsrs::LocalSrs::from_dat_file(subgroup_size + 1, srs_path);
            local_srs.to_srs()
        }
        None => {
            let net_srs = netsrs::NetSrs::new(subgroup_size + 1);
            net_srs.to_srs()
        }
    }
}

pub fn setup_srs(circuit_bytecode: String, srs_path: Option<&str>) ->  Result<u32, String> {
    let acir_buffer = general_purpose::STANDARD
        .decode(circuit_bytecode)
        .map_err(|e| e.to_string())?;
    
    let mut decoder = GzDecoder::new(acir_buffer.as_slice());
    let mut acir_buffer_uncompressed = Vec::<u8>::new();
    decoder
        .read_to_end(&mut acir_buffer_uncompressed)
        .map_err(|e| e.to_string())?;

    let srs = get_srs(&acir_buffer_uncompressed, srs_path);
    unsafe {
        bb_rs::barretenberg_api::srs::init_srs(&srs.g1_data, srs.num_points, &srs.g2_data);
    }
    Ok(srs.num_points)
}