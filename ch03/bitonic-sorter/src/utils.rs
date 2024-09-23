use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    rng.sample_iter(&Standard).take(n).collect()
}