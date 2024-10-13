mod utils;
use crate::utils::{dataset, rng, time};

pub fn main() {
    println!(
        "Hello, world!, rand int is: {}",
        // rng::Rng::new(42).random_u32()
        rng::Rng::new(time::unixtime()).random_u32()
    );

    // println!("gen data: {:?}", _gen_data!());
    println!(
        "gen data: {:?}",
        _gen_data!(rng::Rng::new(time::unixtime()), 10)
    );
}
