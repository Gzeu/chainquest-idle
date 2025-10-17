pub mod mod_stub {
    use tch::{nn, Device, Tensor};
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    pub fn generate_map(seed: i64) -> Vec<Vec<i32>> {
        // CPU-only stubbed generation with seeded RNG for determinism
        let mut rng = ChaCha8Rng::seed_from_u64(seed as u64);
        let mut grid = vec![vec![0; 16]; 16];
        for x in 0..16 { for y in 0..16 {
            // simple pattern using rng bytes
            let v: u8 = rng.next_u32() as u8;
            grid[x][y] = (v % 4) as i32; // 0..=3 align with TileType variants
        }}
        grid
    }
}
