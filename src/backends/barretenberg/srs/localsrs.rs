use crate::barretenberg::srs::G2;

use super::Srs;
use std::fs;

pub struct LocalSrs(pub Srs);

const SRS_DEFAULT_PATH: &str = "srs.local";

impl LocalSrs {
    pub fn new(num_points: u32, path: Option<&str>) -> Self {
        let file = fs::read(path.unwrap_or(SRS_DEFAULT_PATH)).unwrap();
        let srs: Srs = bincode::deserialize(&file).unwrap();
        LocalSrs(srs.get(num_points))
    }

    /**
     * Create a new LocalSrs from a .dat file of the SRS
     * Since G2 is the same every time it is hardcoded and
     * not read from the file. This way the part of the file beyond
     * the G1 data is not read and can be ignored when storing it.
     * @param num_points The number of points for G1
     * @param path The path to the .dat file
     */
    pub fn from_dat_file(num_points: u32, path: Option<&str>) -> Self {
        let file = fs::read(path.unwrap_or(SRS_DEFAULT_PATH)).unwrap();
        let g1_end: u32 = num_points * 64 - 1;

        let srs: Srs = Srs {
            num_points: num_points,
            g1_data: file[0..=g1_end as usize].to_vec(),
            g2_data: G2.to_vec(),
        };

        LocalSrs(srs.get(num_points))
    }

    pub fn save(&self, path: Option<&str>) {
        fs::write(
            path.unwrap_or(SRS_DEFAULT_PATH),
            bincode::serialize(&self.0).unwrap(),
        )
        .unwrap();
    }

    pub fn to_srs(self) -> Srs {
        self.0
    }
}
