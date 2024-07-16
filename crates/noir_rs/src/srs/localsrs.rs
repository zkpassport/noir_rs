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

    pub fn from_dat_file(num_points: u32, path: Option<&str>) -> Self {
        let file = fs::read(path.unwrap_or(SRS_DEFAULT_PATH)).unwrap();

        const G1_START: u32 = 28;
        let g1_end: u32 = G1_START + num_points * 64 - 1;

        const G2_START: usize = 28 + 5040001 * 64;
        const G2_END: usize = G2_START + 128 - 1;

        let srs: Srs = Srs {
            num_points: num_points,
            g1_data: file[G1_START as usize..=g1_end as usize].to_vec(),
            g2_data: file[G2_START..=G2_END].to_vec(),
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
