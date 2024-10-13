use super::Rng;

pub fn gen_data_yinyang(
    rng: Rng,
    n: u32,
    r_small: f32,
    r_big: f32,
) -> (
    Vec<(f32, f32, i32)>,
    Vec<(f32, f32, i32)>,
    Vec<(f32, f32, i32)>,
) {
    let mut pts = vec![];

    let tr_size = (n as f32 * 0.8) as usize;
    let val_size = (n as f32 * 0.1) as usize;
    (
        pts[..tr_size].into(),
        pts[tr_size..tr_size + val_size].to_vec(),
        pts[tr_size + val_size..].to_vec(),
    )
}

pub fn gen_data(
    mut rng: Rng,
    n: u32,
) -> (
    Vec<(f32, f32, i32)>,
    Vec<(f32, f32, i32)>,
    Vec<(f32, f32, i32)>,
) {
    let mut pts = vec![];
    for _ in 0..n {
        let x = rng.uniform(Some(-2.0), Some(2.0));
        let y = rng.uniform(Some(-2.0), Some(2.0));
        let label = match (x, y) {
            _ if x < 0.0 => 0,
            _ if x < 0.0 && y < 0.0 => 1,
            _ => 2,
        };
        pts.push((x, y, label));
    }
    let tr_size = (n as f32 * 0.8) as usize;
    let val_size = (n as f32 * 0.1) as usize;
    (
        pts[..tr_size].into(),
        pts[tr_size..tr_size + val_size].to_vec(),
        pts[tr_size + val_size..].to_vec(),
    )
}

#[macro_export]
macro_rules! _gen_data {
    () => {
        crate::utils::gen_data(crate::utils::rng::Rng::new(42), 100)
    };
    ($rng: expr) => {
        crate::utils::gen_data($rng, 100)
    };
    ($rng: expr, $n: expr) => {
        crate::utils::gen_data($rng, $n)
    };
}
